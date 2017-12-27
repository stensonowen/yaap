extern crate yaap;
use yaap::{Yaap, ArgM};

#[derive(Debug, Default)]
struct Args {
    y: bool,
    v: u8,
    f: f64,
}

fn main() {
    //let mut args: Args = unsafe { ::std::mem::zeroed() }; // less safe alt
    let mut args = Args::default();

    Yaap::create()
        .get_flag(&mut args.y, ArgM::new("yes", "Don't prompt").with_short('y'))
        .get_count(&mut args.v,ArgM::new("verbose", "Verbosity level").with_short('v'))
        .get_val(&mut args.f,  ArgM::new("value", "Some value")
                 .with_default(42.)
                 .required())
        .finish();

    println!("{:?}", args);
}
