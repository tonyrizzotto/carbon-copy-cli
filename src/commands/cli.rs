use clap::{Parser, Subcommand};

/// Copy and implement templates like a boss
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CarbonCopyArgs {
   #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
   /// Copy a template
   Copy {
      /// Directory of the file you want to 
      #[arg(short, long)]
      folder: Option<String>,
   }
}
