
mod err;
mod arg_s;
mod types;

use YaapArg;

pub(crate) use self::types::flag::FlagArg;
pub(crate) use self::types::count::CountArg;
pub(crate) use self::types::val::ValArg;
pub(crate) use self::types::valopt::ValOptArg;
pub(crate) use self::types::list::ListArg;
pub(crate) use self::arg_s::ArgS;
pub(crate) use self::err::ArgError;
use self::err::ArgResult;


/// Argument matcher: contains usage data
#[derive(Debug, Default)]
pub struct ArgM<T: ArgType> {
    help: &'static str,
    long: &'static str,
    short: Option<char>, // str? generic?
    kind: T,
}

// impl `new()` for everything except ValArg, which can only be created from ValOptArg
impl ArgM<FlagArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        ArgM { help, long, short: None, kind: FlagArg::default(), }
    }
}
impl ArgM<CountArg> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        ArgM { help, long, short: None, kind: CountArg::default(), }
    }
}
impl<T: YaapArg> ArgM<ValOptArg<T>> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        ArgM { help, long, short: None, kind: ValOptArg::default(), }
    }
}
impl<T: YaapArg> ArgM<ListArg<T>> {
    pub fn new(long: &'static str, help: &'static str) -> Self {
        ArgM { help, long, short: None, kind: ListArg::default(), }
    }
}

impl<T: ArgType> ArgM<T> {
    fn from(long: &'static str, help: &'static str) -> Self {
        ArgM {
            help, long,
            short: None,
            kind: T::default(),
        }
    }
    /*
    pub fn new(long: &'static str, help: &'static str) -> Self {
        ArgM {
            help, long, short: None, kind: T::default()
        }
    }
    */
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

}


/// How an `ArgS` can match an `ArgM`
#[derive(Debug, PartialEq)]
pub(crate) enum ArgMatch<'a> {
    // irrelevant
    NoMatch,
    // exact match, e.g. `--foo`
    Match,
    // contained match, e.g. `--foo=bar`
    Contains(&'a str),
}


/// Defines which Arg types can be required
// Vals, Lists, and Counts can; Flags and SubCmds cannot
pub trait Requirable: ArgType {
    fn set_required(&mut self);
}

// implement `.required()` for any ArgM which has a kind that can be required
impl<T: Requirable> ArgM<T> {
    pub fn required(mut self) -> Self {
        self.kind.set_required();
        self
    }
}



mod test {
    // misc tests / helpers
    use arg::arg_s::ArgS;

    pub(crate) fn own(s: &str) -> Vec<ArgS> {
        s.split(' ').map(|s| ArgS::from(s.to_string())).collect()
    }
}

