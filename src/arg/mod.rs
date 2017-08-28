
// For now ArgTrait implementors T are just shims around Arg<T>
// Is that a good design pattern? It's verbose but it's safe
// I don't think there's a tidier way to do the same thing


pub mod types;
//pub use self::types::{ArgTrait, ValArg, ListArg, FlagArg, CountArg};
//pub use self::types::{ArgTrait, };
pub use super::ArgTrait;
//pub use self::types::ArgMatch2;
pub use super::ArgMatch2;

pub mod err;
pub use self::err::{ArgError, ArgResult};

#[derive(Debug)]
pub struct Arg<T: ArgTrait> {
    pub(crate) long: &'static str,
    pub(crate) short: Option<char>,
    required: bool,
    help: &'static str,
    pub(crate) kind: T,
    // requires: Vec<Arg|String>
}

/*
impl Arg<ListArg> {
    pub fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}
*/

impl<M,T> Arg<T> where T: ArgTrait<MatchType=M> {

    pub fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, T::default())
    }

    pub fn from(long: &'static str, help: &'static str) -> Arg<T> {
        T::from(long, help)
    }

    pub(super) fn matches(&self, s: &str) -> M {
        T::matches(self, s)
    }

    // fn get_metadata(self) -> Arg<()> { unimplemented!() }

    pub fn short_matches(&self, s: &str) -> ArgMatch2 {
        if let Some(c) = self.short {
            if s.len() == 2 && s.starts_with(&['-',c][..]) {
                ArgMatch2::NextArg
            } else if s.len() > 2 && s.starts_with(&['-',c,'='][..]) {
                ArgMatch2::AtOffset(3)
            } else {
                ArgMatch2::NoMatch
            }
        } else {
            ArgMatch2::NoMatch
        }
    }
    pub fn long_matches(&self, s: &str) -> ArgMatch2 {
        if s.starts_with("--") && s[2..].starts_with(self.long) {
            println!("looks good... ({:?}, {:?})", s, self);
            if s.len() == 2 + self.long.len() {
                ArgMatch2::NextArg
            } else if let Some('=') = s.chars().nth(2 + self.long.len()) {
                ArgMatch2::AtOffset(self.long.len()+3)
            } else {
                ArgMatch2::NoMatch
            }
        } else {
            ArgMatch2::NoMatch
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

/*
#[derive(Debug, PartialEq)]
pub enum ArgMatch<'a> {
    Match,                  // `-c`, `--long`, etc.
    NoMatch,                // <not a valid match>
    Contained(&'a str)      // `-c=XX`, `--long=XX`
}
*/
