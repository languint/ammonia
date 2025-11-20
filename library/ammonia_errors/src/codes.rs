use ammonia_error_macros::gen_error_codes;

// All error codes follow a pattern where the first digit shows what submodule emits that specific error.
// `E0XXX` - `ammonia_lexer`
// `E1XXX` - `ammonia_parser`
// `E9XXX` - `ammonia_errors`
gen_error_codes!(ErrCode, 9999);
