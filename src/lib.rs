/// This Library uses rustyline and colored to build some nice library functions
/// for getting user input, based on various result types
use colored::ColoredString;
use rustyline::{
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    history::FileHistory,
    validate::Validator,
    Editor,
};
use std::borrow::Cow;
use std::fs::read_dir;
use std::path::PathBuf;

pub mod promptable_enum;
// Re-Use trait for other crates
pub use crate::promptable_enum::PromptableEnum;
pub mod file_helper;
// these helpers are not directly used
use crate::file_helper::FileCompleterHelper;
use crate::promptable_enum::EnumCompleterHelper;

pub struct AdvInput {
    ed: Editor<ActiveHelper, FileHistory>,
}
use colored::Colorize;

impl AdvInput {
    /// initiate a new AdvInput (creates the underlying editor)
    pub fn new() -> Self {
        let mut editor = Editor::new().expect("Failed to create rustyline editor");

        editor.set_helper(Some(ActiveHelper::None));
        AdvInput { ed: editor }
    }

    fn reset_helper(&mut self) {
        *self.ed.helper_mut().expect("Helper not set on Editor") = ActiveHelper::None;
    }

    /// returns either a valid usize, or None
    pub fn get_index(&mut self, prompt: impl Into<ColoredString>) -> Option<usize> {
        self.get_index_initial(prompt, 0)
    }

