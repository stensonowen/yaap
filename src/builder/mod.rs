
use super::{Arg, ArgError};
use std::{mem, env};
use std::str::FromStr;

mod flag;
mod count;
mod val;
mod list;


pub trait BuilderState {}

pub struct YaapOpts;
pub struct YaapArgs;
pub struct YaapDone;

impl BuilderState for YaapOpts {}
impl BuilderState for YaapArgs {}
impl BuilderState for YaapDone {}

pub struct Yaap<T: BuilderState> {
    argv: Vec<String>,
    //free: LinkedList<usize>,
    free: Vec<bool>,
    //args: Vec<Arg<()>>
    //args: Vec<Arg<Box<
    errs: Vec<ArgError>,

    // consider: only have a `desc` and a `help` member
    // have the user craft the `about` section themselves
    // but then if they want the binary name they'll have pop it off argv
    //  and then `help` and `desc` can't be &'static str
    // do this for now
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    help: Option<&'static str>,

    state: T,
}

// drop bomb: require (at runtime) that user called `.finish()` or whatever

impl Drop for YaapOpts {
    fn drop(&mut self) {
        panic!("You probably forgot the `.finish()`");
    }
}

impl Drop for YaapArgs {
    fn drop(&mut self) {
        panic!("You probably forgot the `.finish()`");
    }
}

// transitions in state machine

impl From<Yaap<YaapOpts>> for Yaap<YaapArgs> {
    fn from(old: Yaap<YaapOpts>) -> Yaap<YaapArgs> {
        mem::forget(old.state);
        Yaap {
            argv: old.argv,
            free: old.free,
            errs: old.errs,
            name: old.name,
            auth: old.auth,
            desc: old.desc,
            vers: old.vers,
            help: old.help,
            state: YaapArgs,
        }
    }
}

impl From<Yaap<YaapArgs>> for Yaap<YaapDone> {
    fn from(old: Yaap<YaapArgs>) -> Yaap<YaapDone> {
        mem::forget(old.state);
        Yaap {
            argv: old.argv,
            free: old.free,
            errs: old.errs,
            name: old.name,
            auth: old.auth,
            desc: old.desc,
            vers: old.vers,
            help: old.help,
            state: YaapDone,
        }
    }
}


impl Yaap<YaapOpts> {

    pub fn create(mut argv: env::Args) -> Yaap<YaapOpts> {
        let name = argv.nth(0).unwrap(); // TODO
        let argv: Vec<_> = argv.collect();
        let free = argv.iter().map(|a| a == "--").collect();
        Yaap {
            argv, free, name,
            errs: vec![],
            auth: None,
            desc: None,
            vers: None,
            help: None,
            state: YaapOpts
        }
    }

    // set common options

    pub fn with_name(mut self, n: String) -> Self {
        self.name = n; self
    }
    pub fn with_author(mut self, a: &'static str) -> Self {
        self.auth = Some(a); self
    }
    pub fn with_description(mut self, d: &'static str) -> Self {
        self.desc = Some(d); self
    }
    pub fn with_version(mut self, v: &'static str) -> Self {
        self.vers = Some(v); self
    }
    pub fn with_help(mut self, h: &'static str) -> Self {
        self.help = Some(h); self
    }

    // transition to Yaap<YaapDone>

    pub fn collect_free_args<T>(self, _result: &mut Vec<T>) -> Yaap<YaapArgs> 
        where T: FromStr
    {
        self.into()
    }

    pub fn finish(self) -> Yaap<YaapDone> {
        let new: Yaap<YaapArgs> = self.into();
        new.finish()
    }
}

impl Yaap<YaapArgs> { 
    pub fn finish(self) -> Yaap<YaapDone> {
        if self.errs.is_empty() {
            println!("No errors!");
        } else {
            println!("Errors: {:?}", self.errs);
        }
        self.into()
    }
}
