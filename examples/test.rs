extern crate yaap;
use yaap::{Yaap, Arg};
use std::env;

#[derive(Debug, Default)]
struct Args {
    a: u8,
    b: bool,
    c: String,

    w: Vec<i32>,
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
        //.extract_val(&mut args.a, Args::from("alpha", "first").with_short('a').with_default(42))
        .extract_val(&mut args.a, Arg::from("alpha", "first").with_default(42))
        .extract_val(&mut args.b, Arg::from("bool", "a boolean"))
        //.extract_list(&mut args.w, Arg::from("nums", "nummms"))
        .finish()
        ;

    //println!("w: {:?}", args.w);
    //println!("x: {:?}", args.x);
    //println!("z: {:?}", args.z);
    //println!("b: {:?}", args.b);
    println!("{:?}", args);
}
