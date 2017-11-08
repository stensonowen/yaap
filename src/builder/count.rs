use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgResult, ArgTrait, ArgMatch, ArgError};
use begin::BeginsWith;

#[derive(Debug, Default)]
pub struct CountArg;

impl ArgTrait for CountArg {
    type MatchType = usize;

    fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        // major difference:  fn("-ccc") -> Contains("ccc")
        if let Some(c) = arg.short {
            if s.begins_with_3('-', c, c) {
                return ArgMatch::Contains(&s[1..])
            }
        }
        arg.short_matches(s).or_else(|| arg.long_matches(s))
    }

    fn extract_match(arg: &Arg<Self>, s: &str) -> ArgResult<Self::MatchType> {
        // e.g. `-vvv`
        if let Some(c) = arg.short {
            if s.chars().all(|i| i==c) {
                return Ok(s.len())
            }
        }
        // e.g. `--long=18`
        s.parse().map_err(|_| ArgError::BadType {
            long: arg.long, attempt: s.to_owned()
        })
    }
}


impl Yaap<YaapOpts> {
    pub fn count(self, result: &mut usize, arg: Arg<CountArg>) -> Yaap<YaapArgs> {
        let new: Yaap<YaapArgs> = self.into();
        new.count(result, arg)
    }
}

impl Yaap<YaapArgs> {
    pub fn count(mut self, result: &mut usize, arg_m: Arg<CountArg>) -> Self {
        let mut count = 0;
        for arg_s in Self::args(&mut self.argv) {
        //for (s,free) in Self::args(&self.argv).zip(self.free.iter_mut()) {
            match CountArg::does_match(&arg_m, &arg_s.text) {
                ArgMatch::NoMatch => {},
                ArgMatch::Match => {
                    arg_s.free = false;
                    //*free = false;
                    count += 1
                },
                ArgMatch::Contains(t) => {
                    arg_s.free = false;
                    //*free = false;
                    match arg_m.extract_match(t) {
                        Ok(n) => count += n,
                        Err(e) => self.errs.push(e),
                    }
                }
            }
        }
        *result = count;
        self.args.push(arg_m.strip_type());
        self
    }
}

/*
impl Arg<CountArg> {
    fn short_matches_count_(&self, s: &str) -> Result<usize, ArgError> { 
        if let Some(c) = self.short {
            let mut chars = s.chars();
            if chars.nth(0) == Some('-') && chars.all(|i| i==c) {
                // `-vvvv`
                Ok(s.len()-1)
            //} else if s.starts_with(&['-',c,'='][..]) {
            } else if s.chars().zip(&['-',c,'=']).all(|(a,&b)| a==b) {
                // `-v=8`
                match s[3..].parse() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(ArgError::BadType {
                        long: self.long, attempt: s.to_owned(),
                    })
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
            } else if let ArgMatch2::AtOffset(i) = self.long_matches(s) {
                // `--long=5`
                match s[i..].parse() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(ArgError::BadType {
                        long: self.long, attempt: s.to_owned(),
                    })
                }
            } else {
                Ok(0)
            }
        } else {
            // no match
            Ok(0)
        }
    }
}
*/
