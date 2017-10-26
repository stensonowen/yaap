use super::{Yaap, YaapOpts, YaapArgs, Arg};
use super::super::{ArgTrait, ArgResult, ArgMatch, ArgError};
use std::str::FromStr;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct ListArg<T: FromStr + Default + Debug> { 
    pub(super) len: Option<usize>,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
pub enum ListPart<T: FromStr + Default + Debug> {
    ListElem(T),        // --list a b c d
    ListWhole(Vec<T>),  // --list a,b,c,d
    ListDone,
}

impl<T: FromStr + Default + Debug> ArgTrait for ListArg<T> {
    type MatchType = ListPart<T>;

    //fn matches(arg: &Arg<Self>, s: &str) -> Self::MatchType {
    //    unimplemented!()
    //}

    fn extract_match(arg: &Arg<Self>, s: &str) -> ArgResult<Self::MatchType> {
        // call this on each possible elem
        if s.starts_with("--") {
            // should do this? try to accomodate arg vals starting w `--`?
            // should definitely be able to parse negative numbers
            Ok(ListPart::ListDone)
        } else if s.contains(',') {
            let v: Result<Vec<T>, ArgError> = s.split(',').map(|i| 
                match i.parse::<T>() {
                    Ok(i) => Ok(i),
                    Err(_) => Err(ArgError::BadType {
                        long: arg.long, attempt: s.to_owned()
                    })
                }).collect();
            match v {
                Ok(v) => Ok(ListPart::ListWhole(v)),
                Err(e) => Err(ArgError::BadType {
                    long: arg.long, attempt: s.to_owned()
                })
            }
        } else {
            match s.parse() {
                Ok(e) => Ok(ListPart::ListElem(e)),
                Err(_) => Ok(ListPart::ListDone),
            }
        }
    }
}

impl<T: FromStr + Default + Debug> Arg<ListArg<T>> {
    pub fn with_num_args(mut self, max: Option<usize>) -> Self {
        self.kind.len = max;
        self
    }
}

impl Yaap<YaapOpts> {

    pub fn extract_list<T>(self, result: &mut Vec<T>, arg: Arg<ListArg<T>>) 
        -> Yaap<YaapArgs>
        where T: FromStr + Debug + Default
    {
        let new: Yaap<YaapArgs> = self.into();
        new.extract_list(result, arg)
    }

}

impl Yaap<YaapArgs> {

    pub fn extract_list<T>(mut self, result: &mut Vec<T>, arg: Arg<ListArg<T>>)
        -> Self
        where T: FromStr + Debug + Default
    {
        assert!(result.is_empty());
        let mut max_left = arg.kind.len.unwrap_or(self.argv.len());
        let mut match_next = false;
        assert_eq!(self.argv.len(), self.free.len());
        for (s,free) in self.argv.iter().zip(self.free.iter_mut()) {
            if max_left == 0 { 
                break 
            } else {
                max_left -= 1;
            }

            // the string we should match on, if any
            // can either be a substring of this arg or the whole thing
            let arg_str: Option<&str> = if match_next {
                // user will need to specify list arg each time
                //  e.g. `--long 0 --long 1 --long=2`
                //   (or just `--long=0,1,2,3,4`
                match_next = false; // do?
                Some(s)
            } else {
                match ListArg::does_match(&arg, s) {
                    ArgMatch::Contains(ss) => {
                        *free = true;
                        match_next = false;
                        Some(ss)
                    },
                    ArgMatch::Match => {
                        *free = true;
                        match_next = true;
                        None
                    },
                    ArgMatch::NoMatch => {
                        let err = ArgError::MissingArg { long: arg.long };
                        self.errs.push(err);
                        match_next = false;
                        None
                    },
                }
            };

            // try to parse everything and place it into the result
            if let Some(ss) = arg_str {
                match ListArg::extract_match(&arg, ss) {
                    Err(e) => self.errs.push(e),
                    Ok(ListPart::ListDone) => continue,
                    Ok(ListPart::ListElem(e)) => result.push(e),
                    Ok(ListPart::ListWhole(ref mut v)) => result.append(v),
                }
            }
        }

        self.args.push(arg.strip_type());
        self
    }

    /*
    pub fn extract_list<T>(mut self, result: &mut Vec<T>, arg: Arg<ListArg<T>>)
        -> Self
        where T: FromStr + Debug + Default
    {
        let mut res_vec = vec![];
        for (i,a) in self.argv.iter().enumerate() {
            //match Arg<ListArg<T>>::does_match(&arg, s)
            /*
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
            */
        }
        *result = res_vec;
        self.args.push(arg.strip_type());
        self
    }
    */

}
