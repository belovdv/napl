pub trait Set {
    fn as_dict(&self) -> Option<&dyn Dict> {
        self.as_sequence().map(|s| s as &dyn Dict)
    }
    fn as_sequence(&self) -> Option<&dyn Sequence> {
        None
    }
    fn is_finite(&self) -> bool {
        self.get_card().is_some()
    }
    fn get_card(&self) -> Option<usize> {
        None
    }
}

pub trait Dict: Set {
    fn key(&self) {}
    fn value(&self) {}
    fn get(&self) {}
}
pub trait Sequence<'a>: Dict {
    fn iter(&'a self) -> Box<dyn Iterator<Item = &dyn Set> + 'a>;
}


impl<'a, T> Dict for T where T: Sequence<'a> {}

// To be done: proc macro.

pub mod imp {
    use super::*;

    pub struct Unit; // Supportive.
    impl Set for Unit {
        fn get_card(&self) -> Option<usize> {
            Some(1)
        }
    }
    static UNIT: Unit = Unit;

    pub struct All; // All sets, supportive.
    impl Set for All {}

    pub struct Bool;
    impl Set for Bool {
        fn get_card(&self) -> Option<usize> {
            Some(2)
        }
    }

    pub struct Range(usize);
    impl Set for Range {
        fn get_card(&self) -> Option<usize> {
            Some(self.0)
        }
        fn as_sequence(&self) -> Option<&dyn Sequence> {
            Some(self)
        }
    }
    impl<'a> Sequence<'a> for Range {
        fn iter(&self) -> Box<dyn Iterator<Item = &dyn Set>> {
            Box::new([0..self.0].into_iter().map(|_| &UNIT as &dyn Set))
        }
    }

    pub struct Unsigned;
    impl Set for Unsigned {
        fn as_sequence(&self) -> Option<&dyn Sequence> {
            Some(self)
        }
    }
    impl<'a> Sequence<'a> for Unsigned {
        fn iter(&self) -> Box<dyn Iterator<Item = &dyn Set>> {
            Box::new([0..].into_iter().map(|_| &UNIT as &dyn Set))
        }
    }

    pub struct Integer;
    impl Set for Integer {
        fn as_dict(&self) -> Option<&dyn Dict> {
            Some(self)
        }
    }
    impl Dict for Integer {}

    pub struct Float;
    impl Set for Float {}

    pub struct Sum(Vec<Box<dyn Set>>);
    impl Set for Sum {
        fn is_finite(&self) -> bool {
            self.0.iter().all(|s| s.is_finite())
        }
        fn get_card(&self) -> Option<usize> {
            if !self.is_finite() {
                return None;
            }
            Some(self.0.iter().filter_map(|s| s.get_card()).sum())
        }
    }

    pub struct Product(Vec<Box<dyn Set>>);
    impl Set for Product {
        fn is_finite(&self) -> bool {
            self.0.iter().all(|s| s.is_finite())
        }
        fn get_card(&self) -> Option<usize> {
            if !self.is_finite() {
                return None;
            }
            Some(self.0.iter().filter_map(|s| s.get_card()).product())
        }
        fn as_sequence(&self) -> Option<&dyn Sequence> {
            Some(self)
        }
    }
    impl<'a> Sequence<'a> for Product {
        fn iter(&'a self) -> Box<dyn Iterator<Item = &dyn Set> + 'a> {
            Box::new(self.0.iter().map(|b| b.as_ref()))
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
