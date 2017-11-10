
mod test;
mod begin;

mod arg;
pub use arg::ArgM;

mod err;
use err::ArgError;

mod builder;
pub use builder::Yaap;

mod yaaparg;
pub use yaaparg::YaapArg;

use std::fmt::Debug;

type ArgResult<T> = Result<T, ArgError>;
pub type Arg<T> = ArgM<T>;

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

pub trait ArgTrait : Debug + Sized {
    type MatchType;

    fn does_match<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        arg.short_matches(s).or_else(|| arg.long_matches(s))
    }
    fn extract_match(arg: &Arg<Self>, s: &str) -> ArgResult<Self::MatchType>;
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
 *      should see how kbknapp does it
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
 *
 * use generic that allows for osstring internally?
 *  could use env::args_os() to allow invalid unicode args
 *  would this severely complicate string comparisons?
 *
 * Single hyphen? like as in `vim -`? Not possible currently, right?
 *  A dedicated arg type? Like {FlagArg,ListArg,etc.? HyphenArg?)
 *  Is there a clean way to make it just a FlagArg?
 *      Make FlagArg generic over Option? Need a None, right?
 *
 * Subcommands?
 *  Like cargo build --help or whatever? What's the best way to do this?
 *      Maybe a fifth ArgTrait implementor? That can recognize an enum and
 *          steals the remainder of the arguments?
 *      Maybe a way to nest YaapBuilders? That would be kinda cool but might 
 *          not work
 *
 *  Better naming convention?
 *      Args passed by the user (e.g. `./a.out arg1 arg2 arg3`) arg ArgS (ArgStr)
 *      Args constructed by the dev (e.g. `Arg::from(..)` are ArgM (ArgMatch)
 *          public type alias called `Arg` for benefit of users
 *      DONE
 *
 *  Add a --version/-V option like --help/-h ?
 *
 * Should the .is_required() builder helper be Arg-specific?
 *  I don't think it makes sense to be a method of FlagArg, right?
 *      having a required FlagArg means it must be set to true, right?
 *      that's not particularly useful/expressive
 *      it's a little awkward to be an explicit field for (n-1) of n implementors
 *      but idk that's probably how it should be
 */


// use state machine to verify calls are made in the right order
// 1. set yaap options (e.g. `name`, `help`, ...)
// 2. extract options (e.g. `contains`, `extract_list`, ...)
// 3. collect free arguments
