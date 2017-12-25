
use arg::{ArgS, ArgM, ArgType, ArgMatch};
use arg::err::{ArgError, ArgResult};

#[derive(Debug, Default)]
pub struct FlagArg;

impl ArgType for FlagArg {
    type Contents = bool;

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<bool> {
        let mut result = false;
        // check each arg for relevance
        for &mut ArgS { ref text, ref mut used } in args.iter_mut() {
            if *used {
                continue
            }
            match argm.matches(text) {
                // Arg is present; verify only used once and mark arg used
                ArgMatch::Match => if result {
                    return Err(ArgError::Repetition{long: argm.long})
                } else {
                    *used = true;
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
        let mut argm: ArgM<FlagArg> = ArgM::from("flag", "").with_short('f');
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
