
// For now ArgTrait implementors T are just shims around Arg<T>
// Is that a good design pattern? It's verbose but it's safe
// I don't think there's a tidier way to do the same thing

pub trait ArgTrait {
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> bool where Self: Sized;
}

pub struct FlagArg;
pub struct CountArg;
pub struct ValArg;
pub struct ListArg { len: Option<usize> }

impl ArgTrait for FlagArg {
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for CountArg {
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ValArg {
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ListArg {
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
    }
}


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
        FlagArg::matches(self, s)
    }
}

impl Arg<CountArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, CountArg)
    }
    pub(super) fn matches(&self, s: &str) -> bool {
        CountArg::matches(self, s)
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
    fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}

impl<T: ArgTrait> Arg<T> {
    fn short_matches(&self, s: &str) -> bool {
        if let Some(c) = self.short {
            s.len() == 2 && s.starts_with(&['-',c][..])
        } else {
            false
        }
    }
    fn long_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        if s.starts_with("--") && s[2..].starts_with(self.long) {
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

    fn with_short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
    fn is_required(mut self, req: bool) -> Self {
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