    /// returns either a valid usize, or None; uses the given `Initial` value to pre-fill
    pub fn get_index_initial(
        &mut self,
        prompt: impl Into<ColoredString>,
        initial: usize,
    ) -> Option<usize> {
        self.reset_helper();
        let prompt_string = prompt.into().to_string();
        if let Ok(line) = self
            .ed
            .readline_with_initial(prompt_string.as_str(), (&format!("{}", initial), ""))
        {
            match line.trim().parse::<usize>() {
                Ok(idx) => Some(idx),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// returns a valid usize in the given range (inclusive), or None
    pub fn get_index_range(
        &mut self,
        prompt: impl Into<ColoredString>,
        low: usize,
        high: usize,
    ) -> Option<usize> {
        if let Some(idx) = self.get_index(prompt) {
            if idx >= low && idx <= high {
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// returns either a valid float, or None
    pub fn get_f64(&mut self, prompt: impl Into<ColoredString>) -> Option<f64> {
        self.get_f64_initial(prompt, 0.0)
    }

    /// returns either a valid float, or None; uses given `Initial` to pre-fill
    pub fn get_f64_initial(
        &mut self,
        prompt: impl Into<ColoredString>,
        initial: f64,
    ) -> Option<f64> {
        self.reset_helper();
        let prompt_string = prompt.into().to_string();
        if let Ok(line) = self
            .ed
            .readline_with_initial(prompt_string.as_str(), (&format!("{}", initial), ""))
        {
            match line.trim().parse::<f64>() {
                Ok(nmb) => Some(nmb),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// returns either a valid float in between the range (all inclusive), or None
    pub fn get_f64_range(
        &mut self,
        prompt: impl Into<ColoredString>,
        low: f64,
        high: f64,
    ) -> Option<f64> {
        self.reset_helper();
        if let Some(nmb) = self.get_f64(prompt) {
            if nmb >= low && nmb <= high {
                Some(nmb)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// returns either a valid String, or None
    pub fn get_string(&mut self, prompt: impl Into<ColoredString>) -> Option<String> {
        self.get_string_initial(prompt, "")
    }

    /// returns either a valid `String`, or `None`; uses the provided `initial` value to pre-fill
    /// the input
    pub fn get_string_initial(
        &mut self,
        prompt: impl Into<ColoredString>,
        initial: &str,
    ) -> Option<String> {
        self.reset_helper();
        let prompt_string = prompt.into().to_string();
        if let Ok(line) = self
            .ed
            .readline_with_initial(prompt_string.as_str(), (initial, ""))
        {
            Some(line.trim().to_string())
        } else {
            None
        }
    }

    /// Prompts the user to select an enum variant using rustyline tab completion.
    /// The `E` type parameter must implement the `PromptableEnum` trait.
    /// The `prompt` argument can be plain string or ColoredString
    /// if `print_variants` is true, then a line with all variantes will be printed
    /// Returns `Some(E)` if a valid enum variant is parsed, `None` otherweise
    pub fn get_enum_input<E>(
        &mut self,
        prompt: impl Into<ColoredString>,
        print_variants: bool,
    ) -> Option<E>
    where
        E: PromptableEnum,
    {
        self.get_enum_input_initial_default(prompt, None, print_variants, None)
    }

    /// Prompts the user to select an enum variant using rustyline tab completion.
    /// The `E` type parameter must implement the `PromptableEnum` trait.
    /// The `prompt` argument can be plain string or ColoredString
    /// if `print_variants` is true, then a line with all variantes will be printed
    /// Returns `Some(E)` if a valid enum variant is parsed, `default` otherweise
    pub fn get_enum_input_default<E>(
        &mut self,
        prompt: impl Into<ColoredString>,
        print_variants: bool,
        default: Option<E>,
    ) -> Option<E>
    where
        E: PromptableEnum,
    {
        self.get_enum_input_initial_default(prompt, None, print_variants, default)
    }

    /// Prompts the user to select an enum variant using rustyline tab completion.
    /// The `E` type parameter must implement the `PromptableEnum` trait.
    /// The `prompt` argument can be plain string or ColoredString
    /// if `print_variants` is true, then a line with all variantes will be printed
    /// Returns `Some(E)` if a valid enum variant is parsed, `None` otherweise
    /// Uses the given initial value, if `Some<E>` is provided
    pub fn get_enum_input_initial<E>(
        &mut self,
        prompt: impl Into<ColoredString>,
        initial: Option<E>,
        print_variants: bool,
    ) -> Option<E>
    where
        E: PromptableEnum,
    {
        self.get_enum_input_initial_default(prompt, initial, print_variants, None)
    }
    ///
    /// Prompts the user to select an enum variant using rustyline tab completion.
    /// The `E` type parameter must implement the `PromptableEnum` trait.
    /// The `prompt` argument can be plain string or ColoredString
    /// if `print_variants` is true, then a line with all variantes will be printed
    /// Returns `Some(E)` if a valid enum variant is parsed, `Default` otherwise
    /// Uses the given initial value, if `Some<E>` is provided
    pub fn get_enum_input_initial_default<E>(
        &mut self,
        prompt: impl Into<ColoredString>,
        initial: Option<E>,
        print_variants: bool,
        default: Option<E>,
    ) -> Option<E>
    where
        E: PromptableEnum,
    {
        /* define helper for the Enum */
        let variants = E::variants_as_strings();
        if print_variants {
            let mut all_variants = if let Some(d) = &default {
                variants
                    .iter()
                    .map(|v| {
                        if v == &d.display_name() {
                            let mut temp = v.to_owned();
                            temp.push_str("(*)");
                            temp
                        } else {
                            v.clone()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            } else {
                variants.join(", ")
            };
            // Limit to a length of 100
            let mut outputs: Vec<String> = Vec::new();
            loop {
                if all_variants.len() <= 100 {
                    outputs.push(all_variants);
                    break;
                } else {
                    /* find the closest whitespace to 100 */
                    match all_variants[..100].rfind(" ") {
                        Some(idx) => {
                            outputs.push(all_variants[..idx].to_string()); // push that part to the ouptut
                            all_variants = all_variants[(idx + 1)..].to_string();
                            // shift remaining text
                        }
                        None => {
                            /* take everything */
                            outputs.push(all_variants);
                            break;
                        }
                    }
                }
            }
            println!("{}", outputs.join("\n").bright_magenta().italic());
        }
        let helper = EnumCompleterHelper::new(variants);
        *self.ed.helper_mut().expect("Helper not set on Editor") = ActiveHelper::Enum(helper);

        let prompt_str = prompt.into().to_string();
        let init = match initial {
            Some(v) => v.display_name(),
            None => "".to_string(),
        };
        let rl = self
            .ed
            .readline_with_initial(prompt_str.as_str(), (init.as_str(), ""));
        if let Ok(line) = rl {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                return default;
            }
            E::from_input_str(trimmed_line)
        } else {
            default
        }
    }

    /// Prompts the user to select a .json file from the current directory with tab completion
    /// Returns `Ok(PathBuf)` with the selected filename if valid, `Err(PathBuf)` if filename does
    /// not yet exist
    pub fn get_json_file_input(
        &mut self,
        prompt: impl Into<ColoredString>,
        dir: PathBuf,
    ) -> Result<PathBuf, PathBuf> {
        let json_files = read_json_files_in_dir(&dir);
        if json_files.is_empty() {
            eprintln!(
                "{}",
                "No .json files found, just enter name for a new one".yellow()
            );
        }
        let helper_instance = FileCompleterHelper::new(json_files.clone());
        if let Some(helper_ref) = self.ed.helper_mut() {
            *helper_ref = ActiveHelper::File(helper_instance);
        } else {
            eprintln!(
                "{}",
                "Internal Error, Tab-completion not available for filenames".yellow()
            );
        }
        let cps = prompt.into().to_string();
        if !json_files.is_empty() {
            println!("available files: {}", json_files.join(", ").cyan());
        }
        let rl = self.ed.readline(cps.as_str());
        match rl {
            Ok(line) => {
                let tl = line.trim();
                if tl.is_empty() {
                    return Err(get_default_file(dir));
                }
                let file_path = PathBuf::from(tl);
                if file_path.exists() {
                    return Ok(file_path);
                }
                return Err(file_path);
            }
            _ => {
                return Err(get_default_file(dir));
            }
        }
    }
}

fn get_default_file(dir: PathBuf) -> PathBuf {
    eprintln!(
        "{}",
        "Problem with given filename. John Doe will be  used.".yellow()
    );
    let mut file_path = dir.clone();
    file_path.push("JohnDoe.json");
    file_path
}

pub fn read_json_files_in_dir(dir: &PathBuf) -> Vec<String> {
    let mut json_files = Vec::new();
    match read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if extension == "json" {
                                if let Some(file_name) = path.file_name() {
                                    if let Some(s) = file_name.to_str() {
                                        json_files.push(s.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}: Failed to read directory: {}", "Error".red(), e);
        }
    }
    json_files
}

/* concrete helper type for Dynamic Behaviour */
pub enum ActiveHelper {
    None,
    Enum(EnumCompleterHelper),
    File(FileCompleterHelper),
}

/* blanket implementation for rustyline's Helper  */
impl rustyline::Helper for ActiveHelper {}

impl Completer for ActiveHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        match self {
            ActiveHelper::None => Ok((pos, vec![])), // no completions
            ActiveHelper::Enum(helper) => helper.complete(line, pos, ctx),
            ActiveHelper::File(helper) => helper.complete(line, pos, ctx),
        }
    }
}
impl Highlighter for ActiveHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        match self {
            ActiveHelper::None => Cow::Borrowed(line),
            ActiveHelper::Enum(helper) => helper.highlight(line, pos),
            ActiveHelper::File(helper) => helper.highlight(line, pos),
        }
    }
}
impl Hinter for ActiveHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        match self {
            ActiveHelper::None => None,
            ActiveHelper::Enum(helper) => helper.hint(line, pos, ctx),
            ActiveHelper::File(helper) => helper.hint(line, pos, ctx),
        }
    }
}
impl Validator for ActiveHelper {
    fn validate(
        &self,
        ctx: &mut rustyline::validate::ValidationContext,
    ) -> rustyline::Result<rustyline::validate::ValidationResult> {
        match self {
            ActiveHelper::None => Ok(rustyline::validate::ValidationResult::Valid(None)),
            ActiveHelper::Enum(helper) => helper.validate(ctx),
            ActiveHelper::File(helper) => helper.validate(ctx),
        }
    }
}
