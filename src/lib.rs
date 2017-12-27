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

/* Don't forget TODO these
 *  Subcommands (impl main functionality via wrappers around this?)
 *  DashArg? `vim -`?
 *      Or some option to take in stdin? `cat`?
 *  Use OsStr optionally? Don't require FromStr?
 *      Currently can't parse bad utf-8 `Path`s as args :/
 *      `impl<T: From<[u8]> YaapArg for T`? Not ideal
 *      Possible to do this iff FromStr unimplemented?
 *  Make CountArg/ListArg requirable?
 *      Can maybe not break type inference with a Requirable trait?
 *      Effect of omitting these is value- not type-level so don't really care
 *  Arg autocompletion
 *  Free arg numbering? With types and everything?
 *      Can assert first free arg is T and second is U? and extract that way?
 *  ValArg should return a T or Option<T> depending on if it's required
 *
 */

