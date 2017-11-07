use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgResult, ArgMatch, ArgError};

#[derive(Debug, Default)]
pub struct FlagArg;

impl ArgTrait for FlagArg {
    type MatchType = bool;

    fn extract_match(_: &Arg<Self>, _: &str) -> ArgResult<Self::MatchType> {
        // is this indicative of poor design?
        unreachable!()
    }
}

impl Arg<FlagArg> {
    pub(super) fn help() -> Self {
        Self::from("help", "Print this message")
            .with_short('h')
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
        for (s,free) in Self::args(&self.argv).zip(self.free.iter_mut()) {
            match arg.does_match(s) {
                ArgMatch::NoMatch => {},
                ArgMatch::Match => {
                    *free = false;
                    *result = true;
                },
                ArgMatch::Contains(_) => {
                    *free = false; // ill defined?
                    self.errs.push(ArgError::UnexpectedValue {
                        long: arg.long, attempt: s.to_owned()
                    });
                }
            }
        }
        /*
        let mut errs = vec![];
        // *result = self.argv.iter().map(|s| match arg.does_match(s) {
        *result = Self::args(&self.argv).map(|s| match arg.does_match(s) {
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
        */
        self.args.push(arg.strip_type());
        self
    }
}


