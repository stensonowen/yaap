
use arg::{ArgType};
use arg::err::ArgResult;

struct FlagArg;

impl ArgType for FlagArg {
    type Contents = bool;

    fn extract(&self, args: &mut Vec<Option<String>>) -> ArgResult<bool> {
        unimplemented!()
    }
}

