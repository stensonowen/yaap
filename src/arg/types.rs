
use super::{Arg, /*ArgMatch,*/ ArgError};
use super::ArgResult as Result;

// ugh... I think using ArgMatch requires adding `<'a>` to every ArgTrait
// and then to every ArgTrait impl
// and then to every Arg
// and then to every Arg impl
// ... should probably do it though
#[derive(Debug, PartialEq)]
pub enum ArgMatch2 {
    NoMatch,
    NextArg,
    AtOffset(usize),
}

use std::fmt::Debug;
pub trait ArgTrait : Debug {
    type MatchType;
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<Self::MatchType>
        where Self: Sized;
    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<Self::MatchType>
        where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> Result<Self::MatchType> 
        where Self: Sized;
}

// do I have no choice but to expose these to the user?
// they will have to see the type Arg<_>, but they shouldn't need to know it
// ehh I guess they can just not import it
#[derive(Debug)]
pub struct FlagArg;
#[derive(Debug)]
pub struct CountArg;
#[derive(Debug)]
pub struct ValArg;
#[derive(Debug)]
pub struct ListArg { pub(super) len: Option<usize> }

impl FlagArg {
    // The `contained` option is an invalid result
    fn valid_match(am: ArgMatch2) -> Result<bool> {
        match am {
            ArgMatch2::NextArg => Ok(true),
            ArgMatch2::NoMatch => Ok(false),
            ArgMatch2::AtOffset(_) => Err(ArgError::UnexpectedValue),
        }
    }
}

impl ArgTrait for FlagArg {
    type MatchType = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }

    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<bool> {
        FlagArg::valid_match(arg.short_matches(s))
    }
    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<bool> {
        FlagArg::valid_match(arg.long_matches(s))
    }

    fn matches(arg: &Arg<Self>, s: &str) -> Result<bool> {
        let short_matches = FlagArg::short_matches(arg, s);
        if short_matches == Ok(false) {
            FlagArg::long_matches(arg, s)
        } else {
            short_matches
        }
    }
}

impl ArgTrait for CountArg {
    type MatchType = usize;
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }

    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<usize> { 
        if let Some(c) = arg.short {
            let mut chars = s.chars();
            if chars.nth(0) == Some('-') && chars.all(|i| i ==c) {
                Ok(s.len()-1)
            } else {
                Ok(0)
            }
        } else if let Some(c) = arg.short {
            // TODO: comment this arm?
            if let ArgMatch2::AtOffset(_) = arg.short_matches(s) {
                Err(ArgError::UnexpectedValue)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<usize> { 
        if s.starts_with("--") {
            // if s is only `--` and arg.long repeated n>0 times
            let occurrences = s.matches(arg.long).count();
            if occurrences * arg.long.len() + 2 == s.len() {
                Ok(occurrences)
            } else {
                Ok(0)
            }
        } else if let ArgMatch2::AtOffset(_) = arg.long_matches(s) {
            // TODO: comment this arm ?
            Err(ArgError::UnexpectedValue)
        } else {
            Ok(0)
        }
    }

    fn matches(arg: &Arg<Self>, s: &str) -> Result<usize> {
        // TODO: allow `=` syntax?
        // e.g. `-v=3`?
        let short_matches = Self::short_matches(arg, s);
        if short_matches == Ok(0) {
            Self::long_matches(arg, s)
        } else {
            short_matches
        }
    }
}

impl ArgTrait for ValArg {
    type MatchType = ArgMatch2;
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }

    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> { 
        Ok(arg.short_matches(s))
    }

    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> { 
        Ok(arg.long_matches(s))
    }

    fn matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> {
        //arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
        //unimplemented!()
        let short_matches = Self::short_matches(arg, s);
        if short_matches == Ok(ArgMatch2::NoMatch) {
            Self::long_matches(arg, s)
        } else {
            short_matches
        }
    }
}

impl ArgTrait for ListArg {
    type MatchType = ArgMatch2;
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> { unimplemented!() }
    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> { unimplemented!() }
    fn matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> {
        //arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
        unimplemented!()
    }
}

