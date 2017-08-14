#![allow(unused)]

use std::str::FromStr;
use std::env;

mod arg;
use arg::{NumArgs, };

/*
 * KEEP IN MIND
 *  stdin/stdout
 *      optional hyphen or something?
 *  positional args
 *      "anonymous" args: all the positional/unprompted stuff
 *          e.g. `cp` has 2: `src` and `dst`
 *          these are all arguments that don't start with a hyphen
 *           OR are found after a `--`
 */

//struct Yaap(env::Args);


struct YaapBuilder {
    argv: env::Args,
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    num_anon: NumArgs,
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
            num_anon: NumArgs::Zero,
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

    fn build(self) -> Yaap {
        Yaap { 
            argv: self.argv.collect(),
            name: self.name,
            auth: self.auth,
            desc: self.desc,
            vers: self.vers,
            help: self.vers,
            num_anon: self.num_anon,
            anon_args: None,
        }
    }

}

struct Yaap {
    //argv: env::Args,
    argv: Vec<String>,
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    help: Option<&'static str>,

    num_anon: NumArgs,
    anon_args: Option<Vec<String>>,     // `Some` means everything's been parsed
    //errors: Vec<String>, // TODO: error type
}

impl Yaap { 
    fn new() -> YaapBuilder {
        YaapBuilder::new()
    }

    fn from(args: env::Args) -> YaapBuilder {
        YaapBuilder::from(args)
    }

    /// Locate an argument that takes a value and parse it into `result`
    fn extract<T: FromStr>(self, result: &mut T, arg: arg::Arg) -> Self {
        self
    }

    /// Extract all elements that don't start with a hyphen
    // TODO: different collection types?
    fn extract_all<T: FromStr>(self, result: &mut Vec<T>, arg:arg::Arg) -> Self {
        self
    }

    /// Determine whether a specific flag is set
    // TODO: return option instead? distinguish between flag absent and negated?
    fn contains(self, result: &mut bool, arg: arg::Arg) -> Self {
        self
    }

    /// Count the number of occurrences of a flag
    // TODO: be a different type?
    fn count(self, result: &mut usize, arg: arg::Arg) -> Self {
        self
    }

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


}

