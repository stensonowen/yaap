// YaapArg impl and impl for all FromStr impls


use std::str::FromStr;
use std::fmt::Debug;

// the trait a struct/enum must implement to be coerced via a Val/List
pub trait YaapArg : FromStr + Debug {
    // an associated constant might be handy
    fn type_name() -> &'static str;
}

// implement YaapArg for all types that FromStr is implemented for
// enumerated here: https://doc.rust-lang.org/std/str/trait.FromStr.html
macro_rules! impl_yaap {
    ($type:ident) => (
        impl YaapArg for $type {
            fn type_name() -> &'static str {
                stringify!($type)
            }
        }
    )
}

// misc
impl_yaap!(bool);
impl_yaap!(char);
impl_yaap!(String);
// floats
impl_yaap!(f32);
impl_yaap!(f64);
// unsigned
impl_yaap!(u8);
impl_yaap!(u16);
impl_yaap!(u32);
impl_yaap!(u64);
impl_yaap!(usize);
// unsigned
impl_yaap!(i8);
impl_yaap!(i16);
impl_yaap!(i32);
impl_yaap!(i64);
impl_yaap!(isize);
// IpAddr types
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
impl_yaap!(IpAddr);
impl_yaap!(Ipv4Addr);
impl_yaap!(Ipv6Addr);
// SocketAddr types
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
impl_yaap!(SocketAddr);
impl_yaap!(SocketAddrV4);
impl_yaap!(SocketAddrV6);
// omitted: TokenStream (for now)


/*
 *
    impl FromStr for bool  type Err = ParseBoolError;
    impl FromStr for f32  type Err = ParseFloatError;
    impl FromStr for i16  type Err = ParseIntError;
    impl FromStr for u8  type Err = ParseIntError;
    impl FromStr for u32  type Err = ParseIntError;
    impl FromStr for u64  type Err = ParseIntError;
    impl FromStr for char  type Err = ParseCharError;

    impl FromStr for i32  type Err = ParseIntError;
    impl FromStr for u16  type Err = ParseIntError;
    impl FromStr for i128  type Err = ParseIntError;
    impl FromStr for isize  type Err = ParseIntError;
    impl FromStr for u128  type Err = ParseIntError;
    impl FromStr for i8  type Err = ParseIntError;
    impl FromStr for usize  type Err = ParseIntError;
    impl FromStr for i64  type Err = ParseIntError;
    impl FromStr for f64  type Err = ParseFloatError;
    impl FromStr for String  type Err = ParseError;
    impl FromStr for IpAddr  type Err = AddrParseError;
    impl FromStr for Ipv4Addr  type Err = AddrParseError;
    impl FromStr for Ipv6Addr  type Err = AddrParseError;
    impl FromStr for SocketAddrV4  type Err = AddrParseError;
    impl FromStr for SocketAddrV6  type Err = AddrParseError;
    impl FromStr for SocketAddr  type Err = AddrParseError;
    impl FromStr for TokenStream
    */
