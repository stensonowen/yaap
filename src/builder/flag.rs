use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgResult, ArgMatch, ArgError};

#[derive(Debug, Default)]
pub struct FlagArg;

impl ArgTrait for FlagArg {
    //type MatchType = Result<bool, ArgError>;
    type MatchType = bool;

    /*
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
    */

    //fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
    //    // TODO gotta check for ArgMatch::Contains in callee0
    //    arg.short_matches_(s).or_else(|| arg.long_matches_(s))
    //}
    fn extract_match(_: &Arg<Self>, _: &str) -> ArgResult<Self::MatchType> {
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
        let mut errs = vec![];
        *result = self.argv.iter().map(|s| match FlagArg::does_match(&arg, s) {
            ArgMatch::Match => true,
            ArgMatch::NoMatch => false,
            ArgMatch::Contains(_) => {
                errs.push(ArgError::UnexpectedValue {
                    long: arg.long, attempt: s.to_owned()
                });
                false
            },
        }).any(|x|x);
        self.errs.append(&mut errs);
        self.args.push(arg.strip_type());
        self
    }
}
