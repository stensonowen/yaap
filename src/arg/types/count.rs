
//! A flag that has a corresponding count
//! Good: `-v -v -v`, `-v=2`, `--verbose --verbose`, `--verbose=2`
//! Bad: `-v=2 -v`, `--verboseverbose`, `-v 3`

use YaapArg;
use arg::{ArgM, ArgS, ArgType};
use arg::err::{ArgError, ArgResult};
use arg::arg_s::ArgMatch;

#[derive(Debug, Default)]
pub struct CountArg {
    //max: Option<u8>,
}

impl ArgType for CountArg {
    type Contents = u8;
    fn extract(argm: &ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Self::Contents> {
        let mut count = 0;
        let mut definitions_allowed = true;

        for arg_s in args.iter_mut() {
            if arg_s.used {
                continue
            }
            match arg_s.matches(argm.long, argm.short) {
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
                ArgMatch::NoMatch => {},
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
        let argm: ArgM<CountArg> = ArgM::from("count", "").with_short('c');
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
    fn equals_short() {
        assert_eq!(Ok(42), count_helper("-c=42"));
    }

    #[test]
    fn equals_long() {
        assert_eq!(Ok(50), count_helper("--count=50"));
    }

    #[test]
    fn mix_count_and_flag() {
        assert!(count_helper("--count=1 -c").is_err());
    }

    #[test]
    fn mix_flag_and_count() {
        assert!(count_helper("-c --count=1").is_err());
    }

}

