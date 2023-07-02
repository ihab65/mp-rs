use clap::{
    Args,
    Parser,
    Subcommand, Command
};

#[derive(Parser,Default,Debug)]
struct Arguments {
    Commande: Cmd,
    path: str, // this can be either a path to a dir or to a audio file 
}

