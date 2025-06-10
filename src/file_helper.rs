use rustyline::{
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};
use std::borrow::Cow;

pub struct FileCompleterHelper {
    files: Vec<String>,
}

impl FileCompleterHelper {
    pub fn new(files: Vec<String>) -> Self {
        FileCompleterHelper { files }
    }
}

impl Completer for FileCompleterHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let word_start = line[..pos]
            .rfind(|c: char| c.is_whitespace() || c == '/')
            .map_or(0, |i| i + 1);
        let word = &line[word_start..pos];
        let matches = self
            .files
            .iter()
            .filter(|f| f.starts_with(word))
            .map(|f| Pair {
                display: f.clone(),
                replacement: f.clone(),
            })
            .collect();
        Ok((word_start, matches))
    }
}

impl Highlighter for FileCompleterHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }
}

impl Hinter for FileCompleterHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Validator for FileCompleterHelper {}
impl rustyline::Helper for FileCompleterHelper {}
