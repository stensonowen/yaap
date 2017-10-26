use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgMatch, ArgMatch2, ArgError};

#[derive(Debug, Default)]
pub struct FlagArg;

impl ArgTrait for FlagArg {
    type MatchType = Result<bool, ArgError>;

    fn matches(arg: &Arg<Self>, s: &str) -> Result<bool, ArgError> {
        // `-f=...` is an error
        let either = match arg.short_matches(s) {
            ArgMatch2::NoMatch => arg.long_matches(s),
            sm => sm
        };
        match either {
            ArgMatch2::NextArg => Ok(true),
            ArgMatch2::NoMatch => Ok(false),
            ArgMatch2::AtOffset(_) => Err(ArgError::UnexpectedValue { 
                long: arg.long, attempt: s.to_owned()
            })
        }
    }

    fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        // should panic/complain on ArgMatch::Contain ?
        if arg.short_matches_(s) == ArgMatch::Match {
            ArgMatch::Match
        } else if arg.long_matches_(s) == ArgMatch::Match {
            ArgMatch::Match
        } else {
            ArgMatch::NoMatch
        }
    }
    fn extract_match(arg: &Arg<Self>, s: &str) -> Self::MatchType {
        // is this indicative of poor design?
        unreachable!()
    }
}

impl Yaap<YaapOpts> {
    pub fn contains(self, result: &mut bool, arg: Arg<FlagArg>) -> Yaap<YaapArgs> {
        let new: Yaap<YaapArgs> = self.into();
        new.contains(result, arg)
    }
}

impl Yaap<YaapArgs> {
    pub fn contains(mut self, result: &mut bool, arg: Arg<FlagArg>) -> Self {
        // TODO verify only one exists ?
        *result = false;
        for s in &self.argv {
            match arg.matches(s) {
                Ok(true) => {
                    *result = true;
                    break
                },
                Ok(false) => continue,
                Err(e) => self.errs.push(e),
            }
        }
        self.args.push(arg.strip_type());
        self
    }
}
