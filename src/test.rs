
#[cfg(test)]
mod test {
    use super::super::{Yaap, Arg};

    fn own(input: Vec<&'static str>) -> Vec<String> {
        input.into_iter().map(|i| i.to_owned()).collect()
    }

    #[test]
    fn val_bool() {
        let mut b = false;
        Yaap::create_from(String::new(), own(vec!["--b", "true"]))
            .extract_val(&mut b, Arg::from("b", "bbb"))
            .finish();

        assert!(b);
    }

    #[test]
    fn try_extract_bool() {
        let mut b = Some(false);
        Yaap::create_from(String::new(), own(vec!["--b", "true"]))
            .try_extract_val(&mut b, Arg::from("b", "bbb"))
            .finish();

        assert_eq!(b, Some(true));
    }

    #[test]
    fn list() {
        let mut v: Vec<i8> = vec![];
        Yaap::create_from(String::new(), own(vec!["--ll", "0", "--ll", "-1"]))
            .extract_list(&mut v, Arg::from("ll", "help"))
            .finish();

        assert_eq!(2, v.len());
        assert_eq!(-1i8, v.iter().sum());
    }

    #[test]
    fn flag() {
        let mut b = false;
        Yaap::create_from(String::new(), own(vec!["--b"]))
            .contains(&mut b, Arg::from("b", "bbb"))
            .finish();

        assert!(b);
    }

    #[test]
    fn count() {
        let mut c = 0;
        Yaap::create_from(String::new(), own(vec!["-ccc", "-c=2", "-c", "--c=5"]))
            .count(&mut c, Arg::from("c", "").with_short('c'))
            .finish();

        assert_eq!(c, 11);
    }

    #[test]
    fn collect_free() {
        let mut free: Vec<u8> = vec![];
        Yaap::create_from(String::new(), own("0 1 2 3 4".split(' ').collect()))
            .collect_free_args(&mut free)
            .finish();

        assert_eq!(free.len(), 5);
    }

    #[test]
    fn misc() {
        #[derive(Debug)]
        struct Args {
            a: bool,        // flag
            b: usize,       // count
            g: u64,         // val
            d: Vec<i8>,     // list
            e: Option<bool>,// 
        }
        let mut args: Args = unsafe { ::std::mem::zeroed() };
        //let input = "-a --d 1 -b=42 --d=0 --g 6969420 -d=-1"
        //let input = "-a --d 1 -b=42 --d 0 --g 6969420 --d -1"
        //let input = "--g=1234567890 --d = --d 1";
        //let input = "--g=-1 -d 0 --d 1"; // both work
        //let input = "--g=-1 -d=0 --d=1"; // both don't
        //let input = "-g=666";
        //let input = "-d=0 -g=666 --d=1";
        //let input = "--d = -d -1 -d=-2 -d=1 --d=0";
        let input = "-g=0 --d 1"; // wtf
        //let input = "-a -b=42 --g 6969420 --e false".split(' ').collect();
        Yaap::create_from(String::new(), own(input.split(' ').collect()))
            .contains(       &mut args.a, Arg::from("a", "alpha").with_short('a'))
            .count(          &mut args.b, Arg::from("b", "beta") .with_short('b'))
            .extract_val(    &mut args.g, Arg::from("g", "gamma").with_short('g'))
            .try_extract_val(&mut args.e, Arg::from("e", "epsilon"))
            .extract_list(   &mut args.d, Arg::from("d", "delta").with_short('d'))
            .finish();
        println!("args: {:?}", args);
    }

}

