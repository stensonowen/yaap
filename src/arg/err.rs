
use std::fmt;
use std::error::Error;

pub type ArgResult<T> = Result<T, ArgError>;

// should pretty much all ArgErrors contain the type information?
// that might be helpful?

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ArgError {
    UnexpectedValue {
        // flag given a value, e.g. `-v=9`
        long: &'static str,
        attempt: String,
    },
    MissingValue {
        // last argument expected a value
        long: &'static str,
    },
    BadType {
        // e.g. `--num false`
        long: &'static str,
        exp_type: &'static str,
        attempt: String,
    },
    BadTypeFree {
        // if a free arg is the wrong type
        exp_type: &'static str,
        attempt: String,
    },
    MissingArg {
        // required argument omitted
        long: &'static str,
    },
    Repetition {
        // argument set multiple times unexpectedly
        long: &'static str,
    }
}

impl Error for ArgError {
    fn description(&self) -> &str {
        use self::ArgError::*;
        match *self {
            UnexpectedValue{..} => "Flags cannot have values",
            MissingValue{..}    => "Values and Lists require at least one value",
            MissingArg{..}      => "A required argument was omitted",
            BadType{..}         => "Types must be properly derivable via FromStr",
            BadTypeFree{..}     => "Free args must be of the proper type",
            Repetition{..}      => "Arguments cannot be defined multiple times",
        }
    }
    // I don't think `cause` should be implemented (?)
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ArgError::*;
        match *self {
            UnexpectedValue { long, ref attempt } =>
                write!(f, "Flag `{}` cannot be set to a value (as in `{}`)",
                       long, attempt),
            MissingValue { long } =>
                write!(f, "Option `{}` requires a value", long),
            BadType { long, exp_type, ref attempt } =>
                write!(f, "Option `{}` expected type `{}` (couldn't convert `{}`)",
                       long, exp_type, attempt),
            BadTypeFree { exp_type, ref attempt } =>
                write!(f, "Bad free arg type; expected `{}`, couldn't convert `{}`",
                       exp_type, attempt),
            MissingArg { long } =>
                write!(f, "Missing value for argument `{}`", long),
            Repetition { long } =>
                write!(f, "Option `{}` cannot be set more than once", long),
        }
    }
}

