flexar::compiler_error! {
    [[Define] SyntaxError]
    /// Occurs when the syntax expected a string or number literal but didn't find one
    (SY001) "expected string or number literal": ((1) "expected literal, found `", "`.");
}