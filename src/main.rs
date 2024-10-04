use crossterm::{
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    // Clear the terminal
    stdout.execute(terminal::Clear(ClearType::All))?;

    // Get the terminal size
    let (width, height) = terminal::size()?;

    // Use the terminal size to format your UI
    println!("Terminal size: {}x{}", width, height);
    println!("This is where your todo items will be displayed.");

    // Example: Create a simple dynamic layout
    let offset = 3;
    for i in offset..height {
        if i < height - 1 {
            let formatted_left = format!("[{}] Todo item", i);
            let chars_count = formatted_left.chars().count() as u16;
            let spaces_count = (width as u16) - chars_count - 3;
            let spaces = " ".repeat(spaces_count.max(0) as usize);
            let formatted = format!("{}{}[{}]", formatted_left, spaces, "x");
            println!("{}", formatted);
        } else {
            println!("Press any key to continue...");
        }
    }

    stdout.flush()?;

    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Clear the terminal again
    stdout.execute(terminal::Clear(ClearType::All))?;

    Ok(())
}
