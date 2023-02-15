pub trait IsError: super::location::HasSpan + std::fmt::Debug {
    fn message(&self) -> String;
}

pub type Error = Box<dyn IsError>;
pub type Result<T> = core::result::Result<T, Error>;

macro_rules! error_struct {
    ($name:ident, $message:literal, $($data:ident: $ty:ty),*) => {
        #[derive(derive_new::new, Debug)]
        pub struct $name {
            span: crate::common::location::Span,
            $($data: $ty),*
        }

        impl crate::common::location::HasSpan for $name {
            fn span(&self) -> crate::common::location::Span {
                self.span
            }
        }
        impl crate::common::error::IsError for $name {
            fn message(&self) -> String {
                format!($message, $(self.$data),*)
            }
        }
    };
}

pub(crate) use error_struct;

// To be done: get rid of trailing comma in case with no `data`.
macro_rules! raise_error {
    ($name:ident, $span:expr, $($data:expr),*) => {
        return Err(Box::new($name::new($span, $($data),*)))
    };
}
pub(crate) use raise_error;
