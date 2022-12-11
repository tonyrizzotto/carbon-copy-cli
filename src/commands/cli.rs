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
   /// Copy a template by providing input > output locations
   /// 
   /// carbon copy ./folder/file.js ~./development/templates <=== basic usage
   /// 
   /// carbon copy -i <=== open interactive mode
   Copy{
      /// Toggles interactive mode
      #[arg(short, long)]
      interactive: bool,
   }
}
