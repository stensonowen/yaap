
use YaapArg;
use arg::{ArgS, ArgM, ArgType, ArgMatch, Requirable};
use arg::err::{ArgError, ArgResult};

use std::marker::PhantomData;

// ValArg starts out as an OptionalVal, which has an AT of Option<T>
// It can either be given a default value or be marked required, but not both
// Both variants have an AT of T

/// Different types of a ValArg
pub trait ValType<T: YaapArg> {
    fn get_default(self) -> Option<T>;
}

/// Starting kind of a ValArg: has no default value and is not required
#[derive(Debug)] pub struct OptionalVal;
/// Required variant of ValArg: guarantees something is returned
#[derive(Debug)] pub struct RequiredVal;
/// ValArg variant with a default value: guarantees something is returned
#[derive(Debug)] pub struct DefaultVal<T: YaapArg>(T);

impl<T: YaapArg> ValType<T> for OptionalVal {
    fn get_default(self) -> Option<T> { None }
}
impl<T: YaapArg> ValType<T> for RequiredVal {
    fn get_default(self) -> Option<T> { None }
}
impl<T: YaapArg> ValType<T> for DefaultVal<T> {
    fn get_default(self) -> Option<T> { Some(self.0) }
}

/*
impl<T: YaapArg> Default for DefaultVal<T> {
    fn default() -> Self {
        DefaultVal(None)
    }
}
*/

#[derive(Debug)]
pub struct ValArg<S: ValType<T>, T: YaapArg> {
//pub struct ValArg<T: YaapArg> {
    data: PhantomData<T>,
    kind: S,
    //kind: Box<ValType<T>>,
}

/*
impl<S: ValType<T>, T: YaapArg> Default for ValArg<S, T> {
    fn default() -> Self {
        ValArg {
            data: PhantomData,
            //kind: OptionalVal,
            kind: S::default(),
        }
    }
}
*/

impl<T: YaapArg> Requirable for ValArg<OptionalVal, T> {
    fn set_required(&mut self) {
        //self.kind = RequiredVal;
        unimplemented!()
        //self.required = true;
    }
}

/*
 * Which one should have a default?
impl<T: YaapArg> ArgM<ValArg<RequiredVal, T>> {
    pub fn with_default(mut self, d: T) -> Self {
        self.kind.default = Some(d);
        self
    }
}
*/
impl<S: ValType<T>, T: YaapArg> ArgM<ValArg<S, T>> where ValArg<S,T>: ArgType {
    // helper to be called by different ValTypes
    fn try_extract(&mut self, args: &mut Vec<ArgS>) -> ArgResult<Option<T>> {
        let mut result: Option<T> = None;
        let mut expecting = false;
        for &mut ArgS { ref text, ref mut used } in args.iter_mut() {
            if *used {
                //continue
            } else if expecting {
                expecting = false;
                *used = true;
                match text.parse() {
                    Ok(x) => result = Some(x),
                    Err(_) => return Err(ArgError::BadType {
                        long: self.long, attempt: text.to_string(),
                        exp_type: T::type_name(),
                    })
                }
            } else {
                match self.matches(text) {
                    ArgMatch::Match => {
                        *used = true;
                        if result.is_some() {
                            return Err(ArgError::Repetition{long: self.long})
                        } else {
                            expecting = true;
                        }
                    },
                    ArgMatch::Contains(s) => {
                        *used = true;
                        if result.is_some() {
                            return Err(ArgError::Repetition{long: self.long})
                        } else {
                            match s.parse() {
                                Ok(x) => result = Some(x),
                                Err(_) => return Err(ArgError::BadType {
                                    long: self.long, attempt: text.to_string(),
                                    exp_type: T::type_name()
                                })
                            }
                        }
                    },
                    ArgMatch::NoMatch => {},
                }
            }
        }
        if expecting {
            // still expecting an argument
            Err(ArgError::MissingValue { long: self.long })
        } else if let Some(res) = result {
            // successfully extracted single val
            Ok(Some(res))
        } else {
            // otherwise, no arg present
            Ok(None)
        }
    }
}

