#[path = "src/args.rs"]
mod args;

use args::Root;
use clap::CommandFactory;
use std::path::Path;
use supplements::generate;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let file = Path::new(&out_dir).join("definition.rs");
    let mut f = std::fs::File::create(file).unwrap();
    generate(&mut Root::command(), &mut f).unwrap();
}
