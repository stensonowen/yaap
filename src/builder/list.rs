use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgMatch2, ArgError};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct ListArg { pub(super) len: Option<usize> }

impl ArgTrait for ListArg {
    type MatchType = ArgMatch2;

    fn matches(arg: &Arg<Self>, s: &str) -> ArgMatch2 {
        match arg.short_matches(s) {
            ArgMatch2::NoMatch => arg.long_matches(s),
            sm => sm
        }
    }
}

impl Arg<ListArg> {
    pub fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}

impl Yaap<YaapOpts> {

    pub fn extract_list<T>(self, result: &mut Vec<T>, arg: Arg<ListArg>) 
        -> Yaap<YaapArgs>
        where T: FromStr
    {
        let new: Yaap<YaapArgs> = self.into();
        new.extract_list(result, arg)
    }

}

impl Yaap<YaapArgs> {

    pub fn extract_list<T>(mut self, result: &mut Vec<T>, arg: Arg<ListArg>)
        -> Self
        where T: FromStr
    {
        let mut res_vec = vec![];
        for (i,a) in self.argv.iter().enumerate() {
            let matches = arg.matches(a);
            if matches == ArgMatch2::NextArg {
                // `--list 1, 2, 3, 4`
                if let Some(next_args) = self.argv.get(i+1..) {
                    for (j, elem) in next_args.iter()
                        .take_while(|e| !e.starts_with('-')).enumerate()
                    {
                        self.free[j] = false;
                        match elem.parse() {
                            Ok(e) => res_vec.push(e),
                            Err(_) => self.errs.push(ArgError::BadType {
                                long: arg.long, attempt: elem.to_owned()
                            }),
                            // TODO: preserve type of `_`?
                        }
                    }
                } else {
                    self.errs.push(ArgError::MissingArg { long: arg.long });
                }
            } else if let ArgMatch2::AtOffset(j) = matches {
                // `--list=1,2,3,4`
                self.free[i] = false;
                for elem in a[j..].split(',') {
                    match elem.parse() {
                        Ok(e) => res_vec.push(e),
                        Err(_) => self.errs.push(ArgError::BadType {
                            long: arg.long, attempt: elem.to_owned(),
                        }),
                        // TODO: preserve type of `_`?
                    }
                }
            } 
        }
        *result = res_vec;
        self.args.push(arg.strip_type());
        self
    }

}
