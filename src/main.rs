use std::io;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};


fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    // Clear the terminal.
    term.clear();

    // Get arguments.
    let args: Vec<String> = env::args().collect();

    // Dump file contents.
    for arg_idx in 1..args.len() {
        let filename: &str = args[arg_idx].as_str();

        if !Path::new(filename).exists() {
            println!("Failed to open: {}", filename);
            return Err(io::Error::from_raw_os_error(1));
        }

        // Open up the file.
        let mut file = fs::File::open(&filename)?;

        // Read file into vector.
        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes);

        // Literally just dump out the file into stdout.
        for byte_idx in 0..bytes.len() {
            print!(" {}", bytes[byte_idx] as char);
        }

        // Draw little box around.
        term.draw(|x| {
            let block = Block::default()
                .title(filename)
                .borders(Borders::ALL);

            x.render_widget(block, x.size());
        });
    }

    Ok(())
}
