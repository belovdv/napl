pub enum Object {
    Set(Set),
    Instance(Set),
    Derived { from: Box<Object>, act: Action },
    Function(Function),
}

pub use set::Set;
pub mod set {
    pub enum Set {
        Unit(Unit),
        Algebraic(Algebraic),
    }

    pub enum Algebraic {
        Sum(Vec<Set>),
        Product(Vec<Set>),
        Union(Vec<Set>),
        Pow(Box<Set>, Box<Set>),
    }
    pub enum Unit {
        Unit,
        Bool,
        Number(Number),
    }
    pub enum Number {
        Integer,
        Float,
    }
}

pub use action::Action;
pub mod action {
    pub enum Action {
        User,
        Arithmetic(Arithmetic),
    }

    pub enum Arithmetic {
        Sum,   // Take many terms.
        Mul,   // Take many terms.
        Minus, // Take one term.
        Pow,   // Take two terms.
    }
}

pub use function::Function;
pub mod function {
    pub struct Function {}
}

/*
    macro(type, property_type, *(name: property_val))
*/
