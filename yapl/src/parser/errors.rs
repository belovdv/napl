use crate::common::error::error_struct;

error_struct!(UnsupportedSymbol, "symbol '{}' isn't supported", symbol: char);
error_struct!(UnexpectedEOS, "EOS wasn't expected here",);
error_struct!(UnexpectedSymbol, "symbol '{}' wasn't expected here", symbol: char);
error_struct!(ParseInt, "cannot parse '{}' as i64", int: String);
error_struct!(ClosedBracket, "cannot find open pair for this bracket",);
error_struct!(ClosingBracketNotFound, "cannot find closing bracket",);
error_struct!(UnexpectedToken, "`inner` cannot be followed by this",);
error_struct!(EmptyPartInBrackets, "parts in brackets shouldn't be empty",);
error_struct!(UnexpectedEndOfLine, "End of line wasn't expected here",);
error_struct!(WrongLineOffset, "unexpected offset {}", offset: usize);
error_struct!(NewLineOnFileEnd, "unexpected new line on the end of file",);
