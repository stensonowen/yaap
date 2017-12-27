
use YaapArg;
use arg::{ArgS, ArgM, ArgType, ArgMatch};
use arg::err::{ArgError, ArgResult};

#[derive(Debug)]
pub struct ValArg<T: YaapArg> {
    default: Option<T>,
    required: bool,
}

impl<T: YaapArg> Default for ValArg<T> {
    fn default() -> Self {
        ValArg {
            default: None,
            required: false,
        }
    }
}

impl<T: YaapArg> ArgM<ValArg<T>> {
    pub fn with_default(mut self, d: T) -> Self {
        self.kind.default = Some(d);
        self
    }
    pub fn required(mut self) -> Self {
        self.kind.required = true;
        self
    }
}

impl<T: YaapArg> ArgType for ValArg<T> {
    type Contents = Option<T>;

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Option<T>> {
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
                        long: argm.long, attempt: text.to_string(),
                        exp_type: T::type_name(),
                    })
                }
            } else {
                match argm.matches(text) {
                    ArgMatch::Match => {
                        *used = true;
                        if result.is_some() {
                            return Err(ArgError::Repetition{long: argm.long})
                        } else {
                            expecting = true;
                        }
                    },
                    ArgMatch::Contains(s) => {
                        *used = true;
                        if result.is_some() {
                            return Err(ArgError::Repetition{long: argm.long})
                        } else {
                            match s.parse() {
                                Ok(x) => result = Some(x),
                                Err(_) => return Err(ArgError::BadType {
                                    long: argm.long, attempt: text.to_string(),
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
            Err(ArgError::MissingValue { long: argm.long })
        } else if let Some(res) = result {
            // successfully extracted single val
            Ok(Some(res))
        } else if let Some(d) = argm.kind.default.take() {
            // no val but valid default value
            Ok(Some(d))
        } else if argm.kind.required {
            // throw error if arg is required but absent
            Err(ArgError::MissingArg { long: argm.long })
        } else {
            // otherwise, no arg present
            Ok(None)
        }
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
