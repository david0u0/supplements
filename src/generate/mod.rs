use clap::Command;
use std::io::Write;

mod utils;

pub fn generate(cmd: &mut Command, w: &mut impl Write) -> std::io::Result<()> {
    cmd.build();
    generate_recur("", cmd, w)
}

struct NameType(&'static str);
impl NameType {
    const FLAG: Self = NameType("Flag");
    const ARG: Self = NameType("Arg");
    const COMMAND: Self = NameType("Command");
}

fn to_pascal_case(s: &str) -> String {
    s.to_string() // TODO
}

fn to_snake_case(s: &str) -> String {
    s.to_string() // TODO
}

fn to_screaming_snake_case(s: &str) -> String {
    s.to_string() // TODO
}

fn gen_rust_name(ty: NameType, name: &str, is_const: bool) -> String {
    let mut ret = ty.0.to_owned();
    if is_const {
        ret = ret.to_uppercase();
    }

    if is_const {
        ret += "_";
        ret += &to_screaming_snake_case(name);
    } else {
        ret += &to_pascal_case(name);
    }

    ret
}

struct JoinQuotes<I>(I, char);
impl<T, I> std::fmt::Display for JoinQuotes<I>
where
    T: std::fmt::Display,
    I: Iterator<Item = T> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for t in self.0.clone() {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, "{}{}{}", self.1, t, self.1)?;
        }
        Ok(())
    }
}

fn generate_things_in_cmd(indent: &str, cmd: &Command, w: &mut impl Write) -> std::io::Result<()> {
    for flag in utils::flags(cmd) {
        let shorts = flag.get_short_and_visible_aliases().unwrap_or_default();
        let longs = flag.get_long_and_visible_aliases().unwrap_or_default();
        let num_args = flag.get_num_args().expect("built");
        let takes_values = num_args.takes_values();
        let once = num_args.max_values() <= 1;

        let name = longs
            .iter()
            .map(|s| s.to_string())
            .chain(shorts.iter().map(|c| c.to_string()))
            .next()
            .expect("flag should have a name");
        let description = utils::escape_help(flag.get_help().unwrap_or_default());

        let rust_name = gen_rust_name(NameType::FLAG, &name, !takes_values);
        let shorts = JoinQuotes(shorts.iter(), '\'');
        let longs = JoinQuotes(longs.iter(), '\"');

        if takes_values {
            writeln!(
                w,
                "{indent}pub trait {rust_name} {{
{indent}    fn comp_options(_history: &History, _arg: &str) -> Vec<Completion> {{
{indent}        vec![] // TODO posible value?
{indent}    }}
{indent}    fn id() -> id::Flag {{
{indent}        id::Flag::new(line!(), \"{name}\")
{indent}    }}
{indent}    fn generate() -> Flag {{
{indent}        Flag {{
{indent}            id: Self::id(),
{indent}            info: FlagInfo {{
{indent}                short: &[{shorts}],
{indent}                long: &[{longs}],
{indent}                description: \"{description}\"
{indent}            }},
{indent}            comp_options: Some(Self::comp_options),
{indent}            once: {once},
{indent}        }}
{indent}    }}
{indent}}}"
            )?;
        } else {
            writeln!(
                w,
                "
{indent}pub const {rust_name}: Flag = Flag {{
{indent}    id: Self::id(),
{indent}    info: FlagInfo {{
{indent}        short: &[{shorts}],
{indent}        long: &[{longs}],
{indent}        description: \"{description}\"
{indent}    }},
{indent}    comp_options: None,
{indent}    once: {once},
{indent}}}"
            )?;
        }
    }
    Ok(())
}
fn generate_recur(indent: &str, cmd: &Command, w: &mut impl Write) -> std::io::Result<()> {
    writeln!(w, "{indent}pub mod {} {{", to_snake_case(cmd.get_name()))?;
    generate_things_in_cmd(&format!("    {indent}"), cmd, w)?;
    writeln!(w, "{indent}}}")
}
