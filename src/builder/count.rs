use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgMatch2, ArgError};

#[derive(Debug, Default)]
pub struct CountArg;

impl ArgTrait for CountArg {
    type MatchType = Result<usize, ArgError>;

    fn matches(arg: &Arg<Self>, s: &str) -> Result<usize, ArgError> {
        match arg.short_matches_count(s) {
            Ok(0) => arg.long_matches_count(s),
            sm => sm
        }
    }
}


impl Yaap<YaapOpts> {
    pub fn count(self, result: &mut usize, arg: Arg<CountArg>) -> Yaap<YaapArgs> {
        let new: Yaap<YaapArgs> = self.into();
        new.count(result, arg)
    }
}

impl Yaap<YaapArgs> {
    pub fn count(mut self, result: &mut usize, arg: Arg<CountArg>) -> Self {
        let mut count = 0;
        for s in &self.argv {
            match arg.matches(s) {
                Ok(n) => count += n,
                Err(e) => self.errs.push(e),
            }
        }
        *result = count;
        self
    }
}

impl Arg<CountArg> {
    fn short_matches_count(&self, s: &str) -> Result<usize, ArgError> { 
        if let Some(c) = self.short {
            println!("uhhh `{}`", s);
            let mut chars = s.chars();
            if chars.nth(0) == Some('-') && chars.all(|i| i==c) {
                // `-vvvv`
                Ok(s.len()-1)
            } else if s.starts_with(&['-',c,'='][..]) {
                // `-v=8`
                match s[3..].parse() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(ArgError::BadType)
                }
            } else {
                // no match
                Ok(0)
            }
        } else {
            // no match
            Ok(0)
        }
    }

    fn long_matches_count(&self, s: &str) -> Result<usize, ArgError> { 
        if s.starts_with("--") {
            // `--longlonglong` (if s is just arg.long repeated n>0 times)
            let occurrences = s.matches(self.long).count();
            if occurrences * self.long.len() + 2 == s.len() {
                Ok(occurrences)
            } else {
                Ok(0)
            }
        } else if let ArgMatch2::AtOffset(i) = self.long_matches(s) {
            // `--long=8`
            match s[i..].parse() {
                Ok(n) => Ok(n),
                Err(_) => Err(ArgError::BadType)
            }
        } else {
            // no match
            Ok(0)
        }
    }
}
