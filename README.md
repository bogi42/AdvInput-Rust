# AdvInput-Rust

This little helper library uses rustyline and colorized to make life easier for you when
delevoping applications depending on user input. The biggest thing is the fact that you
can get a valid enum value, which is something you can use for creating menus, like this:

```rust
use advanced_inputs::{AdvInput, PromptableEnum};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum MainMenu {
    LoadFromFile,
    CreateNew,
    Save,
    Exit,
}
impl PromptableEnum for OpenCharacter {} 

pub fn main() {
    let mut advi = AdvInput::new(); // initialize
    if let Some(choice) = advi.get_enum_input_default::<MainMenu>(
        "> ", 
        true,
        Some(MainMenu::Exit) 
    {
        match choice {
        MainMenu::LoadFromFile => /* code block */, 
        ...
        }
    }
}
```
    
## How to integrate in your project

You can add this dependency to your Cargo.toml:

~~~rust 
[dependencies]
advanced_inputs = { git = "https://github.com/bogi42/AdvInput-Rust.git" }
~~~

See the main.rs file for more demos, or lib.rs for the functions.

