
use std::borrow::Cow::{self, Borrowed, Owned};
use std::cell::Cell;

use colored::Colorize;
use memchr::memchr;
use rustyline::highlight::Highlighter;

const OPENS: &[u8; 3] = b"{[(";
const _CLOSES: &[u8; 3] = b"}])";

/// Highlight matching bracket when typed or cursor moved on.
#[derive(Default)]
pub struct MatchingBracketHighlighter {
    bracket: Cell<Option<(u8, usize)>>, // memorize the character to search...
}

impl MatchingBracketHighlighter {
    /// Constructor
    #[must_use]
    pub fn new() -> Self {
        Self {
            bracket: Cell::new(None),
        }
    }
}

fn find_errors(line: &str) -> Vec<usize> {
    let mut errors = Vec::new();
    let mut x = 0;
    for (pos, c) in line.chars().enumerate() {
        if c == ')' || c == '(' {
            if find_matching_bracket(line, pos, c as u8).is_none() {
                errors.push(x);
            }
            x += 1;
        }
    }
    errors
}

impl Highlighter for MatchingBracketHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if line.len() < 1 || line.starts_with("/") {
            return Borrowed(line);
        }
        let mut ret = String::from(line);
        // highlight matching brace/bracket/parenthesis if it exists
        if let Some((bracket, pos)) = self.bracket.get() {
            if let Some((matching, idx)) = find_matching_bracket(line, pos, bracket) {
                ret.replace_range(idx..=idx, &format!("{}", String::from(matching as char).cyan()));
            }
        }
        let errors = find_errors(line);
        Owned(ret.chars().fold((String::new(), 0), |(s, mut i), c| {
            if s.contains("?") {
                return (format!("{s}{}", String::from(c).red().bold()), i)
            }
            match c {
                ')' | '(' => {
                    if errors.contains(&i) {
                        return (format!("{s}{}", String::from(c).red().bold()), i + 1)
                    }
                    i += 1;
                }
                '?' => if s.contains('?') || !s.contains('=') {
                    return (format!("{s}{}", String::from(c).red().bold()), i)
                } else {
                    return (format!("{s}{}", String::from(c).purple()), i)
                }
                '=' => if s.contains('=') {
                    return (format!("{s}{}", String::from(c).red().bold()), i)
                } else if let Some(last_c) = s.replace(" ", "").chars().last() {
                    if "*+/%^-".contains(last_c) {
                        return (format!("{s}{}", String::from(c).red().bold()), i)
                    }
                    return (format!("{s}{}", String::from(c).purple()), i)
                } else {
                    return (format!("{s}{}", String::from(c).red().bold()), i)
                }
                '*' | '+' | '/' | '%' | '^' => {
                    if let Some(last_c) = s.replace(" ", "").chars().last() {
                        if c == '*' && last_c == '*' {
                            match s.chars().nth(s.len() - 2) {
                                Some(ll) => {
                                    if "*+/%^-".contains(ll) {
                                        return (format!("{s}{}", String::from(c).red().bold()), i)
                                    } else {
                                        return (format!("{s}{c}"), i)
                                    }
                                }
                                _ => return (format!("{s}{}", String::from(c).red().bold()), i)
                            }
                        }
                        if "*+/%^-".contains(last_c) {
                            return (format!("{s}{}", String::from(c).red().bold()), i)
                        }
                    } else {
                        return (format!("{s}{}", String::from(c).red().bold()), i)
                    }
                } 
                _ => {}
            }
            (format!("{s}{c}"), i)
        }).0)
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        true
    }
}

fn find_matching_bracket(line: &str, pos: usize, bracket: u8) -> Option<(u8, usize)> {
    let matching = matching_bracket(bracket);
    let mut idx;
    let mut unmatched = 1;
    if is_open_bracket(bracket) {
        // forward search
        idx = pos + 1;
        let bytes = &line.as_bytes()[idx..];
        for b in bytes {
            if *b == b'=' {
                return None
            }
            if *b == matching {
                unmatched -= 1;
                if unmatched == 0 {
                    debug_assert_eq!(matching, line.as_bytes()[idx]);
                    return Some((matching, idx));
                }
            } else if *b == bracket {
                unmatched += 1;
            }
            idx += 1;
        }
        debug_assert_eq!(idx, line.len());
    } else {
        // backward search
        idx = pos;
        let bytes = &line.as_bytes()[..idx];
        for b in bytes.iter().rev() {
            if *b == b'=' {
                return None
            }
            if *b == matching {
                unmatched -= 1;
                if unmatched == 0 {
                    return Some((matching, idx - 1));
                }
            } else if *b == bracket {
                unmatched += 1;
            }
            idx -= 1;
        }
    }
    None
}

const fn matching_bracket(bracket: u8) -> u8 {
    match bracket {
        b'{' => b'}',
        b'}' => b'{',
        b'[' => b']',
        b']' => b'[',
        b'(' => b')',
        b')' => b'(',
        b => b,
    }
}
fn is_open_bracket(bracket: u8) -> bool {
    memchr(bracket, OPENS).is_some()
}