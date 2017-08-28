
#[derive(Debug, PartialEq)]
pub enum ArgError {
    UnexpectedValue,    // flag given a value, e.g. `-v=9`
    MissingValue,       // last argument expected a value
    BadType,            // e.g. `--num false`
    MissingArg,         // required argument omitted
    Repetition,         // argument set multiple times unexpectedly
}

pub type ArgResult<T> = Result<T, ArgError>;

