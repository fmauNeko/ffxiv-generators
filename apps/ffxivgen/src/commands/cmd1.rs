use clap::Parser;

use super::Command;

#[derive(Parser)]
pub struct Cmd1 {}

impl Command for Cmd1 {
  fn execute(&self) {
    println!("Cmd1");
  }
}
