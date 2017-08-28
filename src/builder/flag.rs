use super::Arg;
use super::super::{ArgTrait, ArgMatch2, ArgError};

#[derive(Debug, Default)]
pub struct FlagArg;

impl FlagArg {
    fn valid_match(am: ArgMatch2) -> Result<bool, ArgError> {
        match am {
            ArgMatch2::NextArg => Ok(true),
            ArgMatch2::NoMatch => Ok(false),
            ArgMatch2::AtOffset(_) => Err(ArgError::UnexpectedValue),
        }
    }
}

impl ArgTrait for FlagArg {
    type MatchType = Result<bool, ArgError>;
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }

    fn short_matches(arg: &Arg<Self>, s: &str) -> Result<bool, ArgError> {
        FlagArg::valid_match(arg.short_matches(s))
    }
    fn long_matches(arg: &Arg<Self>, s: &str) -> Result<bool, ArgError> {
        FlagArg::valid_match(arg.long_matches(s))
    }

    fn matches(arg: &Arg<Self>, s: &str) -> Result<bool, ArgError> {
        let short_matches = FlagArg::short_matches(arg, s);
        if short_matches == Ok(false) {
            FlagArg::long_matches(arg, s)
        } else {
            short_matches
        }
    }
}

impl Arg<FlagArg> {
}

