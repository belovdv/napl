use crate::common::Symbol;

// To be done: this is placeholder.

// Returns: number of args.
pub fn operator(s: Symbol) -> Option<usize> {
    match s {
        s if ["*", "+", "%", ">", "=", "-"].iter().any(|&p| s == p) => Some(2),
        _ => None,
    }
}
