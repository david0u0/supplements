mod definition {
    include!(concat!(env!("OUT_DIR"), "/definition.rs"));
}
use definition::supplements_example as root;

struct Dummy;

impl root::FlagThisTest for Dummy { }
impl root::Cmd for Dummy {
    type IFlagThisTest = Dummy;
}

fn main() {
    env_logger::init();

    let args:Vec<_> = std::env::args().collect();
    log::info!("args = {:?}", args);

    let res = <Dummy as root::Cmd>::generate().supplement(args.into_iter(), false);
    println!("{:?}", res);
}
