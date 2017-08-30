
// For now ArgTrait implementors T are just shims around Arg<T>
// Is that a good design pattern? It's verbose but it's safe
// I don't think there's a tidier way to do the same thing

pub use super::{ArgTrait, ArgMatch2};

#[derive(Debug)]
pub struct Arg<T: ArgTrait> {
    pub(crate) long: &'static str,
    pub(crate) short: Option<char>,
    required: bool,
    help: &'static str,
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

    pub(super) fn matches(&self, s: &str) -> M {
        T::matches(self, s)
    }

    pub(super) fn short_matches(&self, s: &str) -> ArgMatch2 {
        if let Some(c) = self.short {
            if s.len() == 2 && s.chars().zip(&['-',c]).all(|(a,&b)| a==b) {
                ArgMatch2::NextArg
            } else if s.len() > 2 && 
                    s.chars().zip(&['-',c,'=']).all(|(a,&b)| a==b) {
                ArgMatch2::AtOffset(3)
            } else {
                ArgMatch2::NoMatch
            }
        } else {
            ArgMatch2::NoMatch
        }
    }

    pub(super) fn long_matches(&self, s: &str) -> ArgMatch2 {
        if s.starts_with("--") && s[2..].starts_with(self.long) {
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