impl<T: YaapArg> ArgType for ValArg<OptionalVal, T> {
    type Contents = Option<T>;

    fn new() -> Self {
        ValArg {
            data: PhantomData,
            kind: OptionalVal,
        }
    }

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Option<T>> {
        argm.try_extract(args)
    }
}

impl<T: YaapArg> ArgType for ValArg<RequiredVal, T> {
    type Contents = T;

    fn new() -> Self {
        unreachable!()
    }

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<T> {
        match argm.try_extract(args) {
            Err(e) => Err(e),
            Ok(Some(t)) => Ok(t),
            Ok(None) => Err(ArgError::MissingArg { long: argm.long }),
        }
    }
}

impl<T: YaapArg> ArgType for ValArg<DefaultVal<T>, T> {
    type Contents = T;

    fn new() -> Self {
        unreachable!()
    }

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use YaapArg;
    use arg::{ArgM, ValArg};
    use arg::err::ArgResult;
    use arg::test::own;
    use std::str::FromStr;

    fn val_helper<T: YaapArg>(s: &'static str) -> ArgResult<Option<T>> {
        let mut argm: ArgM<ValArg<T>> = ArgM::from("val", "").with_short('v');
        let mut args = own(s);
        argm.extract(&mut args)
    }

    #[test]
    fn bool() {
        assert_eq!(Ok(Some(true)), val_helper("--val=true"));
    }

    #[test]
    fn num() {
        assert_eq!(Ok(Some(42)), val_helper("--val 42"));
    }

    #[test]
    fn overflow() {
        assert!(val_helper::<u8>("--val 500").is_err())
    }

    #[derive(Debug, PartialEq)]
    enum Color { Red, Green, Blue, }
    impl FromStr for Color {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, ()> {
            match s.to_lowercase().as_ref() {
                "red" | "r" => Ok(Color::Red),
                "green" | "g" => Ok(Color::Green),
                "blue" | "b" => Ok(Color::Blue),
                _ => Err(())
            }
        }
    }
    impl YaapArg for Color {
        fn type_name() -> &'static str { "Color" }
    }

    #[test]
    fn custom_type() {
        assert_eq!(Ok(Some(Color::Red)), val_helper("--val=r"));
        assert_eq!(Ok(Some(Color::Red)), val_helper("--val red"));
        assert!(val_helper::<Color>("--val=x").is_err());
        assert!(val_helper::<Color>("--val y").is_err());
        assert!(val_helper::<Color>("--val re").is_err());
        assert!(val_helper::<Color>("--val redd").is_err());
    }

    #[test]
    fn over_defined() {
        assert!(val_helper::<Color>("--val blue -v=blue").is_err());
        assert!(val_helper::<Color>("--val=blue -v red").is_err());
    }

    #[test]
    fn missing_val() {
        assert!(val_helper::<Color>("--val").is_err());
        assert!(val_helper::<Color>("--val=").is_err());
    }

    #[test]
    fn optional() {
        assert_eq!(Ok(None), val_helper::<Color>("--nothing -2 -c --here"));
    }

    #[test]
    fn mandatory() {
        let mut argm: ArgM<ValArg<Color>> = ArgM::from("val", "").required();
        let mut args = own("--nothing -2 -c --here");
        assert!(argm.extract(&mut args).is_err());
    }

    #[test]
    fn default() {
        let mut argm: ArgM<ValArg<Color>> = ArgM::from("val", "").with_default(Color::Green);
        let mut args = own("--nothing -2 -c --here");
        assert_eq!(Ok(Some(Color::Green)), argm.extract(&mut args));
    }

    #[test]
    fn ignore_used() {
        let mut argm: ArgM<ValArg<Color>> = ArgM::from("val", "");
        let mut args = own("--val BLUE");
        let res1 = argm.extract(&mut args);
        assert_eq!(Ok(Some(Color::Blue)), res1);
        let res2 = argm.extract(&mut args);
        assert_eq!(Ok(None), res2);
    }


}
