
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

        assert!(b.is_some());
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
        Yaap::create_from(String::new(), own(vec!["-ccc", "-c=2", "--cc", "--c=5"]))
            .count(&mut c, Arg::from("c", "").with_short('c'))
            .finish();

        assert_eq!(c, 12);
    }
}
