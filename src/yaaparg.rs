// YaapArg impl and impl for all FromStr impls


use std::str::FromStr;
// the trait a struct/enum must implement to be coerced via a Val/List
pub trait YaapArg : FromStr {
    // an associated constant might be handy
    fn type_name() -> &'static str;
}

/*
 * from https://doc.rust-lang.org/std/str/trait.FromStr.html
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

impl YaapArg for bool  { fn type_name() -> &'static str { "bool"  } }
impl YaapArg for char  { fn type_name() -> &'static str { "char"  } }

impl YaapArg for f32   { fn type_name() -> &'static str { "f32"   } }
impl YaapArg for f64   { fn type_name() -> &'static str { "f64"   } }

impl YaapArg for  u8   { fn type_name() -> &'static str { "u8"    } }
impl YaapArg for u16   { fn type_name() -> &'static str { "u16"   } }
impl YaapArg for u32   { fn type_name() -> &'static str { "u32"   } }
impl YaapArg for u64   { fn type_name() -> &'static str { "u64"   } }
impl YaapArg for usize { fn type_name() -> &'static str { "usize" } }
// TODO u128? 

impl YaapArg for  i8   { fn type_name() -> &'static str { "i8"    } }
impl YaapArg for i16   { fn type_name() -> &'static str { "i16"   } }
impl YaapArg for i32   { fn type_name() -> &'static str { "i32"   } }
impl YaapArg for i64   { fn type_name() -> &'static str { "i64"   } }
impl YaapArg for isize { fn type_name() -> &'static str { "isize" } }
// TODO i128? 

//impl YaapArg for { fn type_name() -> &'static str { ""  } }
impl YaapArg for String { fn type_name() -> &'static str { "String"  } }

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
impl YaapArg for IpAddr { fn type_name() -> &'static str { "IpAddr"  } }
impl YaapArg for Ipv4Addr { fn type_name() -> &'static str { "Ipv4Addr"  } }
impl YaapArg for Ipv6Addr { fn type_name() -> &'static str { "Ipv6Addr"  } }

use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
impl YaapArg for SocketAddr { fn type_name() -> &'static str { "SocketAddr"  } }
impl YaapArg for SocketAddrV4 { fn type_name() -> &'static str { "SocketAddrV4"  } }
impl YaapArg for SocketAddrV6 { fn type_name() -> &'static str { "SocketAddrV6"  } }

// for proc_macro
//impl YaapArg for TokenStream { fn type_name() -> &'static str { "TokenStream"  } }

