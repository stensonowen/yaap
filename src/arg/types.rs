
use super::{Arg, ArgMatch, ArgError};
use super::ArgResult as Result;

// ugh... I think using ArgMatch requires adding `<'a>` to every ArgTrait
// and then to every ArgTrait impl
// and then to every Arg
// and then to every Arg impl
// ... should probably do it though
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

impl ArgTrait for FlagArg {
    type MatchType = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }
    // The `contained` option is an invalid result
    fn matches(arg: &Arg<Self>, s: &str) -> Result<bool> {
        // is there some pattern that might make this nicer? 
        // maybe From<ArgMatch> for Result<bool> ??
        match arg.short_matches(s) {
            ArgMatch::Match => Ok(true),
            ArgMatch::Contained(_t) => Err(ArgError::UnexpectedValue),
            ArgMatch::NoMatch => match arg.long_matches(s) {
                ArgMatch::Match => Ok(true),
                ArgMatch::Contained(_t) => Err(ArgError::UnexpectedValue),
                ArgMatch::NoMatch => Ok(false)
            }
        }
    }
}

impl ArgTrait for CountArg {
    type MatchType = usize;
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> Result<usize> {
        // there might be a better way to do this that doesn't have `returns`
        // creating separate `short_matches` and `long_matches` functions
        //  with the same signature might be easier
        //  and every arg variation supports both, right?
        //  TODO

        // count many short args smushed together (e.g. `-cccc`)
        if let Some(c) = arg.short {
            let mut chars = s.chars();
            if chars.nth(0) == Some('-') && chars.all(|i| i == c) {
                return Ok(s.len()-1)
            }
        }
        // count many long args smushed together (e.g. `-longlong`)
        if s.starts_with("--") {
            let occurrences = s.matches(arg.long).count();
            if occurrences * arg.long.len() + 2 == s.len() {
                return Ok(occurrences)
            }
        }
        // check for incorrect usage ?
        if let Some(c) = arg.short {
            if let ArgMatch::Contained(_) = arg.short_matches(s) {
                println!("AAAA");
                return Err(ArgError::UnexpectedValue)
            }
        }
        if let ArgMatch::Contained(_) = arg.long_matches(s) {
            println!("BBBA");
            return Err(ArgError::UnexpectedValue)
        }
        Ok(0)
    }
}

impl ArgTrait for ValArg {
    type MatchType = ArgMatch2;
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> {
        //arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
        unimplemented!()
    }
}

impl ArgTrait for ListArg {
    type MatchType = ArgMatch2;
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> Result<ArgMatch2> {
        //arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
        unimplemented!()
    }
}


