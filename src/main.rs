// use std::path::PathBuf;
// use colored::*;
use clap::{Parser, Subcommand};

/// Copy and implement templates like a boss
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CarbonCopyArgs {
   #[command(subcommand)]
    command: Option<Commands>,
   // /// Number of times to greet
   // #[arg(short, long, default_value_t = 1)]
   // count: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
   /// implements the copy feature
   Copy {
      /// Directory of the file you want to copy
      #[arg(short, long)]
      file: Option<String>,
   },

   /// Traverse a folder path to a specific file
   Find {
      /// Directory of the file you want to copy
      #[arg(short, long)]
      file: Option<String>,
   },
}


fn main() {
   let args = CarbonCopyArgs::parse();

   // Default message for no args
   println!("Howdy 👋, I'm smart, but not that smart. Give me something to do, friend!")
}