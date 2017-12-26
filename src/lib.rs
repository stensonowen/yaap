//! A safe and extensible arg-parsing framework
#![allow(dead_code)] // TODO remove

use std::fmt::Debug;
use std::str::FromStr;

mod arg;
mod yaap;
mod impls;

pub use arg::ArgM;
pub use yaap::Yaap;

/// Trait required for an object to be derivable from arguments
pub trait YaapArg: Debug + FromStr {
    // associated const would be preferable but is unstable
    /// Name of the type to be used in help messages (e.g. `u8` or `IpAddr`)
    fn type_name() -> &'static str;
}

/* Don't forget TODO
 *  Subcommands (impl main functionality via wrappers around this?)
 *  DashArg? `vim -`?
 *  Use OsStr optionally? Don't require FromStr?
 *      Currently can't parse bad utf-8 `Path`s as args :/
 *      `impl<T: From<[u8]> YaapArg for T`? Not ideal
 *      Possible to do this iff FromStr unimplemented?
 *
 *
 */

