mod cmd1;

pub use cmd1::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Subcommands {
  Cmd1(Cmd1),
}

impl Subcommands {
  pub fn execute(&self) {
    match self {
      Subcommands::Cmd1(cmd1) => cmd1.execute(),
    }
  }
}

pub trait Command {
  fn execute(&self);
}
