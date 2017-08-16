
//use std::str::FromStr;

/*
pub enum NumArgs {
    Zero,
    Exactly(usize),
    Unlimited,
}
*/

pub trait ArgTrait {
    //type Inner;
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> bool where Self: Sized;
}

pub struct FlagArg { repeatable: bool }
pub struct ValArg;
pub struct ListArg { len: Option<usize> }

impl ArgTrait for FlagArg {
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        // by default don't expect argument to be repeatable
        Arg::<FlagArg>::from(long, help).repeatable(false)
    }
    fn matches(arg: &Arg<FlagArg>, s: &str) -> bool {
        arg.short_matches(s)
    }
}

impl ArgTrait for ValArg {
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::from(long, help)
    }
    fn matches(arg: &Arg<ValArg>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ListArg {
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        // by default assume can take an arbitrary number of values
        Arg::<ListArg>::from(long, help).with_num_args(None)
    }
    fn matches(arg: &Arg<ListArg>, s: &str) -> bool {
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
    fn new(long: &'static str, help: &'static str) -> Self {
        unimplemented!()
    }
    fn repeatable(mut self, rep: bool) -> Self {
        self.kind.repeatable = rep;
        self
    }
}

impl Arg<ValArg> {
    fn new(long: &'static str, help: &'static str) -> Self {
        unimplemented!()
    }
}

impl Arg<ListArg> {
    fn new(long: &'static str, help: &'static str) -> Self {
        unimplemented!()
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
    //fn new(inner: T::Inner) -> Arg<T> { unimplemented!() }
    fn from(long: &'static str, help: &'static str) -> Arg<T> {
        T::from(long, help)
    }
    /*
    pub fn new(long: &'static str, help: &'static str) -> Self { // `from`
        Arg {
            long,
            help,
            short: None,
            required: false,
            //repeatable: false,
            //num_args: NumArgs::Zero,
        }
    }
    */

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

    /*
    pub(super) fn matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        // check for short
        match self.short {
            Some(c) if s.starts_with(&['-',c,'='][..]) => 
                return ArgMatch::Contained(&s[3..]),
            Some(c) if s.starts_with(&['-',c][..]) && s.len() == 2 => 
                return ArgMatch::Match,
            _ => {},
        }
        // check for long
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
    */
}

#[derive(Debug, PartialEq)]
pub enum ArgMatch<'a> {
    Match,                  // `-c`, `--long`, etc.
    NoMatch,                // <not a valid match>
    Contained(&'a str)      // `-c=XX`, `--long=XX`
}

