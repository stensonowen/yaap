
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
    fn app1() {
        #[derive(Debug, PartialEq)]
        struct Args {
            a: bool,        // flag
            b: usize,       // count
            g: u64,         // val
            d: Vec<i8>,     // list
            e: Option<bool>,// omitted (use a `try_` fn)
        }
        let mut args: Args = unsafe { ::std::mem::zeroed() };
        let input = "-a --d 1 -b=42 --d=0 --g 6969420 -d=-1";
        Yaap::create_from(String::new(), own(input.split(' ').collect()))
            .contains(       &mut args.a, Arg::from("a", "alpha").with_short('a'))
            .count(          &mut args.b, Arg::from("b", "beta") .with_short('b'))
            .extract_val(    &mut args.g, Arg::from("g", "gamma").with_short('g'))
            .try_extract_val(&mut args.e, Arg::from("e", "epsilon"))
            .extract_list(   &mut args.d, Arg::from("d", "delta").with_short('d'))
            .finish();
        let correct = Args {
            a: true,
            b: 42, 
            g: 6969420,
            d: vec![1, 0, -1],
            e: None
        };
        assert_eq!(args, correct);
    }
    
    #[test]
    fn app1_help() {
        #[derive(Debug, PartialEq)]
        struct Args {
            a: bool,        // flag
            b: usize,       // count
            g: u64,         // val
            d: Vec<i8>,     // list
            e: Option<bool>,// omitted (use a `try_` fn)
        }
        let mut args: Args = unsafe { ::std::mem::zeroed() };
        let input = "--help";
        Yaap::create_from(String::new(), own(input.split(' ').collect()))
            .contains(       &mut args.a, Arg::from("aa",   "alpha").with_short('a'))
            .count(          &mut args.b, Arg::from("bbb",  "beta") .with_short('b'))
            .extract_val(    &mut args.g, Arg::from("gggg", "gamma").with_short('g'))
            .try_extract_val(&mut args.e, Arg::from("eee",  "epsilon"))
            .extract_list(   &mut args.d, Arg::from("dd",   "delta").with_short('d'))
            .finish();
    }

    #[test]
    fn gnu_wc() {
        #[derive(Debug, Default, PartialEq)]
        struct Args {
            bytes: bool,
            chars: bool,
            lines: bool,
            words: bool,
            max_len: bool,
            files: Vec<String>,
        }
        let mut args = Args::default();
        let input = "-c -l -L --files0-from=-";
        Yaap::create_from(String::new(), own(input.split(' ').collect()))
            .contains(&mut args.bytes, Arg::from("bytes", "print byte counts").with_short('c'))
            .contains(&mut args.chars, Arg::from("chars", "print char counts").with_short('m'))
            .contains(&mut args.lines, Arg::from("lines", "print newline counts").with_short('l'))
            .contains(&mut args.words, Arg::from("words", "print word counts").with_short('w'))
            .contains(&mut args.max_len, Arg::from("max-line-length", 
                                                   "print the maximum display width")
                      .with_short('L'))
            .extract_list(&mut args.files, Arg::from("files0-from", "read input from the files"))
            .finish();
        let correct = Args {
            bytes: true,
            chars: false,
            lines: true,
            words: false,
            max_len: true,
            files: vec![String::from("-")],
        };
        assert_eq!(args, correct);
    }

    #[test]
    fn gnu_cp() {
        let mut args: Vec<String> = vec![];
        let input = "src1 src2 src3 dst";
        Yaap::create_from(String::new(), own(input.split(' ').collect()))
            .collect_free_args(&mut args)
            .finish();
        assert_eq!(args.iter().collect::<Vec<_>>(), vec!["src1","src2","src3","dst"]);
    }

    #[test]
    #[should_panic]
    fn forgot_to_finish_1() {
        Yaap::create_from(String::new(), vec![])
            //.finish() // forgetting this is a panic!
            ;
    }

    #[test]
    #[should_panic]
    fn forgot_to_finish_2() {
        let mut b = false;
        Yaap::create_from(String::new(), vec![])
            .contains(&mut b, Arg::from("", ""))
            //.finish() // forgetting this is a panic!
            ;
    }

    #[test]
    fn weird_types() {
        use std::net::SocketAddr;
        let mut t: SocketAddr = unsafe { ::std::mem::zeroed() };
        let mut c: char = unsafe { ::std::mem::zeroed() };
        Yaap::create_from(String::new(), own(vec!["--sa=127.0.0.1:8080", "--c=0"]))
            .extract_val(&mut t, Arg::from("sa", "socket addr"))
            .extract_val(&mut c, Arg::from("c", "character"))
            .finish();
        //println!("ADDR: {:?}", t);
        //println!("CHAR: {:?}", c);
    }
}

