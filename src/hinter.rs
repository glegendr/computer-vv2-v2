

use std::borrow::Cow::{self, Borrowed, Owned};

use colored::Colorize;
use rustyline::highlight::{Highlighter};
use rustyline_derive::{Completer, Helper, Validator};
use std::collections::HashSet;
use rustyline::hint::{Hint, Hinter};
use rustyline::Context;
use crate::highlighter::MatchingBracketHighlighter;


#[derive(Completer, Helper, Validator)]
pub struct ComputorHinter {
    // It's simple example of rustyline, for more efficient, please use ** radix trie **
    pub hints: HashSet<CommandHint>,
    pub highlighter: MatchingBracketHighlighter,
    pub colored_prompt: String,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Highlighter for ComputorHinter {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("{}", hint.italic().cyan().bold()))
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}


impl Hinter for ComputorHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                // expect hint after word complete, like redis cli, add condition:
                // line.ends_with(" ")
                if hint.display.starts_with(line) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}

pub fn diy_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    set.insert(CommandHint::new("/help", "/help"));
    set.insert(CommandHint::new("/tree", "/tree"));
    set.insert(CommandHint::new("/history", "/history"));
    set.insert(CommandHint::new("/list", "/list"));
    set.insert(CommandHint::new("/clear", "/clear"));
    set.insert(CommandHint::new("/chart", "/chart"));
    set.insert(CommandHint::new("/quadratic", "/quadratic"));
    set
}
