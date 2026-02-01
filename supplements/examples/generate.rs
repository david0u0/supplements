use clap::{CommandFactory, Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ThisTest {
    #[clap(help = "help for p1")]
    P1,
    P2,
}

#[derive(Parser, Debug)]
struct Root {
    #[clap(short = 't', long, help = "help for \"t\"")]
    pub this_test: Option<ThisTest>,
    #[clap(short = 'a', long)]
    pub another_test: bool,
}

fn main() {
    use supplements::generate;

    generate(&mut Root::command(), &mut std::io::stdout()).unwrap();
}
