
// Valid
//      `--list=0,1,2,`     (optional trailing comma)
//    ? `--list 0,1,2`      ""
//    ? `--list 0 1 2`
//      `--list 0 --list 1`
//      `--list=0 --list=1
// Invalid

use YaapArg;
use arg::{ArgS, ArgM, ArgType, ArgMatch};
use arg::err::{ArgError, ArgResult};

use std::marker::PhantomData;

#[derive(Debug)]
pub struct ListArg<T: YaapArg>(PhantomData<T>);

impl<T: YaapArg> Default for ListArg<T> {
    fn default() -> Self {
        ListArg(PhantomData)
    }
}

/*
 * Unfortunately this messes with the ArgM::from() type inference
 * Not sure what to do about that
 *  Different name?
 *  Extra trait?
impl<T: YaapArg> ArgM<ListArg<T>> {
    pub fn is_required(mut self) -> Self {
        self.kind.required = true;
        self
    }
}
*/

#[derive(Debug, PartialEq)]
enum Perhaps {
    // it's an error if the next arg isn't valid
    Yes,
    // it doesn't matter if the next arg looks valid, don't use it
    No,
    // use the next arg iff it's valid
    Maybe,
}

// helper to try to parse a list
// can be either comma-delimited or only one elem
// can contain a trailing comma
// must type-check
fn try_parse<T: YaapArg>(s: &str) -> Option<Vec<T>> { 
    // try to parse first in case T::FromStr works w/ commas already
    if let Ok(x) = s.parse() {
        // `--list=0`
        Some(vec![x])
    } else {
        let substr = if s.ends_with(',') {
            // `--list=0,1,2,`
            &s[..s.len()-1]
        } else {
            // `--list=0,1,2`
            s
        };
        // this could be faster if we `.contains(',')` first
        substr.split(',').map(|i| i.parse()).collect::<Result<_,_>>().ok()
    }
}

impl<T: YaapArg> ArgType for ListArg<T> {
    type Contents = Vec<T>;

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Vec<T>> {
        // keep track of whether the next argument can/should be a match
        // if it *should* be, a type error terminates function early
        // if it *can* be, a type error means it's just not relevant
        let mut expecting = Perhaps::No;
        let mut result: Vec<T> = Vec::new();
        for &mut ArgS { ref text, ref mut used } in args.iter_mut() {
            if *used {
                continue
            } else if expecting == Perhaps::Yes {
                expecting = Perhaps::Maybe;
                *used = true;
                try_parse(text)
                    .map(|mut v| result.append(&mut v))
                    .ok_or_else(|| ArgError::BadType {
                        long: argm.long, attempt: text.to_string(),
                        exp_type: T::type_name()
                    })?;
                continue
            } else if expecting == Perhaps::Maybe {
                if let Some(mut v) = try_parse(text) {
                    *used = true;
                    result.append(&mut v);
                    continue
                } else {
                    expecting = Perhaps::No;
                }
            } 
            assert_eq!(expecting, Perhaps::No);
            match argm.matches(text) {
                // `--list 0 1 2` or `--list 0,1,2`
                ArgMatch::Match => {
                    *used = true;
                    expecting = Perhaps::Yes;
                },
                // `--list=0,1,2`
                ArgMatch::Contains(s) => {
                    *used = true;
                    try_parse(s)
                        .map(|mut v| result.append(&mut v))
                        .ok_or_else(|| ArgError::BadType {
                            long: argm.long, attempt: text.to_string(),
                            exp_type: T::type_name()
                        })?;
                },
                ArgMatch::NoMatch => {},
            }
        }
        if expecting == Perhaps::Yes {
            // still expecting an argument
            Err(ArgError::MissingValue { long: argm.long })
        //} else if result.is_empty() && argm.kind.required {
        //    // list was required but empty
        //    Err(ArgError::MissingArg { long: argm.long })
        } else {
            // successfully extracted val(s)
            Ok(result)
        }
    }
}


#[cfg(test)]
mod test {
    use YaapArg;
    use arg::{ArgM, ListArg};
    use arg::err::ArgResult;
    use arg::test::own;
    //use std::str::FromStr;

    fn list_helper<T: YaapArg>(s: &'static str) -> ArgResult<Vec<T>> {
        let mut argm: ArgM<ListArg<T>> = ArgM::from("list", "").with_short('l');
        let mut args = own(s);
        argm.extract(&mut args)
    }

    #[test]
    fn matches_one() {
        assert_eq!(Ok(vec![1u8]), list_helper("--list 1"));
        assert_eq!(Ok(vec![1u8]), list_helper("--list=1"));
    }

    #[test]
    fn matches_many() {
        assert_eq!(Ok(vec![1u8, 2, 3]), list_helper("--list 1 2 3"));
        assert_eq!(Ok(vec![1u8, 2, 3]), list_helper("--list 1,2,3"));
        assert_eq!(Ok(vec![1u8, 2, 3]), list_helper("--list=1,2,3"));
    }

    #[test]
    fn many_matches() {
        assert_eq!(Ok(vec![1u8,2,3]), list_helper("-l 1 -l 2 -l 3"));
        assert_eq!(Ok(vec![1u8,2,3]), list_helper("-l=1 -l 2,3"));
    }

    #[test]
    fn stop_at_bad_type() {
        assert_eq!(Ok(vec![1u8, 2]), list_helper("--list 1 2 -3 4"));
        assert!(list_helper::<u8>("--list -1").is_err());
        assert!(list_helper::<u8>("--list=1,-2").is_err());
    }

    #[test]
    fn trailing_comma() {
        assert_eq!(Ok(vec![1u8, 2, 3]), list_helper("--list=1,2,3,"));
        assert_eq!(Ok(vec![1u8, 2, 3]), list_helper("--list 1,2,3,"));
    }

    #[test]
    fn trailing_arg() {
        assert!(list_helper::<u8>("--list").is_err());
        assert!(list_helper::<u8>("--list -1").is_err());
    }

    #[test]
    fn empty_list() {
        // I'm not married to this behavior, but want to be aware if it changes
        // If ListArg could be required, this should maybe be allowable?
        //assert_eq!(Ok(vec![]), list_helper::<u8>("--list="));
        assert!(list_helper::<u8>("--list=").is_err());
    }


}
