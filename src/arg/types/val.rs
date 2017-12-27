
use YaapArg;
use arg::{ArgS, ArgM, ArgType, Requirable, ValOptArg};
use arg::err::{ArgError, ArgResult};

#[derive(Debug)]
pub struct ValArg<T: YaapArg> {
    default: Option<T>,
}

impl<T: YaapArg> ValArg<T> {
    pub(crate) fn from(t: Option<T>) -> Self {
        ValArg { default: t }
    }
}

impl<T: YaapArg> Default for ValArg<T> {
    fn default() -> Self {
        ValArg { default: None, }
    }
}

impl<T: YaapArg> Requirable for ValArg<T> {
    fn set_required(&mut self) {
        // TODO this must be reworked
        unimplemented!()
        //self.required = true;
    }
}

/*
impl<T: YaapArg> ArgM<ValArg<T>> {
    pub fn with_default(mut self, d: T) -> Self {
        self.kind.default = Some(d);
        self
    }
}
*/

impl<T: YaapArg> ArgType for ValArg<T> {
    type Contents = T;

    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<T> {
        let mut argm_opt = ArgM::<ValOptArg<T>> {
            long: argm.long, short: argm.short.clone(),
            help: argm.help, kind: ValOptArg::default(), 
        };

        match ValOptArg::extract(&mut argm_opt, args) {
            Err(e) => Err(e),
            Ok(Some(t)) => Ok(t),
            Ok(None) => Err(ArgError::MissingArg { long: argm.long }),
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
