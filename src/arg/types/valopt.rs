
use YaapArg;
use arg::{ArgS, ArgM, ArgType, ArgMatch, Requirable, ValArg};
use arg::err::{ArgError, ArgResult};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ValOptArg<T: YaapArg>(PhantomData<T>);

impl<T: YaapArg> Default for ValOptArg<T> {
    fn default() -> Self {
        ValOptArg(PhantomData)
    }
}


impl<T: YaapArg> ArgM<ValOptArg<T>> {
    pub fn default(self, d: T) -> ValArg<T> {
        ValArg::from(Some(d))
    }
    // so... this might make our cool `trait Requirable` soln impossible...
    pub fn required(self) -> ValArg<T> {
        ValArg::from(None)
    }
}

/*
impl<T: YaapArg> Requirable for ValOptArg<T> {
    fn set_required(&mut self) {
        //self.required = true;
        // TODO how does this work? ret a different type?
        unimplemented!()
    }
}

impl<T: YaapArg> ArgM<ValOptArg<T>> {
    pub fn with_default(mut self, d: T) -> Self {
        self.kind.default = Some(d);
        self
    }
}
*/


impl<T: YaapArg> ArgType for ValOptArg<T> {
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
        } else {
            // otherwise probably fine
            Ok(result)
        }
    }
}

