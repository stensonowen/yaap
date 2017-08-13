#![allow(dead_code)]

extern crate typemap;
use typemap::{Key, TypeMap};

use std::str::FromStr;
use std::collections::HashMap;

pub mod types;


struct Empty {}

impl FromStr for Empty { 
    type Err = ();
    fn from_str(_: &str) -> Result<Empty,()> {
        Err(())
    }
}

struct ArgVal<T: FromStr> {
    long: &'static str,
    short: Option<char>,
    required: bool,
    //requires: Vec<&Arg>,
    help: &'static str,
    value: Option<T>,
}

type Arg = ArgVal<Empty>;

impl<T: FromStr> ArgVal<T> {
    fn new() -> ArgVal<T> {
        unimplemented!()
    }
}


/*
struct _YaapBuilder {
    // like a yaap but the submap values are options
    // because they aren't necessarily satisfied yet
    args: TypeMap,
}
*/

struct _Yaap { 
    // map each possible return type to a hashmap of arguments
    //  whose key is their long name and whose value is their val
    args: TypeMap,
    //args: Vec<ArgVal<Box<FromStr<Err=()>>>>,
}

impl _Yaap {
    fn register<T: FromStr + Key>(&mut self, av: ArgVal<T>) {
        //self.args.insert::<T>(av);
        //let mut entry = self.args.entry::<T>().or_insert(0u8);
    }
}

/*
fn _main() {
    let a = ArgVal::<Empty> {
        short: None,
        long: "",
        required: false,
        help: "",
        //value: Some(0u8),
        value: None,
    };

}

fn foo<T: Key>(x: T) {}
*/
