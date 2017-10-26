
mod test;

pub mod begin;
//use begin::{BeginsWith, NumMatches};

pub mod arg;
pub use arg::{Arg};

pub mod err;
pub use err::ArgError;

pub mod builder;
pub use builder::Yaap;

use std::fmt::Debug;

pub type ArgResult<T> = Result<T, ArgError>;

#[derive(Debug, PartialEq)]
pub enum ArgMatch<'a> {
    // the argument doesn't pertain to this string
    NoMatch,

    // the argument matches this string
    // for List/Val this means the relevant data is in the next arg
    // e.g. `--verbose` or `--std c99` (`--std` is the matched arg)

    Match,      
    // this string contains the argument and its value
    // not relevant for Flag (which carries no data)
    // e.g. in `-vvv` or `--std=c99` the last 3 chars are contained
    Contains(&'a str),
}

impl<'a> ArgMatch<'a> {
    // e.g. arg.short_matches(s).or_else(|| arg.long_matches(s))
    //  less verbose and the closure is only computed if necessary
    fn or_else<F: FnOnce()-> ArgMatch<'a>>(self, f: F) -> ArgMatch<'a> {
        match self {
            ArgMatch::NoMatch => f(),
            _ => self
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArgMatch2 {
    NoMatch,
    NextArg,
    AtOffset(usize),
}

pub trait ArgTrait : Debug + Default {
    type MatchType;

    fn matches(arg: &Arg<Self>, s: &str) -> Self::MatchType;

    fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a>;
    fn extract_match(arg: &Arg<Self>, s: &str) -> Self::MatchType;
    //fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgResult<ArgMatch<'a>>;
    //fn extract_match(arg: &Arg<Self>, s: &str) -> ArgResult<Self::MatchType>;
}

//use std::str::FromStr;
//pub trait Parsable : Debug + FromStr { }

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
 *
 * negatable flags (e.g. --long, --no-long, -c, --no-c)
 *
 * help message
 *
 * reduce allocations ??
 *
 * unit tests
 *
 * perf??
 */


// use state machine to verify calls are made in the right order
// 1. set yaap options (e.g. `name`, `help`, ...
// 2. extract options (e.g. `contains`, `extract_list`, ...
// 3. collect free arguments
