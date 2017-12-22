
use arg::ArgType;
use arg::err::ArgResult;

struct CountArg {
    max: Option<u8>,
}

impl ArgType for CountArg {
    type Contents = u8;
    fn extract(&self, args: &mut Vec<Option<String>>) -> ArgResult<u8> {
        unimplemented!()
    }
}

