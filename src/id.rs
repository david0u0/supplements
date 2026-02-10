#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NoVal(u32, &'static str);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SingleVal(u32, &'static str);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MultiVal(u32, &'static str);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arg {
    Single(SingleVal),
    Multi(MultiVal),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flag {
    No(NoVal),
    Single(SingleVal),
    Multi(MultiVal),
}

impl NoVal {
    pub const fn new(id: u32, ident: &'static str) -> Self {
        NoVal(id, ident)
    }
}
impl SingleVal {
    pub const fn new(id: u32, ident: &'static str) -> Self {
        SingleVal(id, ident)
    }
}
impl MultiVal {
    pub const fn new(id: u32, ident: &'static str) -> Self {
        MultiVal(id, ident)
    }
}

impl Flag {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Flag::No(NoVal(_, name)) => name,
            Flag::Single(SingleVal(_, name)) => name,
            Flag::Multi(MultiVal(_, name)) => name,
        }
    }
}
