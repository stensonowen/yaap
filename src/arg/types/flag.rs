
use yaap::{Yaap, YaapArgs};
use arg::{ArgS, ArgM, ArgType};
use arg::err::{ArgError, ArgResult};
use arg::arg_s::ArgMatch;

#[derive(Debug, Default)]
pub struct FlagArg;

impl ArgType for FlagArg {
    type Contents = bool;

    fn extract(argm: &ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<bool> {
        let mut result = false;
        // check each arg for relevance
        for arg_s in args.iter_mut() {
            // need to keep track of whether arg should be marked used for borrowck
            //let mut used = false;
            // only check unused args
            //if let Some(ref mut arg_s) = *arg {
            let mut used = arg_s.used;
            if used == false {
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
    }
}

impl Yaap<YaapArgs> {
    pub fn get_flag(mut self, result: &mut bool, argm: ArgM<FlagArg>) -> Self {
        self
    }
}


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
