pub mod args;
pub mod dummy;
use supplements::completion::CompletionGroup;

pub fn map_comps(comps: &CompletionGroup) -> Vec<&str> {
    let mut ret: Vec<_> = comps.inner().0.iter().map(|c| c.value.as_str()).collect();
    ret.sort();
    ret
}
