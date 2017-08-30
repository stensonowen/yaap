use std::fmt;

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
        attempt: String,
    },
    BadTypeFree {
        // if a free arg is the wrong type
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

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ArgError::*;
        match *self {
            UnexpectedValue { long, ref attempt } => 
                write!(f, "Flag `{}` cannot be set to a value (as in `{}`)",
                       long, attempt),
            MissingValue { long } => 
                write!(f, "Option `{}` requires a value", long),
            BadType { long, ref attempt } => 
                write!(f, "Option `{}` expected type TODO (couldn't convert `{}`)",
                       long, attempt),
            BadTypeFree { ref attempt } => 
                write!(f, "Free arg not the right type (couldn't convert `{}`)",
                       attempt),
            MissingArg { long } => 
                write!(f, "Missing required option `{}`", long),
            Repetition { long } => 
                write!(f, "Option `{}` cannot be set more than once", long),
        }
    }
}
