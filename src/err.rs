use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ArgError {
    UnexpectedValue,    // flag given a value, e.g. `-v=9`
    MissingValue,       // last argument expected a value
    BadType,            // e.g. `--num false`
    MissingArg,         // required argument omitted
    Repetition,         // argument set multiple times unexpectedly
}

//pub type ArgResult<T> = Result<T, ArgError>;

#[derive(Debug)]
struct BadArg {
    long: &'static str,
    //short: Option<char>,
    attempt: String, // maybe be ref ? // only makes sense to have attempts sometimes
    problem: ArgError,
}

impl BadArg {
    //pub fn from(
}

impl fmt::Display for BadArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ArgError::*;
        match self.problem {
            UnexpectedValue => 
                write!(f, "Flag `{}` cannot be set to a value (as in `{}`)",
                       self.long, self.attempt),
            MissingValue    => 
                write!(f, "Option `{}` requires a value", self.long),
            BadType         => 
                write!(f, "Option `{}` expected type TODO (couldn't convert `{}`)",
                       self.long, self.attempt),
            MissingArg      => 
                write!(f, "Missing required option `{}`", self.long),
            Repetition      => 
                write!(f, "Option `{}` cannot be set more than once", self.long),
        }
    }
}
