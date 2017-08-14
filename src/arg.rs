
pub enum NumArgs {
    Zero,
    Exactly(usize),
    Unlimited,
}


pub struct Arg {
    long: &'static str,
    short: Option<char>,
    required: bool,
    help: &'static str,

    // use `count`
    repeatable: bool,

    // use `extract`
    num_args: NumArgs,

    // requires: Vec<Arg|String>
}

impl Arg {
    pub fn new(long: &'static str, help: &'static str) -> Self { // `from`
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

    pub(super) fn matches<'a>(&self, s: &'a str) -> ArgMatch<'a> {
        // check for short
        match self.short {
            Some(c) if s.starts_with(&['-',c,'='][..]) => 
                return ArgMatch::Contained(&s[3..]),
            Some(c) if s.starts_with(&['-',c][..]) && s.len() == 2 => 
                return ArgMatch::Match,
            _ => {},
        }
        // check for long
        if s.starts_with("--") && s[2..].starts_with(self.long) {
            if s.len() == 2 + self.long.len() {
                ArgMatch::Match
            } else if let Some('=') = s.chars().nth(2 + self.long.len()) {
                ArgMatch::Contained(&s[self.long.len()+3..])
            } else {
                ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArgMatch<'a> {
    Match,                  // `-c`, `--long`, etc.
    NoMatch,                // <not a valid match>
    Contained(&'a str)      // `-c=XX`, `--long=XX`
}

