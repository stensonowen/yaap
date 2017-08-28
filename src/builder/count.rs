use super::Arg;
use super::super::{ArgTrait, ArgMatch2, ArgError};

#[derive(Debug, Default)]
pub struct CountArg;

impl ArgTrait for CountArg {
    type MatchType = Result<usize, ArgError>;
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }

    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<usize, ArgError> { 
        // TODO -c=8
        if let Some(c) = arg.short {
            let mut chars = s.chars();
            if chars.nth(0) == Some('-') && chars.all(|i| i ==c) {
                Ok(s.len()-1)
            } else {
                Ok(0)
            }
        } else if let Some(_) = arg.short {
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

    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<usize, ArgError> { 
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

    fn matches(arg: &Arg<Self>, s: &str) -> Result<usize, ArgError> {
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

impl Arg<CountArg> {

}
