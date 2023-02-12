use crate::common::error::error_struct;

// TODO: sort out, what every error exactly is.

error_struct!(LiteralString, "",);
error_struct!(ExpectedIdentifier, "",);
error_struct!(ExpectedWhitespace, "",);
error_struct!(UnsupportedSymbol, "",);
error_struct!(ParseInt, "{}", error: String);
error_struct!(WrongLineOffset, "{}", offset: usize);
error_struct!(WrongBracket, "",);
error_struct!(ClosingBracket, "",);
error_struct!(EmptyBracketPart, "",);
