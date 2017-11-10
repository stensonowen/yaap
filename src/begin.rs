
use std::str::Chars;

pub trait Begins : Copy {
    fn begins(&self, chars: &mut Chars) -> bool;
    fn size(&self) -> usize;
}

impl Begins for char {
    fn begins(&self, chars: &mut Chars) -> bool { 
        Some(*self) == chars.next() 
    }
    fn size(&self) -> usize { 
        1 
    }
}

impl<'a> Begins for &'a str {
    fn begins(&self, c: &mut Chars) -> bool {
        c.as_str().len()>=self.len() && self.chars().zip(c).all(|(a,b)| a==b)
    }
    fn size(&self) -> usize { 
        self.len() 
    }
}

#[derive(Clone, Copy)]
struct Endl;
impl Begins for Endl {
    fn begins(&self, c: &mut Chars) -> bool {
        c.next().is_none()
    }
    fn size(&self) -> usize {
        0
    }
}

#[derive(Debug, PartialEq)]
pub enum NumMatches {
    Zero,
    One,
    Two,
    Three,
}

pub trait BeginsWith {
    // does what `str::starts_with` seems like it does
    // easy to continue if so desired
    fn begins_with_1<A: Begins>(&self, a: A) -> bool;
    fn begins_with_2<A: Begins, B: Begins>(&self, a: A, b: B) -> bool;
    fn begins_with_3<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C) 
        -> bool;
    fn begins_with_n<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C) 
        -> NumMatches;
}

impl BeginsWith for str {
    fn begins_with_1<A: Begins>(&self, a: A) -> bool {
        let mut chars = self.chars();
        a.begins(&mut chars)
    }
    fn begins_with_2<A: Begins, B: Begins>(&self, a: A, b: B) -> bool {
        let mut chars = self.chars();
        a.begins(&mut chars) && b.begins(&mut chars)
    }
    fn begins_with_3<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C)
        -> bool
    {
        let mut chars = self.chars();
        a.begins(&mut chars) && b.begins(&mut chars) && c.begins(&mut chars)
    }
    fn begins_with_n<A: Begins, B: Begins, C: Begins>(&self, a: A, b: B, c: C)
        -> NumMatches 
    {
        // not a great way to generalize this without a macro and boxed trait
        let mut chars = self.chars();
        if a.begins(&mut chars) == false {
            NumMatches::Zero
        } else if b.begins(&mut chars) == false {
            NumMatches::One
        } else if c.begins(&mut chars) == false {
            NumMatches::Two
        } else {
            NumMatches::Three
        }
    }
}

