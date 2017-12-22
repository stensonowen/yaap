
use YaapArg;

mod err;
mod arg_s;
mod types;
use self::types::flag;
//mod flag;
//mod count;

use self::err::ArgResult;

/// Argument matcher: contains usage data
struct ArgM<T: ArgType> {
    long: &'static str,
    short: char, // str? generic?
    kind: T,
}

/// Different kinds of argument matchers (e.g. flag or value)
trait ArgType {
    /// The return type of a match (e.g. boolean for a flag)
    type Contents; // better name?

    /// Extract Contents from ArgS list, invalidate used ArgSs
    fn extract(&self, args: &mut Vec<Option<String>>) -> ArgResult<Self::Contents>;
}

