
use YaapArg;
use arg::{ArgM, ArgType, ArgS, ArgError};
use arg::{FlagArg, CountArg, ValArg, ListArg};
use std::{env, mem};

/// State trait used for the Yaap builder pattern
pub trait BuilderState {}

/// State of Yaap builder in which options can be specified (e.g. version)
#[derive(Debug)] pub struct YaapOpts;

/// State of Yaap builder in which arguments can be extracted
#[derive(Debug)] pub struct YaapArgs;

impl BuilderState for YaapOpts {}
impl BuilderState for YaapArgs {}

/// Argument parser object: aggregate errors and store help message info
// I should be able to `#[deny(unused_must_use)]`, right?
// Making forgetting `.finish()` an error instead of a warning?
#[derive(Debug)]
#[must_use = "Remember to call `.finish()`"]
pub struct Yaap<T: BuilderState> {
    /// Arguments entered by the user
    argv: Vec<ArgS>,
    /// Errors that have been encountered so far (collect before aborting)
    errs: Vec<ArgError>,
    /// Whether or not free args are acceptable
    free: bool,

    // metadata
    name: String,
    auth: Option<&'static str>,
    desc: Option<&'static str>,
    vers: Option<&'static str>,
    help: Option<&'static str>,

    state: T,
}

/// Customize a Yaap parser, including specifying program data
impl Yaap<YaapOpts> {
    pub fn create_from(name: String, argv: Vec<String>) -> Yaap<YaapOpts> {
        let argv = argv.into_iter().map(|s| ArgS {
            text: s,
            used: false,
        }).collect();
        Yaap {
            argv, name,
            errs: vec![],
            free: false,
            auth: None,
            desc: None,
            vers: None,
            help: None,
            state: YaapOpts,
        }
    }
    pub fn create() -> Yaap<YaapOpts> {
        let mut args = env::args();
        let name = args.next().unwrap();
        let argv = args.collect();
        Self::create_from(name, argv)
    }

    // common setters

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

    // "automatically" transition to Yaap<YaapArgs>
    pub fn get_flag(self, result: &mut bool, argm: ArgM<FlagArg>) -> Yaap<YaapArgs> {
        let this: Yaap<YaapArgs> = self.into();
        this.get_flag(result, argm)
    }

    pub fn get_count(self, result: &mut u8, argm: ArgM<CountArg>) -> Yaap<YaapArgs> {
        let this: Yaap<YaapArgs> = self.into();
        this.get_count(result, argm)
    }

    pub fn get_val<T: YaapArg>(self, result: &mut T, argm: ArgM<ValArg<T>>) -> Yaap<YaapArgs> {
        let this: Yaap<YaapArgs> = self.into();
        this.get_val(result, argm)
    }

    // transition to Yaap<YaapDone>
    // TODO collect_free_args, finish

}

/// Extract data from command-line arguments
impl Yaap<YaapArgs> {
    fn get_generic<T: ArgType>(mut self, result: &mut T::Contents, mut argm: ArgM<T>) -> Self {
        match argm.extract(&mut self.argv) {
            Ok(r) => *result = r,
            Err(e) => self.errs.push(e),
        }
        self
    }
    pub fn get_flag(self, result: &mut bool, argm: ArgM<FlagArg>) -> Self {
        self.get_generic(result, argm)
    }
    pub fn get_count(self, result: &mut u8, argm: ArgM<CountArg>) -> Self {
        self.get_generic(result, argm)
    }
    pub fn get_val<T: YaapArg>(mut self, result: &mut T, mut argm: ArgM<ValArg<T>>) -> Self {
        match argm.extract(&mut self.argv) {
            Ok(Some(r)) => *result = r,
            Ok(None) => {},
            Err(e) => self.errs.push(e),
        }
        self
    }
    pub fn get_list<T: YaapArg>(self, result: &mut Vec<T>, argm: ArgM<ListArg<T>>) -> Self {
        self.get_generic(result, argm)
    }
    pub fn finish(self) -> () {
        mem::forget(self.state);
        ()
    }

}

// boring bookkeeping: drop bomb and state transitions

// drop bomb: require (at runtime) that user called `.finish()`

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

impl From<Yaap<YaapOpts>> for Yaap<YaapArgs> {
    fn from(old: Yaap<YaapOpts>) -> Yaap<YaapArgs> {
        mem::forget(old.state);
        Yaap {
            argv: old.argv,
            errs: old.errs,
            free: old.free,
            name: old.name,
            auth: old.auth,
            desc: old.desc,
            vers: old.vers,
            help: old.help,
            state: YaapArgs
        }
    }
}

