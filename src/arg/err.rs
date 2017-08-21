
#[derive(Debug)]
pub enum ArgError {
    UnexpectedValue,

}

pub type ArgResult<T> = Result<T, ArgError>;

