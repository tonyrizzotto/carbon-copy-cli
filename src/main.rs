// use std::path::PathBuf;
// use colored::*;
use clap::{Parser, Subcommand};

/// Copy and implement templates like a boss
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CarbonCopyArgs {
   #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
   /// Copy a template
   Copy {
      /// Directory of the file you want to 
      #[arg(short, long)]
      folder: Option<String>,
   }
}


fn main() {
   // let args = CarbonCopyArgs::parse();

   // Default message for no args
   println!("Howdy 👋, I'm smart, but not that smart. Give me something to do, friend!")
}