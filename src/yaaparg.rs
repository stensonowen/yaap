// YaapArg impl and impl for all FromStr impls


use std::str::FromStr;
use std::fmt::Debug;

// the trait a struct/enum must implement to be coerced via a Val/List
pub trait YaapArg: FromStr + Debug {
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
// signed
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

