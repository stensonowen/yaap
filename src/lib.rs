
pub mod arg;
pub use arg::{Arg};
//use arg::types::{ArgTrait, ArgMatch2, CountArg, FlagArg, ValArg, ListArg};
//use arg::types::{ArgMatch2};
use arg::err::ArgError;

pub mod builder;

#[derive(Debug, PartialEq)]
pub enum ArgMatch2 {
    NoMatch,
    NextArg,
    AtOffset(usize),
}

use std::fmt::Debug;
pub trait ArgTrait : Debug + Default {
    type MatchType;
    // TODO kill short/long matches
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    fn short_matches(arg: &Arg<Self>, s: &str) -> Self::MatchType
        where Self: Sized;
    fn long_matches(arg: &Arg<Self>, s: &str) -> Self::MatchType
        where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> Self::MatchType
        where Self: Sized;
}

/*
 * KEEP IN MIND
 *  stdin/stdout
 *      optional hyphen or something?
 *  positional args
 *      "anonymous" args: all the positional/unprompted stuff
 *          e.g. `cp` has 2: `src` and `dst`
 *          these are all arguments that don't start with a hyphen
 *           OR are found after a `--`
 *  string args that have spaces (with quotes)
 *  negatable args
 *      add flag to `short_matches` and `long_matches`
 *      add function to `arg`?
 *      can only some args be negatable?
 *
 * uhhh should .contains / .count pop elements??
 *  should there be 2 copies to maintain?
 *  if someone ever makes the same call twice it'll silently fail the second time
 *  there's no reason to do that
 *  maybe should change name to `pop`?
 *
 * generating the usage is gonna be strange. we want to collect all possible
 *  arguments before making/printing the usage, but arguments are handled in 
 *  the moment. maybe the type that gets chained together is a Result? Or 
 *  maybe just maintain a list of args and a list of errors and then at the 
 *  end if there are any errors use all the args to print the usage. it will be
 *  ambiguous whether all args were set properly (?) but it will always panic 
 *  so it's fine. 
 * The only concern is that we need a `.finish()` or something at the end, and
 *  I'm not sure there's a great way to make that extremely clear. #[must_use]
 *  would be cool but it doesn't look usable/stable/enabled/?
 *  Maybe overriding Drop or something?
 *  And then `.bind_free_vars(&mut Vec<_>)` or something could be used 
 *   after the early stage but before the `finish` stage (maybe use a state
 *   machine kind of pattern to enforce this?)
 */


// use state machine to verify calls are made in the right order
// 1. set yaap options (e.g. `name`, `help`, ...
// 2. extract options (e.g. `contains`, `extract_list`, ...
// 3. collect free arguments

/*
impl Yaap<YaapArgs> { 
    pub fn count(mut self, result: &mut usize, arg: Arg<CountArg>) -> Self {
        let mut count = 0;
        for s in &self.argv {
            match arg.matches(s) {
                Ok(n) => count += n,
                Err(e) => self.errs.push(e),
            }
        }
        *result = count;
        self
    }

    pub fn contains(mut self, result: &mut bool, arg: Arg<FlagArg>) -> Self {
        *result = false;
        for s in &self.argv {
            match arg.matches(s) {
                Ok(true) => {
                    *result = true;
                    break
                },
                Ok(false) => continue,
                Err(e) => self.errs.push(e),
            }
        }
        self
    }

    // required value
    pub fn extract_val<T>(mut self, result: &mut T, arg: Arg<ValArg>) -> Self 
        where T: FromStr
    {
        // TODO: in the future this can just wrap `try_extract_val`
        //  my only worry is that it'll screw up the help message...
        let mut times_set = 0usize;
        // can't use `.windows()` here because might be missing last argument
        //  in that case the args are malformed but should check anyway
        for (i,a) in self.argv.iter().enumerate() {
            // if relevant, get the 
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
                Err(e) => {
                    self.errs.push(ArgError::BadType);
                }
            }
        }
        if times_set == 0 {
            self.errs.push(ArgError::Missing);
        } else if times_set > 1 {
            self.errs.push(ArgError::Repetition);
        }
        self
    }

    // optional value
    pub fn try_extract_val<T>(mut self, result: &mut Option<T>, arg: Arg<ValArg>) -> Self
        where T: FromStr
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
                Err(e) => {
                    self.errs.push(ArgError::BadType);
                    continue
                }
            }
        }
        if times_set == 0 {
            self.errs.push(ArgError::Missing);
        } else if times_set > 1 {
            self.errs.push(ArgError::Repetition);
        }
        self
    }

    pub fn extract_list<T>(mut self, result: &mut Vec<T>, arg: Arg<ListArg>) -> Self
        where T: FromStr
    {
        let mut res_vec = vec![];
        for (i,a) in self.argv.iter().enumerate() {
            let matches = arg.matches(a);
            if matches == ArgMatch2::NextArg {
                if let Some(next_args) = self.argv.get(i+1..) {
                    for elem in next_args {
                        if elem.starts_with('-') {
                            break
                        }
                        match elem.parse() {
                            Ok(e) => res_vec.push(e),
                            Err(e) => self.errs.push(ArgError::BadType),
                        }
                    }
                } else {
                    self.errs.push(ArgError::Missing);
                }
            } else if let ArgMatch2::AtOffset(j) = matches {
                for elem in a[j..].split(',') {
                    match elem.parse() {
                        Ok(e) => res_vec.push(e),
                        Err(e) => self.errs.push(ArgError::BadType),
                    }
                }
            } 
        }
        *result = res_vec;
        self
    }

    pub fn finish(self) {
        mem::forget(self.state);
        if self.errs.is_empty() {
            println!("No errors!");
        } else {
            println!("Errors: {:?}", self.errs);
        }
    }
}
*/

//impl Yaap<YaapDone> { }

