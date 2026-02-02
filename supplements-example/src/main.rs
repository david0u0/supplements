use clap::{CommandFactory, Parser};
use supplements::{Completion, History};
use supplements_example::args::Root;

mod def {
    include!(concat!(env!("OUT_DIR"), "/definition.rs"));
}

use def::Supplements;

impl def::FlagYetAnotherTest for Supplements {}
impl def::sub2::ArgSubTest for Supplements {
    fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
        vec![
            Completion::new("arg-value-1", ""),
            Completion::new("arg-value-2", ""),
        ]
    }
}

fn main() {
    env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    log::info!("args = {:?}", args);

    if args.len() == 2 && args[1] == "generate" {
        supplements::generate(&mut Root::command(), &mut std::io::stdout()).unwrap();
        return;
    }

    if args.get(1).map(|s| s.as_str()) == Some("parse") {
        let res = Root::try_parse_from(args[1..].iter());
        match res {
            Ok(res) => println!("{:?}", res),
            Err(err) => println!("{err}"),
        }
        return;
    }

    let res = def::get_cmd().supplement(args.into_iter(), false);
    println!("{:?}", res);
}
