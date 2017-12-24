
use YaapArg;
use arg::{ArgS, ArgM, ArgType};
use arg::err::{ArgError, ArgResult};
use arg::arg_s::ArgMatch;

#[derive(Debug)]
pub struct ValArg<T: YaapArg> {
    default: Option<T>,
}

impl<T: YaapArg> Default for ValArg<T> {
    fn default() -> Self {
        ValArg {
            default: None,
        }
    }
}

impl<T: YaapArg> ArgType for ValArg<T> {
    type Contents = T;

    fn extract(argm: &ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<T> {
        let mut result: Option<T> = None;
        for arg_s in args.iter_mut() {
            let mut used = arg_s.used;
            if used == false {
            }
            arg_s.used = used;
        }
        /*
        let mut result = false;
        for arg_s in args.iter_mut() {
                // check if matches either short or long (but short first)
                let m = arg_s.matches_short_opt(argm.short)
                    .or_else(|| arg_s.matches_long(argm.long)); 
                match m {
                    // Arg is present; verify only used once and mark arg used
                    ArgMatch::Match => if result {
                        return Err(ArgError::Repetition{long: argm.long})
                    } else {
                        used = true;
                        result = true;
                    },
                    // ArgS contains a value (flags can't have contained values)
                    ArgMatch::Contains(s) => {
                        // TODO: allow value? `true` or `false`?
                        return Err(ArgError::UnexpectedValue {
                            long: argm.long, attempt: s.to_string(),
                        })
                    },
                    // ArgS is irrelevant
                    ArgMatch::NoMatch => {},
                }
            }
            arg_s.used = used;
        }
        Ok(result)
        */
        unimplemented!()
    }
}


/*
#[cfg(test)]
    mod test {
        use arg::{ArgM, FlagArg};
        use arg::err::ArgResult;
        use arg::test::own;

        fn flag_helper(s: &'static str) -> ArgResult<bool> {
            let argm: ArgM<FlagArg> = ArgM::from("flag", "").with_short('f');
            let mut args = own(s);
            argm.extract(&mut args)
        }

        #[test]
        fn yes() {
            assert_eq!(Ok(true), flag_helper("--flag"));
        }

        #[test]
        fn no() {
            assert_eq!(Ok(false), flag_helper("--nothing --to -c --here"));
        }

        #[test]
        fn short() {
            assert_eq!(Ok(true), flag_helper("--something --to -f --here"));
        }

    }
    */
