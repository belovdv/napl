pub trait Error: super::location::HasSpan {
    fn message(&self) -> String;
}

macro_rules! struct_error {
    ($name:ident, $message:literal, $($data:ident: $ty:ty),*) => {
        #[derive(derive_new::new)]
        pub struct $name {
            $($data: $ty),*,
            span: crate::common::location::Span,
        }

        impl $name {
            pub fn raise_on<T>(object: &T, $($data: $ty),*) -> Self
            where T: crate::common::location::HasSpan {
                Self::new($($data),*,<T as crate::common::location::HasSpan>::span(&object))
            }

            pub fn raise_from_to(
                begin: crate::common::location::Position,
                end: crate::common::location::Position,
                $($data: $ty),*
            ) -> Self {
                Self::new($($data),*, crate::common::location::Span::new(begin, end))
            }
        }

        impl crate::common::location::HasSpan for $name {
            fn span(&self) -> crate::common::location::Span {
                self.span
            }
        }
        impl crate::common::error::Error for $name {
            fn message(&self) -> String {
                format!($message, $(self.$data),*)
            }
        }
    };
}
pub(crate) use struct_error;

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;
