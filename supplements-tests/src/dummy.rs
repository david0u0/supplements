use supplements::completion::CompletionGroup;

mod def {
    include!(concat!(env!("OUT_DIR"), "/definition.rs"));
}
use def::*;
mod dummy_impl {
    include!(concat!(env!("OUT_DIR"), "/dummy_impl.rs"));
}

pub fn run(cmd: &str) -> CompletionGroup {
    let cmd = cmd.split(" ").map(|s| s.to_string());
    def::CMD.supplement(cmd).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::map_comps;

    #[test]
    fn test_simple() {
        let comps = run("git -");
        assert_eq!(vec!["--git-dir"], map_comps(&comps));

        let comps = run("git g");
        assert_eq!(vec!["checkout", "log"], map_comps(&comps));

        let comps = run("git log --");
        assert_eq!(vec!["--git-dir", "--graph", "--pretty"], map_comps(&comps));
    }
}
