pub trait Set {
    type Item;

    fn as_dict(&self) -> Option<&dyn Dict<Item = Self::Item>> {
        self.as_sequence()
            .map(|s| s as &dyn Dict<Item = Self::Item>)
    }
    fn as_sequence(&self) -> Option<&dyn Sequence<Item = Self::Item>> {
        None
    }
    fn is_finite(&self) -> bool {
        self.get_size().is_some()
    }
    fn get_size(&self) -> Option<usize> {
        None
    }
}

pub trait Dict: Set {
    fn key(&self) {}
    fn value(&self) {}
    fn get(&self) {}
}
pub trait Sequence<'a>: Dict {
    fn iter(&'a self) -> Box<dyn Iterator<Item = &'a Self::Item> + 'a>;
}

impl<'a, T> Dict for T where T: Sequence<'a> {}

// To be done: proc macro.

pub mod imp {
    use super::*;

    pub struct Unit; // Supportive.
    impl Set for Unit {
        type Item = ();
        fn get_size(&self) -> Option<usize> {
            Some(1)
        }
    }
    static UNIT: Unit = Unit;

    pub struct Bool;
    impl Set for Bool {
        type Item = bool;
        fn get_size(&self) -> Option<usize> {
            Some(2)
        }
    }

    pub struct Range(usize);
    impl Set for Range {
        type Item = usize;
        fn get_size(&self) -> Option<usize> {
            Some(self.0)
        }
        fn as_sequence(&self) -> Option<&dyn Sequence<Item = Self::Item>> {
            Some(self)
        }
    }
    impl<'a> Sequence<'a> for Range {
        fn iter(&self) -> Box<dyn Iterator<Item = &Self::Item>> {
            Box::new((0..self.0).into_iter().map(|_| &0))
        }
    }

    pub struct Unsigned;
    impl Set for Unsigned {
        type Item = usize;
        fn as_sequence(&self) -> Option<&dyn Sequence<Item = Self::Item>> {
            Some(self)
        }
    }
    impl<'a> Sequence<'a> for Unsigned {
        fn iter(&self) -> Box<dyn Iterator<Item = &Self::Item>> {
            Box::new((0 as usize..).into_iter().map(|_| &0))
        }
    }

    pub struct Integer;
    impl Set for Integer {
        type Item = i64;
        fn as_dict(&self) -> Option<&dyn Dict<Item = Self::Item>> {
            Some(self)
        }
    }
    impl Dict for Integer {}

    pub struct Float;
    impl Set for Float {
        type Item = f32;
    }

    pub struct Sum<T: Set>(Vec<T>);
    impl<T: Set> Set for Sum<T> {
        type Item = T;
        fn is_finite(&self) -> bool {
            self.0.iter().all(|s| s.is_finite())
        }
        fn get_size(&self) -> Option<usize> {
            if !self.is_finite() {
                return None;
            }
            Some(self.0.iter().filter_map(|s| s.get_size()).sum())
        }
    }

    pub struct Product<T: Set>(Vec<T>);
    impl<'a, T: Set> Set for Product<T> {
        type Item = T;
        fn is_finite(&self) -> bool {
            self.0.iter().all(|s| s.is_finite())
        }
        fn get_size(&self) -> Option<usize> {
            if !self.is_finite() {
                return None;
            }
            Some(self.0.iter().filter_map(|s| s.get_size()).product())
        }
        fn as_sequence(&self) -> Option<&dyn Sequence<Item = Self::Item>> {
            Some(self)
        }
    }
    impl<'a, T: Set> Sequence<'a> for Product<T> {
        fn iter(&'a self) -> Box<dyn Iterator<Item = &Self::Item> + 'a> {
            Box::new(self.0.iter())
        }
    }
}

mod deprecated {
    pub enum Set {
        // Basic.
        Unit,
        Bool,
        // Numbers.
        // To be done: bigint.
        Range(usize), //< Integer [0, size).
        Unsigned,     //< Integer [0, +inf).
        Integer,      //< Integer (-inf, +inf).
        Float,        //< Real (-inf, +inf).

        // Product.
        Sum(Vec<Set>),
        Product(Vec<Set>),
        Union(Vec<Set>),
        Pow(Box<Set>, usize),
        Sequence(Box<Set>),
        FiniteSequence(Box<Set>),
        BoundedSequence(Box<Set>, usize), // ???
    }

    impl Set {
        pub fn is_dict() {}
        pub fn is_sequence() {}
        pub fn is_finite() {}
    }
}
