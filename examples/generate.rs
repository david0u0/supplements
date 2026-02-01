use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
pub struct Root {
    #[clap(short = 't', long)]
    pub this_test: Option<String>,
    #[clap(short = 'a', long)]
    pub another_test: bool,
}

fn main() {
    use supplements::generate;

    generate(&mut Root::command(), &mut std::io::stdout()).unwrap();
}
