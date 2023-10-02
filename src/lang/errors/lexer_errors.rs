flexar::compiler_error! {
    [[Define] LexerError]
    /// Occurs when there is a new-line before the string is terminated
    (LX001) "non terminated string": "expected '\"' to terminate string";
    /// Occurs when there is a character that wasn't expected or used by a token
    (LX002) "unexpected character": ((1) "character `", "` is unexpected");
    /// Occurs when there is an invalid escape character
    (LX003) "invalid escape character": ((1) "`", "` is not a valid escape character");
}