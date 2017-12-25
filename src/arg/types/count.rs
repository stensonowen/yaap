
//! A flag that has a corresponding count
//! Good: `-v -v -v`, `-v=2`, `--verbose --verbose`, `--verbose=2`
//! Bad: `-v=2 -v`, `--verboseverbose`, `-v 3`, `--vvvverbosevv`

use YaapArg;
use arg::{ArgM, ArgS, ArgType, ArgMatch};
use arg::err::{ArgError, ArgResult};
//use arg::arg_s::ArgMatch;

#[derive(Debug, Default)]
pub struct CountArg {
    //max: Option<u8>,
}

impl ArgM<CountArg> {
    fn matches_shorts<'a>(&self, arg_s: &'a str) -> Option<u8> {
        let short: char = self.short?;
        let mut chars = arg_s.chars();
        if chars.next() != Some('-') {
            return None
        }
        if chars.all(|c| c == short) {
            Some(arg_s.len() as u8 - 1)
        } else {
            None
        }
    }
}

impl ArgType for CountArg {
    type Contents = u8;
    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Self::Contents> {
        let mut count = 0;
        let mut definitions_allowed = true;

        for &mut ArgS { ref text, ref mut used } in args.iter_mut() {
            if *used {
                continue
            }
            match argm.matches(text) {
                ArgMatch::Match => {
                    if definitions_allowed {
                        count += 1;
                    } else {
                        return Err(ArgError::Repetition{long: argm.long});
                    }
                },
                ArgMatch::Contains(s) => {
                    if definitions_allowed && count == 0 {
                        match s.parse() {
                            Ok(n) => count = n,
                            Err(_) => return Err(ArgError::BadType {
                                long: argm.long, 
                                exp_type: u8::type_name(), 
                                attempt: s.to_string(),
                            }),
                        }
                        definitions_allowed = false;
                    } else {
                        return Err(ArgError::Repetition{long: argm.long});
                    }
                },
                ArgMatch::NoMatch => {
                    // what's the best way to do this?
                    // CountArg can match differently than others, e.g. (`-ccc`)
                    // should we just make an exception here, or encode it in types?
                    // ArgM<X>::matches could vary? And CountArg could just be unique
                    // then others should vary? e.g. FlagArg could not look for `=`
                    // Maybe this can be tacked onto ArgTrait as a method?
                    // For now we'll just add an exception here though
                    if let Some(n) = argm.matches_shorts(text) {
                        if definitions_allowed && count == 0 {
                            definitions_allowed = false;
                            count = n;
                        } else {
                            return Err(ArgError::Repetition{long: argm.long});
                        }
                    }
                },
            }
        }

        /*
        // TODO: permit max value? min value?
        // probably not; would require a BadValue err type which feels out of scope
        if let Some(c) = argm.kind.max {
            if c > max {
                return Err(ArgError::
            }
        }
        */

        Ok(count)
    }
}

#[cfg(test)]
mod test {
    use arg::{ArgM, CountArg};
    use arg::err::ArgResult;
    use arg::test::own;

    fn count_helper(s: &'static str) -> ArgResult<u8> {
        let mut argm: ArgM<CountArg> = ArgM::from("count", "").with_short('c');
        let mut args = own(s);
        argm.extract(&mut args)
    }

    #[test]
    fn zero() {
        assert_eq!(Ok(0), count_helper("--nothing --here"));
    }

    #[test]
    fn one() {
        assert_eq!(Ok(1), count_helper("--count"));
    }

    #[test]
    fn multi() {
        assert_eq!(Ok(5), count_helper("-c --count -c -c --count"));
    }

    #[test]
    fn consecutive() {
        assert_eq!(Ok(3), count_helper("-ccc"));
    }

    #[test]
    fn consecutive_long() {
        assert_eq!(Ok(0), count_helper("--countcount"));
    }

    #[test]
    fn equals_short() {
        assert_eq!(Ok(42), count_helper("-c=42"));
    }

    #[test]
    fn equals_long() {
        assert_eq!(Ok(50), count_helper("--count=50"));
    }

    #[test]
    fn mix_consecutive_and_flag() {
        assert!(count_helper("-cc -c").is_err());
    }

    #[test]
    fn mix_flag_and_consecutive() {
        assert!(count_helper("-c -cc").is_err());
    }

    #[test]
    fn mix_count_and_flag() {
        assert!(count_helper("--count=1 -c").is_err());
    }

    #[test]
    fn mix_flag_and_count() {
        assert!(count_helper("-c --count=1").is_err());
    }

    #[test]
    fn mix_consecutive_and_count() {
        assert!(count_helper("-cc --count=1").is_err());
    }

    #[test]
    fn mix_count_and_consecutive() {
        assert!(count_helper("--count=1 -cc").is_err());
    }
}

