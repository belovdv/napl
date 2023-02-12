use crate::common::error::struct_error;

struct_error!(ErrorSimple, "error: {}", message: String);
