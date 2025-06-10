use rustyline::{
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};
use std::borrow::Cow;
// Import necessary traits from strum
use strum::IntoEnumIterator;
pub use strum_macros::{Display, EnumIter, EnumString};

/* Helper Function that turns PascalCase into a more readable string */
pub fn add_spaces_before_caps(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        // Add a space if current char is uppercase, not the first char, and the next char is
        // lowercase
        if c.is_uppercase()
            && !result.is_empty()
            && chars.peek().map_or(false, |&next_c| next_c.is_lowercase())
        {
            result.push(' ');
        }
        result.push(c);
    }
    result
}

/// A trait for enums that can be prompted for and parsed from a string
/// Implementations must provide a way to parse a string into an enum variant
/// and a list of all valid string representations for completion
pub trait PromptableEnum:
    Sized + Clone + std::fmt::Debug + IntoEnumIterator + ToString + 'static
{
    // Returns the name of the enum variant formatted
    fn display_name(&self) -> String {
        add_spaces_before_caps(&format!("{:?}", self))
    }

    /// attempts to parse a string slice into an instance of `Self`.
    /// This tries to match against the formatted display name (case-insensitively) and also the
    /// original Debug name (PascalCase) for flexibility
    fn from_input_str(s: &str) -> Option<Self> {
        let s_lower = s.to_lowercase();
        for variant in Self::iter() {
            if variant.display_name().to_lowercase() == s_lower {
                return Some(variant);
            }
            if format!("{:?}", variant).to_lowercase() == s_lower {
                return Some(variant);
            }
        }
        None
    }

    /// returns a `Vec` of `String`s representing all valid variants that can be used for tab
    /// completion
    fn variants_as_strings() -> Vec<String> {
        Self::iter().map(|v| v.display_name()).collect()
    }
}

/// a rustyline helper that provides tab completion for `PromptableEnum` variants.
pub struct EnumCompleterHelper {
    variants: Vec<String>,
}

impl EnumCompleterHelper {
    pub fn new(variants: Vec<String>) -> Self {
        EnumCompleterHelper { variants }
    }
}

/// Implement Completer trait for tab completion
impl Completer for EnumCompleterHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        /* find start of word being typed is ALWAYS the full line */
        let word = &line[..pos];
        /* find variants that start with the current word (case-insensitive) */
        let matches = self
            .variants
            .iter()
            .filter(|v| v.to_lowercase().starts_with(&word.to_lowercase()))
            .map(|v| Pair {
                // `display` is what the user sees in the suggestion list
                display: v.clone(),
                // `replacement` is what is inserted when the user selects the suggestion
                replacement: v.clone(),
            })
            .collect();
        // return start position of the word and the matching candidates
        Ok((0, matches))
    }
}

/* these are required blanket implementations for rustyline's Editor::set_helper()
 * We don't need custom logic for them right now */
impl Highlighter for EnumCompleterHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }
}

impl Hinter for EnumCompleterHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}
impl Validator for EnumCompleterHelper {}
impl rustyline::Helper for EnumCompleterHelper {}
