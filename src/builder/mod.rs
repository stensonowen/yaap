
use super::{Arg, ArgError};
use std::{mem, env};
use std::str::FromStr;

mod flag;
mod count;
mod val;
mod list;


pub trait BuilderState {}

#[must_use] pub struct YaapOpts; 
#[must_use] pub struct YaapArgs; 
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
        panic!("You probably forgot `.finish()`");
    }
}

impl Drop for YaapArgs {
    fn drop(&mut self) {
        panic!("You probably forgot `.finish()`");
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

    pub fn create_from(name: String, argv: Vec<String>) -> Yaap<YaapOpts> {
        let free = argv.iter().map(|a| !a.starts_with('-')).collect();
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

    pub fn create() -> Yaap<YaapOpts> {
        let mut args = env::args();
        let name = args.nth(0).unwrap(); // TODO ?
        let argv = args.collect();
        Self::create_from(name, argv)
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

    pub fn collect_free_args<T>(self, result: &mut Vec<T>) -> Yaap<YaapDone> 
        where T: FromStr
    {
        let new: Yaap<YaapArgs> = self.into();
        new.collect_free_args(result)
    }

    pub fn finish(self) -> Yaap<YaapDone> {
        let new: Yaap<YaapArgs> = self.into();
        new.finish()
    }
}

impl Yaap<YaapArgs> { 
    pub fn collect_free_args<T>(mut self, result: &mut Vec<T>) -> Yaap<YaapDone> 
        where T: FromStr
    {
        // TODO: maybe make `argv` a field of YaapOpts/YaapArgs or something
        // that way it wouldn't be present in `Yaap<YaapDone>`, which would 
        //  mean these free args don't need to be cloned
        // ehhh, not particularly great either way
        let mut free = vec![];
        let mut rest_are_free = false;
        for (arg, &is_free) in self.argv.iter().zip(self.free.iter()) {
            if rest_are_free || is_free {
                match arg.parse() {
                    Ok(t) => free.push(t),
                    Err(_) => self.errs.push(ArgError::BadType),
                }
            } else if arg == "--" {
                rest_are_free = true;
            }
        }
        *result = free;
        self.into()
    }

    pub fn finish(self) -> Yaap<YaapDone> {
        let new: Yaap<YaapDone> = self.into();
        new.finish()
    }
}

impl Yaap<YaapDone> {
    pub fn finish(self) -> Yaap<YaapDone> {
        if !self.errs.is_empty() {
            panic!("Errors: {:?}", self.errs);
        } else {
            self
        }
    }

    // TODO: getters
    // in case someone wants to see the help message or metadata / args / something
}
