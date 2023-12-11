mod commands;

use clap::Parser;

use crate::commands::Subcommands;

macro_rules! env_var {
  ($var:expr) => {
    concat!("FFXIVGEN_", $var)
  };
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, env = env_var!("GAME_PATH"))]
  /// Game folder
  ///
  /// The path to the game folder. This is the folder that contains the
  /// `ffxiv_dx11.exe` file and the `sqpack` folder.
  game_path: String,

  #[command(subcommand)]
  command: Option<Subcommands>,
}

fn main() {
  let args = Args::parse();
  let command = args.command.unwrap();

  command.execute();
}
