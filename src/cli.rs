use clap:: {
    Parser,
    Subcommand
};

use crate::actions::PathToPlay;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct MPRSArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play what's in the spicified path 
    Play(PathToPlay)
}