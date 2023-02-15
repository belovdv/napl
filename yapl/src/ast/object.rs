// To be done: there will be unit actions on objects in this file.

/// All is an object.
#[derive(Default)]
pub enum Object {
    #[default]
    None, // To be done: is it necessary?
    Action(Action),
    Set(Set),
    Instance(Set),
}

pub enum Action {}

pub enum Predicate {}

// To be done: this should be able to refer to `Id`.
pub enum Set {
    Product(Vec<Set>),
    Sum(Vec<Set>),
    Atom(SetAtom),
    Sequence(Box<Set>),
    Filtered(Box<Set>, Predicate),
}

pub enum SetAtom {
    Integer,
    Character,
}
