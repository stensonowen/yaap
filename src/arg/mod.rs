
// For now ArgTrait implementors T are just shims around Arg<T>
// Is that a good design pattern? It's verbose but it's safe
// I don't think there's a tidier way to do the same thing


pub mod types;
pub use self::types::{ArgTrait, ValArg, ListArg, FlagArg, CountArg};

pub mod err;
use self::err::{ArgError, ArgResult};

#[derive(Debug)]
pub struct Arg<T: ArgTrait> {
    long: &'static str,
    short: Option<char>,
    required: bool,
    help: &'static str,
    kind: T,
    // requires: Vec<Arg|String>
}

impl Arg<FlagArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, FlagArg)
    }
    pub(super) fn matches(&self, s: &str) -> bool {
        FlagArg::matches(self, s).unwrap()
    }
}

impl Arg<CountArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, CountArg)
    }
    pub(super) fn matches(&self, s: &str) -> usize {
        CountArg::matches(self, s).unwrap()
    }
}

impl Arg<ValArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, ValArg)
    }
}

impl Arg<ListArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, ListArg { len: None } )
    }
    pub fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}

impl<T: ArgTrait> Arg<T> {
    fn short_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        if let Some(c) = self.short {
            if s.len() == 2 && s.starts_with(&['-',c][..]) {
                ArgMatch::Match
            } else if s.len() > 2 && s.starts_with(&['-',c,'='][..]) {
                ArgMatch::Contained(&s[3..])
            } else {
                ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }
    fn long_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        if s.starts_with("--") && s[2..].starts_with(self.long) {
            println!("looks good... ({:?}, {:?})", s, self);
            if s.len() == 2 + self.long.len() {
                ArgMatch::Match
            } else if let Some('=') = s.chars().nth(2 + self.long.len()) {
                ArgMatch::Contained(&s[self.long.len()+3..])
            } else {
                ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }
    fn default(long: &'static str, help: &'static str, default: T) -> Arg<T> {
        Arg::<T> {
            long: long,
            short: None,
            required: false,
            help: help,
            kind: default,
        }
    }
    pub fn from(long: &'static str, help: &'static str) -> Arg<T> {
        T::from(long, help)
    }

    // TODO: is this the nicest way to do the builder pattern?
    // is `Arg.foo().is_required(true)` better than `Arg.foo().is_required()`?
    //  the former is more verbose. the latter makes the defaults clearer
    // I dunno. It's worth evaluating

    pub fn with_short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
    pub fn is_required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }

}

#[derive(Debug, PartialEq)]
pub enum ArgMatch<'a> {
    Match,                  // `-c`, `--long`, etc.
    NoMatch,                // <not a valid match>
    Contained(&'a str)      // `-c=XX`, `--long=XX`
}

