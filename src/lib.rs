#![allow(unused)]

use std::str::FromStr;
//use std::collections::LinkedList;
use std::env;
use std::mem;

pub mod arg;
pub use arg::{Arg};
use arg::types::{CountArg, FlagArg, ValArg, ListArg};

/*
 * KEEP IN MIND
 *  stdin/stdout
 *      optional hyphen or something?
 *  positional args
 *      "anonymous" args: all the positional/unprompted stuff
 *          e.g. `cp` has 2: `src` and `dst`
 *          these are all arguments that don't start with a hyphen
 *           OR are found after a `--`
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

pub trait BuilderState {}

pub struct YaapOpts;
pub struct YaapArgs;
pub struct YaapDone;

impl BuilderState for YaapOpts {}
impl BuilderState for YaapArgs {}
impl BuilderState for YaapDone {}


pub struct Yaap<T: BuilderState> {
    argv: Vec<String>,
    //free: LinkedList<usize>,
    free: Vec<bool>,
    //args: Vec<Arg<()>>
    //args: Vec<Arg<Box<

    // consider: only have a `desc` and a `help` member
    // have the user craft the `about` section themselves
    // but then if they want the binary name they'll have pop it off argv
    //  and then `help` and `desc` can't be &'static str
    // do this for now
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    help: Option<&'static str>,

    state: T,
}

// drop bomb: require (at runtime) that user called `.finish()` or whatever

impl Drop for YaapOpts {
    fn drop(&mut self) {
        panic!("You probably forgot the `.finish()`");
    }
}

impl Drop for YaapArgs {
    fn drop(&mut self) {
        panic!("You probably forgot the `.finish()`");
    }
}

// transitions in state machine

impl From<Yaap<YaapOpts>> for Yaap<YaapArgs> {
    fn from(old: Yaap<YaapOpts>) -> Yaap<YaapArgs> {
        mem::forget(old.state);
        Yaap {
            argv: old.argv,
            free: old.free,
            name: old.name,
            auth: old.auth,
            desc: old.desc,
            vers: old.vers,
            help: old.help,
            state: YaapArgs,
        }
    }
}

impl From<Yaap<YaapArgs>> for Yaap<YaapDone> {
    fn from(old: Yaap<YaapArgs>) -> Yaap<YaapDone> {
        mem::forget(old.state);
        Yaap {
            argv: old.argv,
            free: old.free,
            name: old.name,
            auth: old.auth,
            desc: old.desc,
            vers: old.vers,
            help: old.help,
            state: YaapDone,
        }
    }
}


impl Yaap<YaapOpts> {
    pub fn create(mut argv: env::Args) -> Yaap<YaapOpts> {
        let name = argv.nth(0).unwrap(); // TODO
        let argv: Vec<_> = argv.collect();
        let free = argv.iter().map(|a| a == "--").collect();
        Yaap {
            argv, free, name,
            auth: None,
            desc: None,
            vers: None,
            help: None,
            state: YaapOpts
        }
    }

    // set options

    pub fn with_name(mut self, n: String) -> Self {
        self.name = n; self
    }
    pub fn with_author(mut self, a: &'static str) -> Self {
        self.auth = Some(a); self
    }
    pub fn with_description(mut self, d: &'static str) -> Self {
        self.desc = Some(d); self
    }
    pub fn with_version(mut self, v: &'static str) -> Self {
        self.vers = Some(v); self
    }
    pub fn with_help(mut self, h: &'static str) -> Self {
        self.help = Some(h); self
    }

    // auto transition to Yaap<YaapArgs>

    pub fn count(self, result: &mut usize, arg: Arg<CountArg>) -> Yaap<YaapArgs> {
        let new: Yaap<YaapArgs> = self.into();
        new.count(result, arg)
    }
    pub fn contains(self, result: &mut bool, arg: Arg<FlagArg>) -> Yaap<YaapArgs> {
        let new: Yaap<YaapArgs> = self.into();
        new.contains(result, arg)
    }
    pub fn extract_val<T>(self, result: &mut T, arg: Arg<ValArg>) 
        -> Yaap<YaapArgs> 
        where T: FromStr
    {
        let new: Yaap<YaapArgs> = self.into();
        new.extract_val(result, arg)
    }
    pub fn extract_list<T>(self, result: &mut Vec<T>, arg: Arg<ListArg>) 
        -> Yaap<YaapArgs> 
        where T: FromStr
    {
        let new: Yaap<YaapArgs> = self.into();
        new.extract_list(result, arg)
    }

    // auto transition to Yaap<YaapDone>
    // TODO

}

impl Yaap<YaapArgs> { 
    pub fn count(self, result: &mut usize, arg: Arg<CountArg>) -> Self {
        let mut count = 0;
        for s in &self.argv {
            count += arg.matches(s);
        }
        *result = count;
        self
    }

    pub fn contains(self, result: &mut bool, arg: Arg<FlagArg>) -> Self {
        *result = false;
        for s in &self.argv {
            if arg.matches(s) {
                *result = true;
                break;
            }
        }
        self
    }

    // required value
    pub fn extract_val<T>(self, result: &mut T, arg: Arg<ValArg>) -> Self 
        where T: FromStr
    {
        unimplemented!()
    }

    // optional value
    pub fn try_extract_val<T>(self, result: &mut Option<T>, arg: Arg<ValArg>) -> Self
        where T: FromStr
    {
        unimplemented!()
    }

    pub fn extract_list<T>(self, result: &mut Vec<T>, arg: Arg<ListArg>) -> Self
        where T: FromStr
    {
        unimplemented!()
    }
}

impl Yaap<YaapDone> { }

/*
mod builder;
use builder::{BuilderState};

pub struct YaapBuilder {
    argv: env::Args,
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    //num_anon: NumArgs,
}

impl YaapBuilder {
    fn new() -> Self {
        Self::from(env::args())
    }

    fn from(mut argv: env::Args) -> Self {
        YaapBuilder {
            name: argv.nth(0).unwrap(), // TODO
            argv: argv,
            auth: None,
            desc: None,
            vers: None,
        }
    }

    fn name(mut self, name: &'static str) -> Self {
        self.name = name.to_string();
        self
    }
    fn author(mut self, auth: &'static str) -> Self {
        self.auth = Some(auth);
        self
    }
    fn description(mut self, desc: &'static str) -> Self {
        self.desc = Some(desc);
        self
    }
    fn version(mut self, vers: &'static str) -> Self {
        self.vers = Some(vers);
        self
    }

    fn get_free_args(argv: &Vec<String>) -> LinkedList<usize> {
        // store the index of all variables that can't be flags
        // i.e. anything not starting with a `-`
        //  or anything after the arg `--`
        let mut free = LinkedList::new();
        let mut remaining_are_free = false;
        for (i,arg) in argv.iter().enumerate() {
            if arg == "--" {
                remaining_are_free = true;
            } else if remaining_are_free || arg.starts_with('-') == false {
                free.push_back(i);
            } 
        }
        free
    }

    pub fn build(self) -> Yaap {
        let all_args = self.argv.collect();
        let free_args = Self::get_free_args(&all_args);
        Yaap { 
            argv: all_args,
            free: free_args,
            name: self.name,
            auth: self.auth,
            desc: self.desc,
            vers: self.vers,
            help: self.vers,
        }
    }

}

pub struct Yaap {
    //argv: env::Args,
    argv: Vec<String>,
    free: LinkedList<usize>,
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    help: Option<&'static str>,

    //num_anon: NumArgs,
    //anon_args: Option<Vec<String>>,     // `Some` means everything's been parsed
    //errors: Vec<String>, // TODO: error type
}

use arg::{ArgTrait, FlagArg, CountArg, ValArg, ListArg};

impl Yaap { 

    // ctors

    pub fn new() -> YaapBuilder {
        YaapBuilder::new()
    }

    pub fn from(args: env::Args) -> YaapBuilder {
        YaapBuilder::from(args)
    }

    // builders


    // safer accessors 


    /*
    pub fn count(self, result: &mut usize, arg: Arg<CountArg>) -> Self {
        let mut count = 0;
        for s in &self.argv {
            count += arg.matches(s);
        }
        *result = count;
        self
    }

    pub fn contains(self, result: &mut bool, arg: Arg<FlagArg>) -> Self {
        *result = false;
        for s in &self.argv {
            if arg.matches(s) {
                *result = true;
                break;
            }
        }
        self
    }

    pub fn extract_val<T>(self, result: &mut T, arg: Arg<ValArg>) -> Self 
        where T: FromStr
    {
        unimplemented!()
    }

    pub fn extract_list<T>(self, result: &mut Vec<T>, arg: Arg<ListArg>) -> Self
        where T: FromStr
    {
        unimplemented!()
    }
    */

    // accessors

    /*
    /// Locate a required argument that takes a value and parse it into `result`
    fn _extract_<T: FromStr>(self, result: &mut T, arg: arg::Arg) -> Self {
        let mut opt: Option<T> = None;
        let attempt = self._try_extract_(&mut opt, arg);
        match opt {
            Some(r) => { *result = r; attempt },
            None => attempt.msg_usage_quit("")
        }
    }

    /// Check for an argument that takes a value and parse it into `result`
    fn _try_extract_<T: FromStr>(self, result: &mut Option<T>, arg: arg::Arg) -> Self {
        *result = None;
        for (i,slice) in self.argv.windows(2).enumerate() {
            // what is the right syntax for this? has it not landed?
            let ref this = slice[0];
            let ref next = slice[1];

            let val = match arg.matches(this) {
                arg::ArgMatch::NoMatch => continue,
                arg::ArgMatch::Contained(val) => val,
                arg::ArgMatch::Match => next,
            };
            match val.parse::<T>() {
                Ok(v) => { *result = Some(v); break },
                Err(e) => self.msg_usage_quit("wrong type dipshit"),
            }
        }
        self
    }

    /// Extract all elements that don't start with a hyphen
    // TODO: different collection types?
    // uhhhhh this needs to be rethought
    // should it really be a runtime error to call .extract() on an arg
    //  with numargs set to many? can't verify lengths are correct statically 
    fn _extract_all_<T: FromStr>(self, result: &mut Vec<T>, arg:arg::Arg) -> Self {
        unimplemented!()
    }

    /// Determine whether a specific flag is set
    // TODO: return option instead? distinguish between flag absent and negated?
    fn _contains_(self, result: &mut bool, arg: arg::Arg) -> Self {
        *result = false;
        for (i,s) in self.argv.iter().enumerate() {
            match arg.matches(s) {
                arg::ArgMatch::NoMatch => continue,
                arg::ArgMatch::Contained(_) => self.msg_usage_quit("fuck"),
                arg::ArgMatch::Match => {
                    // TODO: remove `i`?
                    *result = true;
                    break
                },
            }
        }
        self
    }

    /// Count the number of occurrences of a flag
    // TODO: be a different type?
    fn _count_(self, result: &mut usize, arg: arg::Arg) -> Self {
        // TODO remove after counting?
        let mut count = 0;
        for (i,s) in self.argv.iter().enumerate() {
            match arg.matches(s) {
                arg::ArgMatch::NoMatch => continue,
                arg::ArgMatch::Contained(_) => self.msg_usage_quit("fuck"),
                arg::ArgMatch::Match => count += 1,
            }
        }
        *result = count;
        self
    }
    */


    // helpers

    fn usage_and_quit(&self) -> ! {
        unimplemented!()
    }

    fn msg_usage_quit(&self, msg: &str) -> ! {
        // ughhhh eprintln is nightly :/
        println!("{}", msg);
        self.usage_and_quit()
    }

    /*
    /// Collects all the anonymous arguments internally for easy access
    fn parse_anonymous_args(mut self) -> Self {
        assert!(self.anon_args.is_none());
        // this won't work; needs to be in order
        let anon_args = match self.argv.iter().position(|s| s=="--") {
            Some(i) => self.argv.split_off(i+1),
            None => vec![],
        };
        //if let Some("--") = self.argv.last() { self.argv.pop(); }
        self
    }
    */

}

*/
