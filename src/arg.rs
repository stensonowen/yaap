
// For now ArgTrait implementors T are just shims around Arg<T>
// Is that a good design pattern? It's verbose but it's safe
// I don't think there's a tidier way to do the same thing

pub use super::{ArgTrait, ArgResult, ArgMatch};
use begin::{Begins, BeginsWith, NumMatches};

#[derive(Debug)]
pub struct Arg<T: ArgTrait> {
    pub(crate) long: &'static str,
    pub(crate) short: Option<char>,
    pub(crate) required: bool,
    pub(crate) help: &'static str,
    pub(crate) kind: T,
    // requires: Vec<Arg|String>
}


impl<M,T> Arg<T> where T: ArgTrait<MatchType=M> {

    pub fn from(long: &'static str, help: &'static str) -> Arg<T> {
        Arg::<T> {
            long: long,
            short: None,
            required: false,
            help: help,
            kind: T::default(),
        }
    }

    pub(super) fn strip_type(self) -> Arg<()> {
        Arg {
            long: self.long,
            short: self.short,
            required: self.required,
            help: self.help, // heh
            kind: (),
        }
    }

    pub(super) fn does_match<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        T::does_match(self, s)
    }

    pub(super) fn extract_match(&self, arg: &Arg<T>, s: &str) -> ArgResult<M> {
        T::extract_match(self, s)
    }


    pub(super) fn short_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        if let Some(c) = self.short {
            self.begins_with(s, '-', c, '=')
        } else {
            ArgMatch::NoMatch
        }
    }

    pub(super) fn long_matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        self.begins_with(s, "--", self.long, '=')
    }

    fn begins_with<'a, A, B, C>(&self, s: &'a str, a: A, b: B, c: C)
        -> ArgMatch<'a> 
        where A: Begins, B: Begins, C: Begins
    {
        match s.begins_with_n(a, b, c) {
            NumMatches::Zero | NumMatches::One => ArgMatch::NoMatch,
            NumMatches::Two => {
                let len = a.size() + b.size();
                if s.len() == len {
                    // only match if it's exact (e.g. don't match "--longfoo")
                    ArgMatch::Match
                } else {
                    ArgMatch::NoMatch
                }
            },
            NumMatches::Three => {
                let len = a.size() + b.size() + c.size();
                ArgMatch::Contains(&s[len..])
            }
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
    pub fn required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }

}
