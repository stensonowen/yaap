//! A safe and extensible arg-parsing framework
#![allow(dead_code)]

use std::fmt::Debug;
use std::str::FromStr;

mod arg;
mod yaap;
mod impls;

pub use arg::ArgM;
pub use yaap::Yaap;
pub use arg::FlagArg; // TODO remove

/// Trait required for an object to be derivable from arguments
pub trait YaapArg: Debug + FromStr {
    // associated const would be preferable but is unstable
    /// Name of the type to be used in help messages (e.g. `u8` or `IpAddr`)
    fn type_name() -> &'static str;
}


/*
fn foo() {
    let mut x = false;
    Yaap::create()
        .transit()
        //.get_flag(&mut args.x, ArgM::from("x", "the exes"))
        //.get_flag(&mut args.x, ArgM::from::<FlagArg>("x", "the exes"))
        .get_flag(&mut x, ArgM::<FlagArg>::from_("x", "the exes"))
        ;
        //.finish();
}
*/
