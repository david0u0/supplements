use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Arg {
    #[clap(long, global = true)]
    pub git_dir: Option<std::path::PathBuf>,
    #[clap(subcommand)]
    pub sub: SubCommand,
}
#[derive(Parser, Debug)]
pub enum SubCommand {
    Checkout {
        file_or_commit: Option<String>,
        files: Vec<std::path::PathBuf>,
    },
    #[clap(about = "log")]
    Log {
        #[clap(long)]
        graph: bool,
        #[clap(long)]
        pretty: Option<Pretty>,
        commit: Option<String>,
    },
    #[clap(external_subcommand)]
    Other(Vec<String>),
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Pretty {
    #[clap(help = "<sha1> <title line>")]
    Oneline,
    #[clap(help = "<sha1> / <author> / <title line>)")]
    Short,
    #[clap(help = "<sha1> / <author> / <committer> / <title> / <commit msg>")]
    Full,
}
