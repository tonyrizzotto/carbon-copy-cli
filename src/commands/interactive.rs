use std::io;
// use std::io::Write;

use colored::Colorize;

pub fn interactive_mode() -> io::Result<()>{
  let mut new_file_name: String = String::new();
  let mut new_file_location: String = String::new();
  let stdin: std::io::Stdin = io::stdin();
  
  println!("Welcome to Carbon Copys interactive mode!");
  println!("Please provide a name for your new template.\nThis can be any file type.");
  
  stdin.read_line(&mut new_file_name).expect("Couldn't read from stdin");

  println!("Gotcha!");

  println!("And where would you like to put your new file?");

  stdin.read_line(&mut new_file_location).expect("Couldn't read from stdin");

  println!("Ok - we'll put your new file:\n{} in {}", new_file_name.green(), new_file_location.purple());
  Ok(())
}