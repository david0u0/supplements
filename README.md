# Supplements
> Shell-agnostic, extensible CLI completion for Rust 💊

*supplements** is a Rust library that generates completion scaffolding as Rust code.

Give it a [`clap`](https://github.com/clap-rs/clap) object, and intead of spitting out shell files that you later have to manually edit, it spits out Rust! `supplement` is:

- **Shell-agnostic**
- **Powerful** - Some features are not widely supported in every shell, and `supplement` comes to rescue
- **Stop modifying generated files** - Instead, **extend** it with rust's trait system
- **It's rust** 🦀

## Example
```rs
// Say you have this clap definition
use clap::{CommandFactory, Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Git {
    #[clap(long)]
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
    Log {
        #[clap(long)]
        graph: bool,
        #[clap(long)]
        pretty: Option<Pretty>,
        commit: Option<String>,
    },
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

// generate it!
supplements::generate(&mut Git::command(), &mut f).unwrap();
```

This will give you something like following:

```rs
pub struct Supplements;
use supplements::*;
pub const ID_FLAG_GIT_DIR: id::Flag = id::Flag::new(line!(), "git_dir");
pub trait FlagGitDir {
    const OBJ: Flag = Flag {
        id: ID_FLAG_GIT_DIR,
        info: info::FlagInfo {
            short: &[],
            long: &["git-dir"],
            description: "",
        },
        comp_options: Some(Self::comp_options),
        once: true,
    };
    fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
        vec![]
    }
}
pub const CMD: Command = Command {
    id: id::Command::new(line!(), "supplements-example"),
    info: info::CommandInfo {
        name: "supplements-example",
        description: "",
    },
    all_flags: &[<Supplements as FlagGitDir>::OBJ],
    args: &[],
    commands: &[checkout::CMD, log::CMD],
};
pub mod checkout {
    #[allow(unused)]
    use super::Supplements;
    use supplements::*;
    pub const ID_ARG_FILE_OR_COMMIT: id::Arg = id::Arg::new(line!(), "file_or_commit");
    pub trait ArgFileOrCommit {
        const OBJ: Arg = Arg {
            id: ID_ARG_FILE_OR_COMMIT,
            comp_options: Self::comp_options,
            max_values: 1,
        };
        fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
            vec![]
        }
    }
    pub const ID_ARG_FILES: id::Arg = id::Arg::new(line!(), "files");
    pub trait ArgFiles {
        const OBJ: Arg = Arg {
            id: ID_ARG_FILES,
            comp_options: Self::comp_options,
            max_values: 18446744073709551615,
        };
        fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
            vec![]
        }
    }
    pub const CMD: Command = Command {
        id: id::Command::new(line!(), "checkout"),
        info: info::CommandInfo {
            name: "checkout",
            description: "",
        },
        all_flags: &[],
        args: &[<Supplements as ArgFileOrCommit>::OBJ, <Supplements as ArgFiles>::OBJ],
        commands: &[],
    };
}
pub mod log {
    #[allow(unused)]
    use super::Supplements;
    use supplements::*;
    pub const FLAG_GRAPH: Flag = Flag {
        id: id::Flag::new(line!(), "graph"),
        info: info::FlagInfo {
            short: &[],
            long: &["graph"],
            description: "",
        },
        comp_options: None,
        once: true,
    };
    pub const FLAG_PRETTY: Flag = Flag {
        id: id::Flag::new(line!(), "pretty"),
        info: info::FlagInfo {
            short: &[],
            long: &["pretty"],
            description: "",
        },
        comp_options: Some(|_, _| vec![Completion::new("oneline", "<sha1> <title line>"), Completion::new("short", "<sha1> / <author> / <title line>)"), Completion::new("full", "<sha1> / <author> / <committer> / <title> / <commit msg>")]),
        once: true,
    };
    pub const ID_ARG_COMMIT: id::Arg = id::Arg::new(line!(), "commit");
    pub trait ArgCommit {
        const OBJ: Arg = Arg {
            id: ID_ARG_COMMIT,
            comp_options: Self::comp_options,
            max_values: 1,
        };
        fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
            vec![]
        }
    }
    pub const CMD: Command = Command {
        id: id::Command::new(line!(), "log"),
        info: info::CommandInfo {
            name: "log",
            description: "",
        },
        all_flags: &[FLAG_GRAPH, FLAG_PRETTY],
        args: &[<Supplements as ArgCommit>::OBJ],
        commands: &[],
    };
}
```

And now you can start to implement it. If you missed something, it's a compile error, so just relex and let Rust get your back 💪

```rs
use std::process::Command;
use supplements::{Completion, History};
use supplements_example::args::Git;

mod def {
    include!(concat!(env!("OUT_DIR"), "/definition.rs"));
}

use def::Supplements;

fn run_git(args: &str) -> String {
    let out = Command::new("git")
        .args(args.split(" "))
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(out).unwrap()
}
impl def::FlagGitDir for Supplements {} // default implementation
impl def::checkout::ArgFileOrCommit for Supplements {
    /// For the first argument, it can either be a git commit or a file
    fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
        let mut ret = vec![];
        for line in run_git("log --oneline -10").lines() {
            let (hash, description) = line.split_once(" ").unwrap();
            ret.push(Completion::new(hash, description));
        }
        for line in run_git("diff-tree --no-commit-id --name-only HEAD -r").lines() {
            ret.push(Completion::new(line, "Modified file"));
        }
        ret
    }
}
impl def::checkout::ArgFiles for Supplements {
    /// For the second and more arguments, it can only be file
    /// Let's also filter out those files we've already seen!
    fn comp_options(history: &History, _arg: &str) -> Vec<Completion> {
        let prev: Vec<_> = history
            .find_all(&[
                def::checkout::ID_ARG_FILES,
                def::checkout::ID_ARG_FILE_OR_COMMIT,
            ])
            .collect();
        run_git("diff-tree --no-commit-id --name-only HEAD -r")
            .lines()
            .filter_map(|line| {
                if prev.iter().any(|p| p.value == line) {
                    None
                } else {
                    Some(Completion::new(line, "Modified file"))
                }
            })
            .collect()
    }
}
impl def::log::ArgCommit for Supplements {
    fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {
        run_git("log --oneline -10")
            .lines()
            .map(|line| {
                let (hash, description) = line.split_once(" ").unwrap();
                Completion::new(hash, description)
            })
            .collect()
    }
}

fn main() {
    // args should be something like ["git-complete", "git", "log", "--pre"]
    // or ["git-complete", "git", "checkout", ""], which means we ant to complete the argument after "checkout"
    let mut args = std::env::args();

    args.next(); // To ignore the name of this executable file
    let res = def::CMD.supplement(args).unwrap();
    for c in res.iter() {
        println!("{}\t{}", c.value, c.description);
    }
}
```

Finally, compile it to binary file named `git-complete`, and create a shell completion file to let shell knows how to use the binary.


```fish
# Put this to /usr/share/fish/completions/git.fish or  ~/.config/fish/completions/git.fish

function __do_completion
    set cmd (commandline -j)
    set cmd_arr (string split ' ' $cmd)
    if [ -z "$cmd_arr[-1]" ]
        # preserve the last white space
        eval "path/to/git-complete $cmd ''"
    else
        eval path/to/git-complete $cmd
    end
end

complete -k -c git -x -a "(__do_completion)"
```
