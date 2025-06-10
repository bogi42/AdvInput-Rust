use advanced_inputs::{AdvInput, PromptableEnum};
use colored::Colorize;
use strum_macros::{Display, EnumIter, EnumString};

// This Enum implements PromptableEnum to test ith with get_enum_value()
// The Clone is not needed, but you need EnumIter, EnumString and Display for the Marker
// Trait, as well as Debug. PartialEq and Eq is needed for comparisions as well
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, EnumString, Display)]
enum Direction {
    North,
    South,
    West,
    East,
    Up,
    Down,
}
impl PromptableEnum for Direction {} // Marker Trait

fn main() {
    let mut adv_input = AdvInput::new();
    println!("---- Testing input for ranged index ---");
    match adv_input.get_index_range("Enter a number (1 - 6): ".to_string().green(), 1, 6) {
        Some(number) => println!("You chose : {}", number.to_string().bold().blue()),
        None => println!("No valid number"),
    }
    println!("---- Testing get_enum_input function ---");
    match adv_input.get_enum_input::<Direction>("Enter a direction: ", true) {
        Some(dir) => println!("You chose: {}", dir.to_string().blue().bold()),
        None => println!("No direction selected."),
    }
}
