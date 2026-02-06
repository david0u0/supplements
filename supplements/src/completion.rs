use std::fs;
use std::io::Result as IoResult;
use std::io::Write;
use std::path::{MAIN_SEPARATOR_STR, Path};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Completion {
    pub value: String,
    pub description: String,
}
impl Completion {
    pub fn new(value: &str, description: &str) -> Self {
        Completion {
            value: value.to_owned(),
            description: description.to_owned(),
        }
    }
    pub fn files(arg: &str) -> Vec<Self> {
        let path = Path::new(arg);
        let (arg_dir, dir) = match arg {
            "" => (Path::new(""), Path::new("./")),
            "/" => (Path::new("/"), Path::new("/")),
            _ => {
                let arg_dir = if arg.ends_with(MAIN_SEPARATOR_STR) {
                    // path like xyz/ will have `parent() == Some("")`, but we want Some("xyz")
                    path
                } else {
                    path.parent().unwrap()
                };

                let dir = if arg_dir == Path::new("") {
                    Path::new("./")
                } else {
                    arg_dir
                };
                (arg_dir, dir)
            }
        };
        log::debug!("arg_dir = {:?}, dir = {:?}", arg_dir, dir);
        let paths = match fs::read_dir(dir) {
            Ok(paths) => paths,
            Err(err) => {
                log::warn!("error reading current directory: {:?}", err);
                return vec![];
            }
        };

        paths
            .filter_map(|p| {
                let p = match p {
                    Ok(p) => p.path(),
                    Err(err) => {
                        log::warn!("error reading current directory: {:?}", err);
                        return None;
                    }
                };
                let Some(file_name) = p.file_name() else {
                    return None;
                };
                let file_name = arg_dir.join(file_name);
                let trailing = if file_name.is_dir() { "/" } else { "" };
                let file_name = file_name.to_string_lossy();
                if file_name.starts_with(arg) {
                    let file_name = format!("{}{}", file_name, trailing);
                    Some(Completion::new(&file_name, ""))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
pub enum Shell {
    Zsh,
    Fish,
    Bash,
}
impl std::str::FromStr for Shell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = match s {
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "bash" => Shell::Bash,
            _ => return Err(format!("Unknown shell {}", s)),
        };

        Ok(ret)
    }
}

pub struct CompletionGroup {
    arg: String,
    comps: Vec<Completion>,
}

impl CompletionGroup {
    pub fn new(comps: Vec<Completion>, arg: String) -> Self {
        CompletionGroup { arg, comps }
    }
    pub fn into_inner(self) -> (Vec<Completion>, String) {
        (self.comps, self.arg)
    }

    pub fn print(&self, shell: Shell, w: &mut impl Write) -> IoResult<()> {
        for comp in self.comps.iter() {
            if !comp.value.starts_with(&self.arg) {
                continue;
            }

            match shell {
                Shell::Fish => writeln!(w, "{}\t{}", comp.value, comp.description)?,
                Shell::Zsh => writeln!(w, "{}\t{}", comp.value, comp.description)?,
                Shell::Bash => writeln!(w, "{}", comp.value)?, // Bash doesn't allow description
            }
        }
        Ok(())
    }
}
