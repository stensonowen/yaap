
use std::env;
use arg;

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

    name: String,
    //auth: Option<&'static str>,
    desc: Option<&'static str>,
    //vers: Option<&'static str>,
    help: Option<&'static str>,

    //args: Vec<Arg>,
    state: T,
}


impl Yaap<YaapOpts> {
    pub fn create(mut argv: env::Args) -> Yaap<YaapOpts> {
        let name = argv.nth(0).unwrap(); // TODO
        let argv: Vec<_> = argv.collect();
        let free = argv.iter().map(|_| false).collect();
        Yaap {
            argv, free, name,
            auth: None,
            desc: None,
            vers: None,
            help: None,
            state: YaapOpts
        }
    }
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


}

impl Yaap<YaapArgs> { 
    /*
    pub fn count(self, result: &mut usize, arg: Arg<CountArg>) -> Self {
        let mut count = 0;
        for s in &self.argv {
            count += arg.matches(s);
        }
        *result = count;
        self
    }

    pub fn contains(self, result: &mut bool, arg: Arg<FlagArg>) -> Self {
        *result = false;
        for s in &self.argv {
            if arg.matches(s) {
                *result = true;
                break;
            }
        }
        self
    }

    pub fn extract_val<T>(self, result: &mut T, arg: Arg<ValArg>) -> Self 
        where T: FromStr
    {
        unimplemented!()
    }

    pub fn extract_list<T>(self, result: &mut Vec<T>, arg: Arg<ListArg>) -> Self
        where T: FromStr
    {
        unimplemented!()
    }
    */
}

impl Yaap<YaapDone> { }
