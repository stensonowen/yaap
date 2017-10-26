use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgResult, ArgMatch, ArgError};
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct ValArg<T: FromStr + Default + Debug> {
    default: Option<T>
}

impl<T: FromStr + Default + Debug> ArgTrait for ValArg<T> {
    type MatchType = T;

    /*
    fn matches(arg: &Arg<Self>, s: &str) -> Self::MatchType {
        unimplemented!()
        //match arg.short_matches(s) {
        //    ArgMatch2::NoMatch => arg.long_matches(s),
        //    sm => sm
        //}
    }
    */

    //fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
    //    arg.short_matches_(s).or_else(|| arg.long_matches_(s))
    //}
    fn extract_match(arg: &Arg<Self>, s: &str) -> ArgResult<Self::MatchType> {
        s.parse().map_err(|_| ArgError::BadType {
            long: arg.long, attempt: s.to_owned()
        })
        //match s.parse() {
        //    Ok(t) => Ok(t),
        //    Err(_) => Err(ArgError::BadType {
        //        long: arg.long, attempt: s.to_owned()
        //    }),
        //}
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


    fn find_occurrences<T>(&mut self, result: &mut T, mut arg: Arg<ValArg<T>>) -> usize
        where T: FromStr + Default + Debug
    {
        let mut times_set = 0usize;
        // can't use `.windows()` here because might be missing last argument
        assert_eq!(self.argv.len(), self.free.len());
        for (i,(s,free)) in self.argv.iter().zip(self.free.iter_mut()).enumerate() {
            // the text of the arg value: either `--long=X` or `--long X`
            let arg_str = match ValArg::does_match(&arg, s) {
                ArgMatch::Contains(s) => s,
                ArgMatch::NoMatch => continue,
                ArgMatch::Match => match self.argv.get(i+1) {
                    Some(next) => next,
                    None => { 
                        self.errs.push(ArgError::MissingValue { long: arg.long } );
                        // not need to handle free vars? irrecoverable, right?
                        //*free = false;
                        continue 
                    }
                },
            };
            *free = false;
            match ValArg::extract_match(&arg, arg_str) {
                Ok(arg_val) => {
                    *result = arg_val;
                    times_set += 1;
                },
                Err(e) => self.errs.push(e),
            };
        }
        times_set
    }

    pub fn extract_val<T>(mut self, result: &mut T, mut arg: Arg<ValArg<T>>) -> Self
        where T: FromStr + Default + Debug
    {
        // TODO: in the future this can just wrap `try_extract_val`
        //  my only worry is that it'll screw up the help message...
        let mut times_set = 0usize;
        // can't use `.windows()` here because might be missing last argument
        //  in that case the args are malformed but should check anyway
        assert_eq!(self.argv.len(), self.free.len());
        for (i,(s,free)) in self.argv.iter().zip(self.free.iter_mut()).enumerate() {
            // the text of the arg value: either `--long=X` or `--long X`
            let arg_str = match ValArg::does_match(&arg, s) {
                ArgMatch::Contains(s) => s,
                ArgMatch::NoMatch => continue,
                ArgMatch::Match => match self.argv.get(i+1) {
                    Some(next) => next,
                    None => { 
                        self.errs.push(ArgError::MissingValue { long: arg.long } );
                        // not need to handle free vars? irrecoverable, right?
                        //*free = false;
                        continue 
                    }
                },
            };
            *free = false;
            match ValArg::extract_match(&arg, arg_str) {
                Ok(arg_val) => {
                    *result = arg_val;
                    times_set += 1;
                },
                Err(e) => self.errs.push(e),
            };
        }
        if times_set == 0 {
            if let Some(def) = arg.kind.default.take() {
                *result = def;
            } else {
                self.errs.push(ArgError::MissingArg { long: arg.long });
            }
        } else if times_set > 1 {
            // viable for a list but not here
            self.errs.push(ArgError::Repetition { long: arg.long } );
        }
        self.args.push(arg.strip_type());
        self
    }

    // optional value
    pub fn try_extract_val<T>(mut self, result: &mut Option<T>, mut arg: Arg<ValArg<T>>)
        -> Self
        where T: FromStr + Default + Debug
    {
        let mut times_set = 0usize;
        // can't use `.windows()` here because might be missing last argument
        //  in that case the args are malformed but should check anyway
        for (i,a) in self.argv.iter().enumerate() {
            // if relevant, get the 
            let arg_str = "foo";
            /*
            let arg_str = match arg.matches(a) {
                ArgMatch2::NoMatch => continue,
                ArgMatch2::AtOffset(i) => &a[i..],
                ArgMatch2::NextArg => {
                    if let Some(next) = self.argv.get(i+1) {
                        next
                    } else {
                        self.errs.push(ArgError::MissingValue { long: arg.long } );
                        self.free[i] = false;
                        continue
                    }
                },
            };
            */
            self.free[i] = false;
            match arg_str.parse() {
                Ok(arg_val) => {
                    *result = Some(arg_val);
                    times_set += 1;
                },
                Err(_) => {
                    // TODO: preserve type?
                    self.errs.push(ArgError::BadType { 
                        long: arg.long, attempt: arg_str.to_owned()
                    });
                    continue
                }
            }
        }
        if times_set == 0 {
            if let Some(def) = arg.kind.default.take() {
                *result = Some(def);
            }
        } else if times_set > 1 {
            self.errs.push(ArgError::Repetition { long: arg.long } );
        }
        self.args.push(arg.strip_type());
        self
    }
}

