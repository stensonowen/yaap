
pub mod mod_a {

    pub trait ArgTrait {
        fn from() -> Arg<Self> where Self: Sized;
    }

    pub struct FlagArg;
    pub struct ListArg;

    impl ArgTrait for FlagArg {
        fn from() -> Arg<FlagArg> {
            Arg::<FlagArg>::new()
        }
    }

    impl ArgTrait for ListArg {
        fn from() -> Arg<ListArg> {
            Arg::<ListArg>::new()
        }
    }

    pub struct Arg<T: ArgTrait> {
        kind: T,
    }

    impl Arg<FlagArg> {
        fn new() -> Self {
            Arg::default(FlagArg)
        }
    }

    impl Arg<ListArg> {
        fn new() -> Self {
            Arg::default(ListArg)
        }
    }

    impl<T: ArgTrait> Arg<T> {
        fn default(default: T) -> Arg<T> {
            Arg::<T> {
                kind: default,
            }
        }
        pub fn from() -> Arg<T> {
            T::from()
        }

    }
}


mod mod_b {
    use mod_a::Arg;
    use mod_a::{FlagArg};

    pub struct YaapBuilder { }

    impl YaapBuilder {
        fn new() -> Self {
            Self::from()
        }

        fn from() -> Self {
            YaapBuilder { }
        }

        pub fn build(self) -> Yaap {
            Yaap { }
        }

    }

    pub struct Yaap { }

    impl Yaap { 

        // ctors

        pub fn new() -> YaapBuilder {
            YaapBuilder::new()
        }

        // safer accessors 

        pub fn contains(self, arg: Arg<FlagArg>) -> Self {
            self
        }

    }
}

use mod_b::Yaap;
use mod_a::{FlagArg, ListArg, Arg};
fn main() {
    Yaap::new()
        .build()
        .contains(Arg::<FlagArg>::new())
        //.contains(&mut args.x, Arg::new("x", "the exes"))
        //.contains(&mut args.x, Arg::from("x", "the exes"))
        ;
}
