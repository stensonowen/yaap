
/// How an ArgS can match an ArgM
#[derive(Debug)]
enum ArgMatch<'a> {
    // irrelevant
    NoMatch,
    // exact match, e.g. `--foo`
    Match,
    // contained match, e.g. `--foo=bar`
    Contains(&'a str),
}


/// User-supplied argument string (e.g. "--help")
#[derive(Debug)]
struct ArgS(String);

impl ArgS {
    // uh the lifetimes can be elided but is that clearer?
    fn matches_short<'a>(&'a self, short: char) -> ArgMatch<'a> {
        let mut chars = self.0.chars();
        if (chars.next(), chars.next()) == (Some('-'), Some(short)) {
            match chars.next() {
                None => ArgMatch::Match,
                Some('=') => ArgMatch::Contains(&self.0[2..]),
                // invalid state; error wil be caught somewhere else
                _ => ArgMatch::NoMatch
            }
        } else {
            ArgMatch::NoMatch
        }
    }

    fn matches_long<'a>(&'a self, long: &str) -> ArgMatch<'a> {
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


