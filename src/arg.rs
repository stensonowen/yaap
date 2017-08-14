
pub enum NumArgs {
    Zero,
    Exactly(usize),
    Unlimited,
}

pub(super) struct Arg {
    long: &'static str,
    short: Option<char>,
    required: bool,
    help: &'static str,

    // use `count`
    repeatable: bool,

    // use `extract`
    //takes_value: bool,
    num_args: NumArgs,

    // requires: Vec<Arg|String>
}

impl Arg {
    fn new(long: &'static str, help: &'static str) -> Self {
        Arg {
            long,
            help,
            short: None,
            required: false,
            repeatable: false,
            num_args: NumArgs::Zero,
        }
    }

    // TODO: is this the nicest way to do the builder pattern?
    // is `Arg.foo().is_required(true)` better than `Arg.foo().is_required()`?
    //  the former is more verbose. the latter makes the defaults clearer
    // I dunno. It's worth evaluating

    fn with_short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
    fn is_required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }
    fn is_repeatable(mut self, rep: bool) -> Self {
        self.repeatable = rep;
        self
    }
    fn num_args(mut self, num: NumArgs) -> Self {
        self.num_args = num;
        self
    }
}

