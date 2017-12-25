
/*
/// How an ArgS can match an ArgM
#[derive(Debug, PartialEq)]
pub(crate) enum ArgMatch<'a> {
    // irrelevant
    NoMatch,
    // exact match, e.g. `--foo`
    Match,
    // contained match, e.g. `--foo=bar`
    Contains(&'a str),
}
*/

/*
impl<'a> ArgMatch<'a> {
    pub(crate) fn or_else<F: Fn()->ArgMatch<'a>>(self, f: F) -> ArgMatch<'a> {
        match self {
            ArgMatch::NoMatch => f(),
            other => other,
        }
    }
}
*/

/// User-supplied argument string (e.g. "--help")
#[derive(Debug)]
pub struct ArgS {
    pub(crate) text: String,
    pub(crate) used: bool,
}

impl ArgS {
    pub(crate) fn from(s: String) -> Self {
        ArgS {
            text: s,
            used: false,
        }
    }

    /*
    //fn matches<'a>(&'a self, long: &'static str, short: Option<char>) -> ArgMatch<'a> {
    pub(crate) fn matches(&mut self, long: &'static str, short: Option<char>) -> ArgMatch {
        match self.matches_short_opt(short) {
            ArgMatch::NoMatch => self.matches_long(long),
            other => other,
        }
    }

    pub(crate) fn matches2(&mut self, long: &'static str, short: Option<char>) -> ArgMatch {
        {
            let m = match self.matches_short_opt(short) {
                ArgMatch::NoMatch => self.matches_long(long),
                other => other,
            };
            if m != ArgMatch::NoMatch {
                return m
            }
        }
        self.used = true;
        ArgMatch::NoMatch
    }


    // uh the lifetimes can be elided but is that clearer?
    fn matches_short<'a>(&'a self, short: char) -> ArgMatch<'a> {
        let mut chars = self.text.chars();
        if (chars.next(), chars.next()) == (Some('-'), Some(short)) {
            match chars.next() {
                None => ArgMatch::Match,
                Some('=') => ArgMatch::Contains(&self.text[3..]),
                // invalid state; error wil be caught somewhere else
                _ => ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }

    pub(crate) fn matches_short_opt<'a>(&'a self, short: Option<char>) -> ArgMatch<'a> {
        if let Some(c) = short {
            self.matches_short(c)
        } else {
            ArgMatch::NoMatch
        }
    }

    pub(crate) fn matches_long<'a>(&'a self, long: &str) -> ArgMatch<'a> {
        let this = &self.text;
        if this.starts_with("--") && this[2..].starts_with(long) {
            // indexing like this might fuck up if long options are unicode?
            let end = &self.text[2+long.len()..];
            match end.chars().next() {
                None => ArgMatch::Match,
                Some('=') => ArgMatch::Contains(&end[1..]),
                _ => ArgMatch::NoMatch,
            }
        } else {
            ArgMatch::NoMatch
        }
    }
    */

    /*
    /// Determine whether an ArgM matches this ArgS; set self.used if so
    pub(crate) fn matches(&mut self, long: &'static str, short: Option<char>) -> ArgMatch {
        let mut chars = self.text.chars();
        // isn't a match for either unless it starts with a `-`
        if chars.next() != Some('-') {
            return ArgMatch::NoMatch
        }
        match chars.next() {
            // `-c...` where `c` is the short match
            Some(c) if short == Some(c) => {
                match chars.next() {
                    // e.g. `-c`
                    None => { 
                        self.used = true; 
                        ArgMatch::Match
                    },
                    // e.g. `-c=X`
                    Some('=') => { 
                        self.used = true; 
                        ArgMatch::Contains(&self.text[3..])
                    },
                    // e.g. `-cdef`
                    _ => ArgMatch::NoMatch,
                }
            },
            // `--`: look for long match
            Some('-') => {
                // `--long...`
                if self.text[2..].starts_with(long) {
                    let rest = &self.text[2+long.len()..];
                    match rest.chars().next() {
                        // `--long`
                        None => {
                            self.used = true;
                            ArgMatch::Match
                        },
                        // `--long=X`
                        Some('=') => {
                            self.used = true;
                            ArgMatch::Contains(&rest[1..])
                        },
                        // `--longggg`
                        _ => ArgMatch::NoMatch,
                    }
                } else {
                    // a long arg, but not this one
                    ArgMatch::NoMatch
                }
            },
            // a short arg, but not this one
            _ => ArgMatch::NoMatch,
        }
    }
    */
}


