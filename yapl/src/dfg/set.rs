use std::cmp::Ordering;

use super::{ObjS, Space};

// To be done: algebraic description of operations and, most important, `Any` and `Unit`.

pub enum Set {
    Any,          // The Superset - contains all sets and their elements (func?) except self.
    Unit,         // Auxiliary, recurrent: Unit = { Unit }. To be done: recurrent??
    Integer,      // Superset for int.
    Unsigned,     // [0, +inf) - base for infinite sequences.
    Range(usize), // [0, val) - Base for finite sequences.

    AnySet,  // Superset - contains all sets including self.
    AnySeq,  // Superset - contains all sequential sets including self.
    AnyFunc, // Superset - contains all functions.

    // Expected at least two args.
    Sum(Vec<ObjS>),          // Enum.
    Product(Vec<ObjS>),      // Tuple.
    Union(Vec<ObjS>),        // After match branches.
    Intersection(Vec<ObjS>), // Trait sum.

    // Superset for functions. To be done: from - vec?
    Pow {
        from: ObjS,
        to: ObjS,
        with_type: ObjS,
    },
    // Predicate?
    Difference {
        from: ObjS,
        sub: ObjS,
    },
}

impl Set {
    pub fn get_superset(&self, space: &Space) -> ObjS {
        space.b_s.any // To be done: a bit of preciseness... It's unused now.
    }
    pub fn get_elemets_set_type(&self, space: &Space) -> ObjS {
        // To be done: Check.
        match self {
            Set::Any | Set::AnySet | Set::AnySeq => space.b_s.any,

            Set::Union(obs) | Set::Intersection(obs) => {
                obs[0].get(&space.s).get_elemets_set_type(space)
            }
            Set::Pow { with_type, .. } => *with_type,
            Set::Difference { from, .. } => from.get(&space.s).get_elemets_set_type(space),

            Set::Product(_) | Set::Sum(_) => space.b_s.unit, // Check.
            Set::AnyFunc | Set::Unit | Set::Integer | Set::Unsigned | Set::Range(_) => {
                space.b_s.unit
            }
        }
    }

    // To be done: what is `sequential` exactly?
    pub fn is_sequential(&self, space: &Space) -> bool {
        match &self {
            Set::Unsigned | Self::Range(_) | Self::Unsigned => true,
            Set::Union(_)
            | Set::AnySeq
            | Set::Sum(_)
            | Set::Pow { .. }
            | Set::AnyFunc
            | Set::AnySet
            | Set::Unit
            | Set::Any
            | Set::Integer
            | Set::Pow { .. }
            | Set::Product(_) => false,

            Set::Product(v) => v.iter().all(|s| s.get(&space.s).is_sequential(space)),
            Set::Intersection(v) => v.iter().any(|s| s.get(&space.s).is_sequential(space)),
            Set::Difference { from, .. } => from.get(&space.s).is_sequential(space),
        }
    }
    pub fn is_finite(&self, space: &Space) -> bool {
        match self {
            Set::Unit | Set::Range(_) => true,

            Set::Any | Set::Integer | Set::Unsigned | Set::AnySet | Set::AnySeq | Set::AnyFunc => {
                false
            }

            Set::Pow { from, to, .. } => {
                from.get(&space.s).is_finite(space) && to.get(&space.s).is_finite(space)
            }
            Set::Union(v) | Set::Product(v) | Set::Sum(v) => {
                v.iter().all(|s| s.get(&space.s).is_finite(space))
            }
            Set::Intersection(v) => v.iter().any(|s| s.get(&space.s).is_finite(space)),
            Set::Difference { from, .. } => from.get(&space.s).is_finite(space),
        }
    }

    // R `None` if couldn't prove any. To be done: a bit of preciseness... It's unused now.
    pub fn is_subset_of(&self, other: &Set) -> Option<Ordering> {
        None
    }
    // R `None` if couldn't prove any. To be done: a bit of preciseness... It's unused now.
    pub fn is_superset_for(&self, other: &Set) -> Option<Ordering> {
        None
    }
}
