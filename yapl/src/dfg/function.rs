use super::r#trait;
use super::set::{self, Set};
use super::{Obj, Object};

pub trait Function {
    // fn output(&self, input: &Vec<Box<dyn Set>>) -> Option<Box<dyn Set>>;
}

pub mod imp {
    use super::*;

    pub struct Map {}
    impl Function for Map {
        // fn output(&self, input: &Vec<Box<dyn Set>>) -> Option<Box<dyn Set>> {
        //     todo!()
        // }
    }
}
