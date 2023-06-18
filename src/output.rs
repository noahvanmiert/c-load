use crossterm::{execute, Result};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};


pub struct Output;

impl Output {

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
