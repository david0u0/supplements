use std::iter::Peekable;

pub struct FlagInfo {
    pub short: Option<char>,
    pub long: &'static str,
    pub description: &'static str,
}
pub struct CommandInfo {
    pub name: &'static str,
    pub description: &'static str,
}
pub enum ArgsOrCommands {
    Args(Vec<Arg>),
    Commands(Vec<Command>),
    None,
}
pub struct SupplementID(u32, &'static str);
pub enum History {
    Flag { id: SupplementID, value: String },
    Command(SupplementID),
    Arg { id: SupplementID, value: String },
}

pub struct Flag {
    id: SupplementID,
    info: FlagInfo,
    comp_options: Option<fn() -> Vec<String>>,
}
pub struct Arg {
    id: SupplementID,
    comp_options: fn() -> Vec<String>,
}
pub struct Command {
    id: SupplementID,
    info: CommandInfo,
    flags: Vec<Flag>,
    args_or_commands: ArgsOrCommands,
}

impl Command {
    pub fn supplement(&self, args: impl Iterator<Item = String>, last_is_empty: bool) {
        let last_arg = if last_is_empty {
            Some(String::new())
        } else {
            None
        };
        let mut args = args.chain(last_arg.into_iter()).peekable();
        if args.peek().is_none() {
            panic!();
        }
        self.supplement_recur(args)
    }

    fn supplement_recur(&self, mut args: Peekable<impl Iterator<Item = String>>) {
        let arg = args.next().unwrap();

        if args.peek().is_none() {
            return self.comp(&arg);
        }
    }

    fn comp(&self, last_arg: &str) {}
}

mod def {
    use super::*;

    pub trait BFlag {
        fn comp_options() -> Vec<String> {
            vec!["option1".to_owned(), "option2".to_owned()]
        }
        fn generate() -> Flag {
            Flag {
                id: SupplementID(line!(), "b"),
                info: FlagInfo {
                    short: Some('b'),
                    long: "long-b",
                    description: "test description for flag B",
                },
                comp_options: Some(Self::comp_options),
            }
        }
    }

    pub trait AArg {
        fn comp_options() -> Vec<String> {
            vec!["option1".to_owned(), "option2".to_owned()]
        }
        fn generate() -> Arg {
            Arg {
                id: SupplementID(line!(), ""),
                comp_options: Self::comp_options,
            }
        }
    }

    pub trait Root {
        type B: BFlag;
        type Sub: SubCommand;

        fn generate() -> Command {
            Command {
                id: SupplementID(line!(), "root"),
                flags: vec![Self::B::generate()],
                info: CommandInfo {
                    name: "root",
                    description: "",
                },
                args_or_commands: ArgsOrCommands::Commands(vec![Self::Sub::generate()]),
            }
        }
    }

    pub trait SubCommand {
        type B: BFlag;
        type A: AArg;
        fn generate() -> Command {
            Command {
                id: SupplementID(line!(), "sub"),
                flags: vec![Self::B::generate()],
                info: CommandInfo {
                    name: "sub",
                    description: "test sub description",
                },
                args_or_commands: ArgsOrCommands::Args(vec![Self::A::generate()]),
            }
        }
    }
}

mod my_impl {
    use super::def;

    pub struct AArg;
    impl def::AArg for AArg {}

    pub struct BFlag;
    impl def::BFlag for BFlag {}

    pub struct Root;
    impl def::Root for Root {
        type B = BFlag;
        type Sub = SubCommand;
    }

    pub struct SubCommand;
    impl def::SubCommand for SubCommand {
        type B = BFlag;
        type A = AArg;
    }
}

fn main() {
    use def::Root;
    my_impl::Root::generate().supplement(std::env::args(), true);
    println!("Hello, world!");
}
