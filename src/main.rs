use clap::Parser;
use commands::cli::{Commands, CarbonCopyArgs};

mod commands;
// use std::path::PathBuf;
use colored::*;
fn main() {
   let cli:CarbonCopyArgs = commands::cli::CarbonCopyArgs::parse();

   match &cli.command {
      Some(Commands::Copy { folder }) => {
          println!("You are in the folder path: {:?}", folder);
      }
      None => {
         // Default message for no args
         println!("Howdy 👋, I'm smart, but not {} smart. Try running {} for more options!",
            "that".blue(),
            "`carbon help`".bright_green())
      }
   }
}
   