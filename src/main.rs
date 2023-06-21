mod clipboard;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use clipboard::{ClipContent, Clipboard};
use notify_rust::{Notification, Timeout};
use std::{env, io::ErrorKind};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Retrieves clipboard content
    Get(GetArgs),
    /// Loads content to clipboard
    Load(LoadArgs),
    /// Saves current clipboard
    Save(SaveArgs),
}

#[derive(Args)]
struct GetArgs {
    /// Index of the value to retrieve
    index: u32,
}

#[derive(Args)]
struct LoadArgs {
    /// Index of the value to restore in the clipboard
    index: u32,
}

#[derive(Args)]
struct SaveArgs {
    /// Index for storing the current clipboard
    index: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let temp_filename = env::temp_dir().join("clipman.bin");
    let mut clipboard = match Clipboard::open(&temp_filename) {
        Ok(clipboard) => clipboard,
        Err(err) if err.kind() == ErrorKind::NotFound => Clipboard::new(),
        Err(err) => return Err(err.into()),
    };
    match &cli.command {
        Commands::Get(args) => {
            if let Some(data) = clipboard.get(args.index) {
                match data {
                    ClipContent::Text(text) => println!("{}", text),
                    ClipContent::Image(image) => {
                        println!("[{} x {} image]", image.width, image.height)
                    }
                }
            } else {
                bail!("index not found");
            }
        }
        Commands::Save(args) => {
            let data = clipboard::get_clipboard()?;
            clipboard.insert(args.index, data);
            clipboard.save(&temp_filename)?;
            notify(&format!("Clipboard saved in slot {}", args.index))?;
        }
        Commands::Load(args) => {
            let data = clipboard.get(args.index);
            if let Some(data) = data {
                notify(&format!("Restoring slot {}", args.index))?;
                if let Err(err) = clipboard::set_clipboard(data) {
                    notify("An error ocurred")?;
                    bail!(err);
                }
            } else {
                bail!("index not found");
            }
        }
    }
    Ok(())
}

fn notify(message: &str) -> Result<()> {
    Notification::new()
        .summary("Clipboard manager")
        .body(message)
        .timeout(Timeout::Milliseconds(1000))
        .show()?;
    Ok(())
}
