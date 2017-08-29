use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgMatch2, ArgError};
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct ValArg<T: FromStr + Default + Debug> {
    default: Option<T>
}

impl<T: FromStr + Default + Debug> ArgTrait for ValArg<T> {
    type MatchType = ArgMatch2;

    fn matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 {
        match arg.short_matches(s) {
            ArgMatch2::NoMatch => arg.long_matches(s),
            sm => sm
        }
    }
}

impl<T: FromStr+Default+Debug> Arg<ValArg<T>> {
    pub fn with_default(mut self, def: T) -> Self {
        self.kind.default = Some(def);
        self
    }
}

impl Yaap<YaapOpts> {
    pub fn extract_val<T>(self, result: &mut T, arg: Arg<ValArg<T>>)
        -> Yaap<YaapArgs>
        where T: FromStr + Default + Debug
    {
        let new: Yaap<YaapArgs> = self.into();
        new.extract_val(result, arg)
    }

    pub fn try_extract_val<T>(self, result: &mut Option<T>, arg: Arg<ValArg<T>>)
        -> Yaap<YaapArgs>
        where T: FromStr + Default + Debug
    {
        let new: Yaap<YaapArgs> = self.into();
        new.try_extract_val(result, arg)
    }
}

impl Yaap<YaapArgs> {

    pub fn extract_val<T>(mut self, result: &mut T, arg: Arg<ValArg<T>>) -> Self
        where T: FromStr + Default + Debug
    {
        // TODO: in the future this can just wrap `try_extract_val`
        //  my only worry is that it'll screw up the help message...
        let mut times_set = 0usize;
        // can't use `.windows()` here because might be missing last argument
        //  in that case the args are malformed but should check anyway
        for (i,a) in self.argv.iter().enumerate() {
            let arg_str = match arg.matches(a) {
                ArgMatch2::NoMatch => continue,
                ArgMatch2::AtOffset(i) => &a[i..],
                ArgMatch2::NextArg => match self.argv.get(i+1) {
                    Some(next) => next,
                    None => { self.errs.push(ArgError::MissingValue); continue }
                },
            };
            match arg_str.parse() {
                Ok(arg_val) => {
                    *result = arg_val;
                    times_set += 1;
                },
                Err(_) => {
                    // TODO: preserve type?
                    self.errs.push(ArgError::BadType);
                }
            }
        }
        if times_set == 0 {
            if let Some(def) = arg.kind.default {
                *result = def;
            } else {
                self.errs.push(ArgError::MissingArg);
            }
        } else if times_set > 1 {
            self.errs.push(ArgError::Repetition);
        }
        self
    }

    // optional value
    pub fn try_extract_val<T>(mut self, result: &mut Option<T>, arg: Arg<ValArg<T>>) -> Self
        where T: FromStr + Default + Debug
    {
        let mut times_set = 0usize;
        // can't use `.windows()` here because might be missing last argument
        //  in that case the args are malformed but should check anyway
        for (i,a) in self.argv.iter().enumerate() {
            // if relevant, get the 
            let arg_str = match arg.matches(a) {
                ArgMatch2::NoMatch => continue,
                ArgMatch2::AtOffset(i) => &a[i..],
                ArgMatch2::NextArg => {
                    if let Some(next) = self.argv.get(i+1) {
                        next
                    } else {
                        self.errs.push(ArgError::MissingValue);
                        continue
                    }
                },
            };
            match arg_str.parse() {
                Ok(arg_val) => {
                    *result = Some(arg_val);
                    times_set += 1;
                },
                Err(_) => {
                    // TODO: preserve type?
                    self.errs.push(ArgError::BadType);
                    continue
                }
            }
        }
        if times_set == 0 {
            if let Some(def) = arg.kind.default {
                *result = Some(def);
            }
        } else if times_set > 1 {
            self.errs.push(ArgError::Repetition);
        }
        self
    }
}

