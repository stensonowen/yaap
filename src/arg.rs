
use std::fmt::Debug;
use std::num::ParseIntError;

pub trait ArgTrait : Debug {
    //type Match;
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    //fn matches(arg: &Arg<Self>, s: &str) -> bool where Self: Sized;
    //fn matches(arg: &Arg<Self>, s: &str) -> Self::Match where Self: Sized;
    fn matches<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> where Self: Sized;
}

#[derive(Debug)] pub struct FlagArg;
#[derive(Debug)] pub struct CountArg;
#[derive(Debug)] pub struct ValArg;
#[derive(Debug)] pub struct ListArg { len: Option<usize> }

impl ArgTrait for FlagArg {
    fn from(long: &'static str, help: &'static str) -> Arg<FlagArg> {
        Arg::<FlagArg>::new(long, help)
    }
    fn matches<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        //  fn("-c") -> Match
        //  fn("--long") -> Match
        //  fn("-c=XX") -> Contained("XX")
        arg.short_matches(s).or_else(|| arg.long_matches(s))
    }
}

impl ArgTrait for CountArg {
    fn from(long: &'static str, help: &'static str) -> Arg<CountArg> {
        Arg::<CountArg>::new(long, help)
    }
    fn matches<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        // match:
        //  fn("-c") -> Match
        //  fn("--long") -> Match
        //  fn("-cccc") -> Contained("cccc")
        //  fn("--long=4") -> Contained("4")
        // don't match:
        //  fn("-c=cc") -> Contained("cc")
        //  fn("--longlong") -> Contained("longlong")
        let n_long_matches = s.begins_with_n("--", arg.long, '=');
        if n_long_matches == 3 {
            let substr = &s[arg.long.len()+3..];
            if substr.len() > 0 && substr.chars().all(char::is_numeric) {
                //  "--long=42"
                ArgMatch::Contained(substr)
            } else {
                //  "--long=foo"
                ArgMatch::NoMatch
            }
        } else if n_long_matches == 2 {
            if arg.long.len() + 2 == s.len() {
                //  "--long"
                ArgMatch::Match
            } else {
                //  "--longfoo"
                ArgMatch::NoMatch
            }
        } else if let Some(c) = arg.short {
            assert!(n_long_matches < 2);
            let n_short_matches = s.begins_with_n('-', c, '=');
            if n_short_matches == 3 && s.len() > 3 {
                let substr = &s[3..];
                if substr.chars().all(char::is_numeric) {
                    //  "-c=42"
                    ArgMatch::Contained(substr)
                } else {
                    //  "-c=foo"
                    ArgMatch::NoMatch
                }
            } else if n_short_matches == 2 {
                let substr = &s[2..];
                if substr.len() == 0 {
                    //  "-c"
                    ArgMatch::Match
                } else {
                    //  "-cfoo"
                    ArgMatch::Contained(substr)
                }
            } else {
                ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }
}

impl ArgTrait for ValArg {
    //type Match = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<ValArg> {
        Arg::<ValArg>::new(long, help)
    }
    //fn matches(arg: &Arg<Self>, s: &str) -> bool {
    //fn matches<'a>(arg: &'a Arg<Self>, s: &str) -> ArgMatch<'a> {
    fn matches<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        //arg.short_matches(s) || arg.long_matches(s) == ArgMatch::Match
        unimplemented!()
    }
}

impl ArgTrait for ListArg {
    //type Match = bool;
    fn from(long: &'static str, help: &'static str) -> Arg<ListArg> {
        Arg::<ListArg>::new(long, help)
    }
    //fn matches(arg: &Arg<Self>, s: &str) -> bool {
    //    arg.short_matches(s) || arg.long_matches(s) != ArgMatch::NoMatch
    //}
    //fn matches<'a>(arg: &'a Arg<Self>, s: &str) -> ArgMatch<'a> {
    fn matches<'a>(arg: &Arg<Self>, s: &'a str) -> ArgMatch<'a> {
        unimplemented!()
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
    pub(super) fn matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        FlagArg::matches(self, s)
    }
}

impl Arg<CountArg> {
    fn new(long: &'static str, help: &'static str) -> Self {
        Arg::default(long, help, CountArg)
    }
    pub(super) fn matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
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

struct Endl;
impl Begins for Endl {
    fn begins(&self, c: &mut Chars) -> bool {
        c.next().is_none()
    }
}

trait BeginsWith {
    // easy to continue if you want
    fn begins_with_1<A: Begins>(&self, a: A) -> bool;
    fn begins_with_2<A: Begins, B: Begins>(&self, a: A, b: B) -> bool;
    fn begins_with_3<A: Begins, B: Begins, G: Begins>(&self, a: A, b: B, g: G) -> bool;
    fn begins_with_n<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C) -> u8;
}

impl BeginsWith for str {
    fn begins_with_1<A: Begins>(&self, a: A) -> bool {
        let mut c = self.chars();
        a.begins(&mut c)
    }
    fn begins_with_2<A: Begins, B: Begins>(&self, a: A, b: B) -> bool {
        let mut c = self.chars();
        a.begins(&mut c) && b.begins(&mut c)
    }
    fn begins_with_3<A: Begins, B: Begins, G: Begins>(&self, a: A, b: B, g: G) 
        -> bool 
    {
        let mut c = self.chars();
        a.begins(&mut c) && b.begins(&mut c) && g.begins(&mut c)
    }
    fn begins_with_n<A: Begins, B: Begins, G: Begins>(&self, a: A, b: B, g: G) -> u8 {
        // not a great way to generalize this without a macro and boxed trait
        let mut chars = self.chars();
        if a.begins(&mut chars) == false {
            0
        } else if b.begins(&mut chars) == false {
            1
        } else if g.begins(&mut chars) == false {
            2
        } else {
            3
        }
    }
}


impl<T: ArgTrait> Arg<T> {

    fn short_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        if let Some(c) = self.short {
            match s.begins_with_n('-', c, '=') {
                3   => ArgMatch::Contained(&s[3..]),
                2   => ArgMatch::Match,
                0|1 => ArgMatch::NoMatch,
                _   => unreachable!(),
            }
        } else {
            ArgMatch::NoMatch
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

impl<'a> ArgMatch<'a> {
    fn is_match(&self) -> bool {
        self != &ArgMatch::NoMatch
    }
    fn or_else<F: FnOnce() -> ArgMatch<'a>>(self, f: F) -> ArgMatch<'a> 
    //fn or_else<F, T>(self, f: F, arg: &Arg<T>, s: &'a str) -> ArgMatch<'a> 
    //fn or_else<F, T>(self, f: F) -> ArgMatch<'a> 
        //where F: Fn(&Arg<T>, &str) -> ArgMatch<'a>, T: ArgTrait
    //fn or_else<F, T>(self, f: F, s: &'a str) -> ArgMatch<'a> 
    //    where F: Fn(&str) -> ArgMatch<'a>, T: ArgTrait
    {
        // not really sure how to make this not janky without currying or something
        if self == ArgMatch::NoMatch {
            f()
            //f(arg, s)
            //f(s)
        } else {
            self
        }
    }
}

