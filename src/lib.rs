#![allow(unused)]

use std::str::FromStr;
use std::env;

pub mod arg;
pub use arg::{Arg};

/*
 * KEEP IN MIND
 *  stdin/stdout
 *      optional hyphen or something?
 *  positional args
 *      "anonymous" args: all the positional/unprompted stuff
 *          e.g. `cp` has 2: `src` and `dst`
 *          these are all arguments that don't start with a hyphen
 *           OR are found after a `--`
 *
 * uhhh should .contains / .count pop elements??
 *  should there be 2 copies to maintain?
 *  if someone ever makes the same call twice it'll silently fail the second time
 *  there's no reason to do that
 *  maybe should change name to `pop`?
 */

//struct Yaap(env::Args);


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
            //num_anon: NumArgs::Zero,
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

    pub fn build(self) -> Yaap {
        Yaap { 
            argv: self.argv.collect(),
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
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    help: Option<&'static str>,

    //num_anon: NumArgs,
    //anon_args: Option<Vec<String>>,     // `Some` means everything's been parsed
    //errors: Vec<String>, // TODO: error type
}

impl Yaap { 

    // ctors

    pub fn new() -> YaapBuilder {
        YaapBuilder::new()
    }

    pub fn from(args: env::Args) -> YaapBuilder {
        YaapBuilder::from(args)
    }

    // builders



    // accessors

    /// Locate a required argument that takes a value and parse it into `result`
    pub fn extract<T: FromStr>(self, result: &mut T, arg: arg::Arg) -> Self {
        let mut opt: Option<T> = None;
        let attempt = self.try_extract(&mut opt, arg);
        match opt {
            Some(r) => { *result = r; attempt },
            None => attempt.msg_usage_quit("")
        }
    }

    /// Check for an argument that takes a value and parse it into `result`
    pub fn try_extract<T: FromStr>(self, result: &mut Option<T>, arg: arg::Arg) -> Self {
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
    fn extract_all<T: FromStr>(self, result: &mut Vec<T>, arg:arg::Arg) -> Self {
        unimplemented!()
    }

    /// Determine whether a specific flag is set
    // TODO: return option instead? distinguish between flag absent and negated?
    pub fn contains(self, result: &mut bool, arg: arg::Arg) -> Self {
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
    pub fn count(self, result: &mut usize, arg: arg::Arg) -> Self {
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

