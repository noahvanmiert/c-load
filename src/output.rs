/*
    Made by Noah Van Miert
    18/06/2023

    This file is part of the C-load project.
*/

use crossterm::{execute, Result};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};


pub struct Output;

impl Output {

    /// Prints a string in a given color.
    pub fn print(text: &str, color: Color) -> Result<()> {
        execute!(
            std::io::stdout(),
            SetForegroundColor(color),
            Print(text),
            ResetColor
        )?;
    
        Ok(())
    }

}
