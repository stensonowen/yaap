
use super::Arg;
use super::super::{ArgTrait, ArgMatch2};

#[derive(Debug, Default)]
pub struct ValArg;

impl ArgTrait for ValArg {
    // can't return an error
    type MatchType = ArgMatch2;
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }

    fn short_matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 { 
        arg.short_matches(s)
    }

    fn long_matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 { 
        arg.long_matches(s)
    }

    fn matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 {
        //arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
        //unimplemented!()
        let short_matches = Self::short_matches(arg, s);
        if short_matches == ArgMatch2::NoMatch {
            Self::long_matches(arg, s)
        } else {
            short_matches
        }
    }
}
