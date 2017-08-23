extern crate yaap;
use yaap::{Yaap, Arg};
use std::env;

#[derive(Debug, Default)]
struct Args {
    a: u8,
    b: bool,
    c: String,

    x: bool,
    y: bool,
    z: usize,
}

fn main() {
    //let mut args: Args;// = Args { 0, false, String::new, false, false, 0 };
    let mut args = Args::default();

    Yaap::create(env::args())
        //.build()
        //.contains(&mut args.x, Arg::<FlagArg>::new("x", "the exes"))
        .contains(&mut args.x, Arg::from("x", "the exes"))
        .count(&mut args.z, Arg::from("omega", "zzz top")
               .with_short('z')
               )
        //.contains(&mut args.x, Arg::from("x", "the exes"))
        .extract_val(&mut args.b, Arg::from("bool", "a boolean"))
        .finish()
        ;

    println!("x: {:?}", args.x);
    println!("z: {:?}", args.z);
    println!("b: {:?}", args.b);
}
