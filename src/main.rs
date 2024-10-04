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
            println!("Todo item {}", i + 1);
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
