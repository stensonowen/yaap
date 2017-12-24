
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
            // only check unused args
            if arg_s.used {
                continue
            }
            // check if matches either short or long (but short first)
            let m = arg_s.matches(argm.long, argm.short);
            match m {
                // Arg is present; verify only used once and mark arg used
                ArgMatch::Match => if result {
                    return Err(ArgError::Repetition{long: argm.long})
                } else {
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
        Ok(result)
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
