extern crate yaap;
use yaap::{Yaap, Arg};

#[derive(Debug, Default)]
struct Args {
    a: u8,
    b: bool,
    c: String,

    x: bool,
    y: bool,
    z: usize,
}

use yaap::arg::FlagArg;
fn main() {
    //let mut args: Args;// = Args { 0, false, String::new, false, false, 0 };
    let mut args = Args::default();

    Yaap::new()
        .build()
        //.contains(&mut args.x, Arg::<FlagArg>::new("x", "the exes"))
        .contains(&mut args.x, Arg::from("x", "the exes"))
        //.contains(&mut args.x, Arg::from("x", "the exes"))
        ;

    println!("x: {:?}", args.x);
}
