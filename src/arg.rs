
use std::fmt::Debug;

pub trait ArgTrait : Debug {
    fn from(long: &'static str, help: &'static str) -> Arg<Self> 
        where Self: Sized;
    fn matches(arg: &Arg<Self>, s: &str) -> bool where Self: Sized;
}

#[derive(Debug)] pub struct FlagArg;
#[derive(Debug)] pub struct CountArg;
#[derive(Debug)] pub struct ValArg;
#[derive(Debug)] pub struct ListArg { len: Option<usize> }

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
    pub(super) fn matches(&self, s: &str) -> bool {
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

//  s.begins_with('-',char)
//  s.begins_with("--", "foo")

use std::str::Chars;
use std::iter;

trait Begins {
    //fn size(&self) -> usize;
    fn begins(&self, chars: &mut Chars) -> bool;
    //fn equals(&self, chars: &mut Chars) -> bool;
}

impl Begins for char {
    //fn size(&self) -> usize { 1 }
    fn begins(&self, chars: &mut Chars) -> bool { chars.next() == Some(*self) }
    //fn equals(&self, chars: &mut Chars) -> bool { chars.eq(iter::once(*self)) }
}

impl<'a> Begins for &'a str {
    //fn size(&self) -> usize { self.len() }
    //c.as_str().len()>=self.len() && self.chars().zip(c).all(|(a,b)| a==b)
    //fn begins(&self, c: &mut Chars) -> bool { c.as_str().starts_with(self) }
    //fn equals(&self, c: &mut Chars) -> bool { c.as_str() == self }
    //self.chars().eq(c)

    fn begins(&self, c: &mut Chars) -> bool { 
        c.as_str().len()>=self.len() && self.chars().zip(c).all(|(a,b)| a==b)
    }
}

impl Begins for () {
    // just so we can opt to not use args or something idk
    fn begins(&self, _: &mut Chars) -> bool { false }
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
    //jfn begins_wth_n<A,B,C,D,E>(&self, a: A, b: B, c: C, d: D, e: E) -> usize
    //j    where A: Begins, B: Begins, C: Begins, D: Begins, E: Begins
    //j{
    fn begins_with_n<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C) -> usize {
        // not a great way to generalize this without a macro and boxed trait
        let mut chars = self.chars();
        if a.begins(&mut chars) == false {
            //println!("ENDED WITH `{}`", chars.as_str());
            0
        } else if b.begins(&mut chars) == false {
            //println!("ENDED WITH `{}`", chars.as_str());
            1
        } else if c.begins(&mut chars) == false {
            //println!("ENDED WITH `{}`", chars.as_str());
            2
        } else {
            //println!("ENDED WITH `{}`", chars.as_str());
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
        //println!("   {:?}   {}   ", self, s);
        //let x = s.begins_with_n("--", self.long, '=');
        //println!("{}", "--x".begins_with_n("--",self.long,'='));
        //println!("{}\t{}\t{}", s, x, self.long);
        match s.begins_with_n("--", self.long, '=') {
        //let n = s.begins_with_n("--", self.long, '=');
        //match n {
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

