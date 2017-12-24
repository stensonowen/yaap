
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

impl<'a> ArgMatch<'a> {
    pub(crate) fn or_else<F: Fn()->ArgMatch<'a>>(self, f: F) -> ArgMatch<'a> {
        match self {
            ArgMatch::NoMatch => f(),
            other => other,
        }
    }
}

/// User-supplied argument string (e.g. "--help")
#[derive(Debug)]
//pub(crate) struct ArgS(String);
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
    // uh the lifetimes can be elided but is that clearer?
    fn matches_short<'a>(&'a  self, short: char) -> ArgMatch<'a> {
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

    pub(crate) fn matches_short_opt<'a>(&'a  self, short: Option<char>) -> ArgMatch<'a> {
        if let Some(c) = short {
            self.matches_short(c)
        } else {
            ArgMatch::NoMatch
        }
    }

    pub(crate) fn matches_long<'a>(&'a  self, long: &str) -> ArgMatch<'a> {
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
}


