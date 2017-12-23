
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
pub(crate) struct ArgS(String);

impl ArgS {
    pub(crate) fn from(s: &str) -> Self {
        ArgS(s.to_string())
    }
    // uh the lifetimes can be elided but is that clearer?
    fn matches_short<'a>(&'a self, short: char) -> ArgMatch<'a> {
        let mut chars = self.0.chars();
        if (chars.next(), chars.next()) == (Some('-'), Some(short)) {
            match chars.next() {
                None => ArgMatch::Match,
                Some('=') => ArgMatch::Contains(&self.0[3..]),
                // invalid state; error wil be caught somewhere else
                _ => ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }

    pub(crate) fn matches_short_opt(&self, short: Option<char>) -> ArgMatch {
        if let Some(c) = short {
            self.matches_short(c)
        } else {
            ArgMatch::NoMatch
        }
    }

    pub(crate) fn matches_long<'a>(&'a self, long: &str) -> ArgMatch<'a> {
        let this = &self.0;
        if this.starts_with("--") && this[2..].starts_with(long) {
            // indexing like this might fuck up if long options are unicode?
            let end = &self.0[2+long.len()..];
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


