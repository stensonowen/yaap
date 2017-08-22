
#[derive(Debug)]
pub enum ArgError {
    UnexpectedValue,
    BadType,
}

pub type ArgResult<T> = Result<T, ArgError>;

