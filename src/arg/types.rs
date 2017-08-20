
use super::{Arg, ArgMatch};

pub trait ArgTrait {
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> bool where Self: Sized;
}

// do I have no choice but to expose these to the user?
// they will have to see the type Arg<_>, but they shouldn't need to know it
// ehh I guess they can just not import it
pub struct FlagArg;
pub struct CountArg;
pub struct ValArg;
pub struct ListArg { pub(super) len: Option<usize> }

impl ArgTrait for FlagArg {
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for CountArg {
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        // TODO: count multiple args smushed together (e.g. `-xxxx`)
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ValArg {
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ListArg {
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
    }
}


