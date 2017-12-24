
mod err;
mod arg_s;
mod types;

use self::types::flag::FlagArg;
use self::types::count::CountArg;
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
    /// Wrapper so ArgType::extract can be called on ArgM
    fn extract(&self, args: &mut Vec<ArgS>) -> ArgResult<<T as ArgType>::Contents> {
        T::extract(self, args)
    }
    fn from(long: &'static str, help: &'static str) -> Self {
        ArgM {
            help, long,
            short: None,
            kind: T::default(),
        }
    }
    fn with_short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
}

/// Different kinds of argument matchers (e.g. flag or value)
pub trait ArgType: Default + Sized {
    /// The return type of a match (e.g. boolean for a flag)
    type Contents; // better name?

    /// Extract Contents from ArgS list, invalidate used ArgSs
    /// If an error is returned, no guarantees are made about the state of `args`
    fn extract(argm: &ArgM<Self>, args: &mut Vec<ArgS>) -> ArgResult<Self::Contents>;
}

mod test {
    // misc tests / helpers
    use arg::arg_s::ArgS;

    pub(crate) fn own(s: &str) -> Vec<ArgS> {
        s.split(' ').map(|s| ArgS::from(s.to_string())).collect()
    }
}
