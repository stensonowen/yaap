
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
#[derive(Debug)] pub struct YaapDone;

impl BuilderState for YaapOpts {}
impl BuilderState for YaapArgs {}
impl BuilderState for YaapDone {}

#[derive(Debug)]
pub struct Yaap<T: BuilderState> {
    argv: Vec<String>,
    free: Vec<bool>,
    errs: Vec<ArgError>,
    args: Vec<Arg<()>>,

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
            args: old.args,
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
            args: old.args,
            name: old.name,
            auth: old.auth,
            desc: old.desc,
            vers: old.vers,
            help: old.help,
            state: YaapDone,
        }
    }
}

// hack to shim args into a struct
// otherwise end up with like `Vec<Box<ArgTrait<MatchType=ArgTraitTrait>>>>`
//  and I think because ValType is generic it might be more complicated
impl super::ArgTrait for () {
    type MatchType = ();
    fn matches(_: &Arg<Self>, _: &str) { unimplemented!() }
}

impl Yaap<YaapOpts> {

    pub fn create_from(name: String, argv: Vec<String>) -> Yaap<YaapOpts> {
        let free = argv.iter().map(|a| !a.starts_with('-')).collect();
        Yaap {
            argv, free, name,
            args: vec![],
            errs: vec![],
            auth: None,
            desc: None,
            vers: None,
            help: None,
            state: YaapOpts,
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
                    Err(_) => self.errs.push(ArgError::BadTypeFree {
                        attempt: arg.to_owned(),
                    }),
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
            println!("{:?}", self);
            self
        }
    }

    // TODO: getters
    // in case someone wants to see the help message or metadata / args / something

    fn usage(&self) -> String {
        if let Some(h) = self.help { 
            h.to_owned()
        } else {
            let mut s = format!("{}{}
Usage: {} [OPTIONS] [FREE ARGS ?]

Options:
",
                self.desc.unwrap_or(""), 
                if self.desc.is_some() { "\n\n" } else { "" },
                self.name, // if self.has_free
                );
            let help_arg = Arg::from("--help", "Display this message")
                .with_short('h');
            let max_arg_len = self.args.iter().fold("help".len(), |acc, arg| {
                ::std::cmp::max(acc, arg.long.len())
            });
            let max_len = "-x ".len() + "--".len() + max_arg_len + "  ".len();
            for arg in vec![help_arg].iter().chain(self.args.iter()) { 
                let mut len = 0;
                let mut arg_s = String::from("\t");
                // short
                if let Some(c) = arg.short {
                    arg_s.push('-');
                    arg_s.push(c);
                    arg_s.push(' ');
                    len += 3;
                }
                // long
                arg_s.push_str("--");
                arg_s.push_str(arg.long);
                len += 2 + arg.long.len();
                // padding
                for _ in 0 .. (max_len - len) {
                    arg_s.push(' ');
                }
                // help
                arg_s.push_str(arg.help);
                // required
                if arg.required {
                    arg_s.push_str(" (required)");
                }
                arg_s.push('\n');
                s.push_str(&arg_s);
            }
            s.push('\n');
            s
        }
    }
}
