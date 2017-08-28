use super::Arg;
use super::super::{ArgTrait, ArgMatch2};

#[derive(Debug, Default)]
pub struct ListArg { pub(super) len: Option<usize> }

impl ArgTrait for ListArg {
    // can't return an error
    type MatchType = ArgMatch2;
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    fn short_matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 { 
        arg.short_matches(s)
    }
    fn long_matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 { 
        arg.long_matches(s)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 {
        //arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
        //unimplemented!()
        let short_matches = Self::short_matches(arg, s);
        if short_matches == ArgMatch2::NoMatch {
            Self::long_matches(arg, s)
        } else {
            short_matches
        }
    }
}

//impl <T: ArgTrait> Arg<T> where T: ArgTrait {
impl Arg<ListArg> {
    pub fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}


