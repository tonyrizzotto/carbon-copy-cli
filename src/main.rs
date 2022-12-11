use clap::Parser;
use commands::cli::{Commands, CarbonCopyArgs};
mod commands;
use colored::*;

fn main() {
   let cli:CarbonCopyArgs = commands::cli::CarbonCopyArgs::parse();

   match &cli.command {
      Some(Commands::Copy { interactive }) => {
         if *interactive { commands::interactive::interactive_mode().expect("Couldn't read from stdin"); }
         else { println!("No interactive flag was provided") }
      }
      None => {
         // Default message for no args
         println!("Howdy 👋, no options were provided. Try running {} for more options! 🚀",
            "`carbon help`".bright_green())
      }
   }
}
   