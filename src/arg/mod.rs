
mod err;
mod arg_s;
mod types;

pub(crate) use self::types::flag::FlagArg;
pub(crate) use self::types::count::CountArg;
pub(crate) use self::types::val::ValArg;
pub(crate) use self::arg_s::ArgS;
pub(crate) use self::err::ArgError;
use self::err::ArgResult;

/// Argument matcher: contains usage data
pub struct ArgM<T: ArgType> {
    help: &'static str,
    long: &'static str,
    short: Option<char>, // str? generic?
    kind: T,
}

impl<T: ArgType> ArgM<T> {
    fn from(long: &'static str, help: &'static str) -> Self {
        ArgM {
            help, long,
            short: None,
            kind: T::default(),
            //kind: T::empty(),
        }
    }
    pub fn new(long: &'static str, help: &'static str) -> Self {
        ArgM {
            help, long, short: None, kind: T::empty()
        }
    }
    /// Supply a short (`char`) version that is used with one hyphen (e.g. `-v`)
    pub fn with_short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
    /// Wrapper so ArgType::extract can be called on ArgM
    pub(crate) fn extract(&mut self, args: &mut Vec<ArgS>) -> ArgResult<<T as ArgType>::Contents> {
        T::extract(self, args)
    }
    /// Determine if this matches a user-supplied string
    pub(crate) fn matches<'a>(&self, arg_s: &'a str) -> ArgMatch<'a> {
        match self.matches_short(arg_s) {
            ArgMatch::NoMatch => self.matches_long(arg_s),
            other => other,
        }
    }
    fn matches_short<'a>(&self, arg_s: &'a str) -> ArgMatch<'a> {
        let short = match self.short {
            Some(c) => c,
            None => return ArgMatch::NoMatch
        };
        let mut chars = arg_s.chars();
        if (chars.next(), chars.next()) == (Some('-'), Some(short)) {
            match chars.next() {
                None => ArgMatch::Match,
                // TODO: will this fuck up on non-unicode chars? I think so
                Some('=') => ArgMatch::Contains(&arg_s[3..]),
                _ => ArgMatch::NoMatch,
            }
        } else {
            ArgMatch::NoMatch
        }
    }
    fn matches_long<'a>(&self, arg_s: &'a str) -> ArgMatch<'a> {
        if arg_s.starts_with("--") && arg_s[2..].starts_with(self.long) {
            // TODO: indexing like this might fuck up if arg is unicode
            let rest = &arg_s[2+self.long.len()..];
            match rest.chars().next() {
                None => ArgMatch::Match,
                Some('=') => ArgMatch::Contains(&rest[1..]),
                _ => ArgMatch::NoMatch,
            }
        } else {
            ArgMatch::NoMatch
        }
    }
}

/// Different kinds of argument matchers (e.g. flag or value)
pub trait ArgType: Default + Sized {
    /// The return type of a match (e.g. boolean for a flag)
    type Contents; // better name?

    /// Extract Contents from ArgS list, invalidate used ArgSs
    /// If an error is returned, no guarantees are made about the state of `args`
    fn extract(argm: &mut ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Self::Contents>;

    fn empty() -> Self {
        Self::default()
    }
}

/// How an ArgS can match an ArgM
#[derive(Debug, PartialEq)]
pub(crate) enum ArgMatch<'a> {
    // irrelevant
    NoMatch,
    // exact match, e.g. `--foo`
    Match,
    // contained match, e.g. `--foo=bar`
    Contains(&'a str),
}

mod test {
    // misc tests / helpers
    use arg::arg_s::ArgS;

    pub(crate) fn own(s: &str) -> Vec<ArgS> {
        s.split(' ').map(|s| ArgS::from(s.to_string())).collect()
    }
}
