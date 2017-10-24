extern crate yaap;
use yaap::{Yaap, Arg};
use yaap::arg::FlagArg;

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

    Yaap::new()
        .build()
        .contains(&mut args.x, Arg::<FlagArg>::from("x", "the exes"))
        .contains(&mut args.y, Arg::from("y", "trutho or falso or none of the above"))
        .count(&mut args.z, Arg::from("z", "zzzzzzzzz").with_short('z'))
        ;

    println!("x: {:?}", args.x);
    println!("z: {:?}", args.z);
}


