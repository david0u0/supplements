use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ThisTest {
    #[clap(help = "help for p1")]
    P1,
    P2,
}

#[derive(Parser, Debug)]
pub struct Root {
    #[clap(short = 't', long, help = "help for \"t\"")]
    pub this_test: Option<ThisTest>,
    #[clap(short = 'a', long)]
    pub another_test: bool,
    #[clap(short = 'y', long)]
    pub yet_another_test: u32,
}
