use typemap::{TypeMap, Key};
use std::collections::HashMap;
use std::str::FromStr;
use super::ArgVal;

//#[derive(Debug, PartialEq)]
//struct Value(i32);

// uhhhhh fork typemap??
//impl Key for u8 { type Value = u8; }

// For now we can only implement Key for our own structs, no primitives
// If this actually ends up being promising I'll fork typemap to do that
// In the future if a user defines their own type they can impl Key
// List of FromStr types here: 
//  https://doc.rust-lang.org/std/str/trait.FromStr.html#implementors

#[derive(Debug, PartialEq)] struct MyInt(i32);
#[derive(Debug, PartialEq)] struct MyBool(bool);
#[derive(Debug, PartialEq)] struct MyFloat(f64);
#[derive(Debug, PartialEq)] struct MyString(String);

impl Key for MyInt    { type Value = HashMap<&'static str, i32>; }
impl Key for MyBool   { type Value = HashMap<&'static str, bool>; }
impl Key for MyFloat  { type Value = HashMap<&'static str, f64>; }
impl Key for MyString { type Value = HashMap<&'static str, String>; }

impl FromStr for MyInt {
    type Err = ::std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        s.parse::<i32>().map(MyInt) 
    }
}

impl FromStr for MyBool {
    type Err = ::std::str::ParseBoolError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        s.parse::<bool>().map(MyBool) 
    }
}

impl FromStr for MyFloat {
    type Err = ::std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        s.parse::<f64>().map(MyFloat) 
    }
}

impl FromStr for MyString {
    type Err = ::std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        s.parse::<String>().map(MyString) 
    }
}
/*
trait Supported {} 
impl Supported for MyInt {}
impl Supported for MyBool {}
impl Supported for MyFloat {}
impl Supported for MyString {}

impl<T: Supported> Key for T {
    type Value = T;
} 
*/


