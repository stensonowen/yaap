
use std::fmt::Debug;
use std::num::ParseIntError;

pub trait ArgTrait : Debug {
    type Match;
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    //fn matches(arg: &Arg<Self>, s: &str) -> bool where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> Self::Match where Self: Sized;
}

#[derive(Debug)] pub struct FlagArg;
#[derive(Debug)] pub struct CountArg;
#[derive(Debug)] pub struct ValArg;
#[derive(Debug)] pub struct ListArg { len: Option<usize> }

impl ArgTrait for FlagArg {
    type Match = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for CountArg {
    type Match = Result<usize, ParseIntError>;
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> Self::Match {
        if s.starts_with("--") && 
            s[2..].matches(arg.long).count() * arg.long.len() == s.len()-2 {
                Ok((s.len()-2) / arg.long.len())
        } else if s.starts_with("-") && 
            //arg.short.map(|c| s[1..].matches(c).count() == s.len()-1).unwrap_or(false) {
            arg.short.map(|c| s.chars().skip(1).all(|i| i==c)).unwrap_or(false) {
                Ok(s.len()-1)
        } else if let ArgMatch::Contained(n) = arg.long_matches(s) {
            n.parse()
        } else {
            Ok(0)
        }
        //arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ValArg {
    type Match = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
    }
}

impl ArgTrait for ListArg {
    type Match = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    fn matches(arg: &Arg<Self>, s: &str) -> bool {
        arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
    }
}


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
    fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, FlagArg)
    }
    pub(super) fn matches(&self, s: &str) -> bool {
        FlagArg::matches(self, s)
    }
}

impl Arg<CountArg> {
    fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, CountArg)
    }
    pub(super) fn matches(&self, s: &str) -> Result<usize,ParseIntError> {
        CountArg::matches(self, s)
    }
}

impl Arg<ValArg> {
    fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, ValArg)
    }
}

impl Arg<ListArg> {
    fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, ListArg { len: None } )
    }
    pub fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}

use std::str::Chars;
use std::iter;

trait Begins {
    fn begins(&self, chars: &mut Chars) -> bool;
}

impl Begins for char {
    fn begins(&self, chars: &mut Chars) -> bool { Some(*self) == chars.next() }
}

impl<'a> Begins for &'a str {
    fn begins(&self, c: &mut Chars) -> bool { 
        c.as_str().len()>=self.len() && self.chars().zip(c).all(|(a,b)| a==b)
    }
}

trait BeginsWith {
    // easy to continue if you want
    fn begins_with_a<A: Begins>(&self, a: A) -> bool;
    fn begins_with  <A: Begins, B: Begins>(&self, a: A, b: B) -> bool;
    fn begins_with_n<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C) -> usize;
}

impl BeginsWith for str {
    fn begins_with_a<A: Begins>(&self, a: A) -> bool {
        let mut c = self.chars();
        a.begins(&mut c)
    }
    fn begins_with<A: Begins, B: Begins>(&self, a: A, b: B) -> bool {
        let mut c = self.chars();
        a.begins(&mut c) && b.begins(&mut c)
    }
    fn begins_with_n<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C) -> usize {
        // not a great way to generalize this without a macro and boxed trait
        let mut chars = self.chars();
        if a.begins(&mut chars) == false {
            0
        } else if b.begins(&mut chars) == false {
            1
        } else if c.begins(&mut chars) == false {
            2
        } else {
            3
        }
    }
}


impl<T: ArgTrait> Arg<T> {

    fn short_matches(&self, s: &str) -> bool {
        if let Some(c) = self.short {
            s.len()==2 && s.begins_with('-', c)
        } else {
            false
        }
    }
    fn long_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        match s.begins_with_n("--", self.long, '=') {
            3   => ArgMatch::Contained(&s[self.long.len()+3..]),
            2   => ArgMatch::Match,
            0|1 => ArgMatch::NoMatch,
            _   => unreachable!(),
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

