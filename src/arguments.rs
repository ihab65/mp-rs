use clap:: {
    // Args,
    Parser,
    // Subcommand
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// 1st argument
    pub first_arg: String,
    /// 2nd argument
    pub second_arg: String,
    /// 3rd argument
    pub third_arg: String
}
