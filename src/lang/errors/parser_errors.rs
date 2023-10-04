flexar::compiler_error! {
    [[Define] SyntaxError]
    /// Occurs when the syntax expected a string or number literal but didn't find one
    (SY001) "expected string or number literal": ((1) "expected literal, found `", "`.");
    /// Occurs when the syntax expected an expr but didn't find one
    (SY002) "expected expr": ((1) "expected expr, found `", "`.");
    /// Occurs when the syntax expected an atom but didn't find one
    (SY003) "expected atom": ((1) "expected atom, found `", "`.");
    /// Occurs when the syntax expected an binop but didn't find one
    (SY004) "expected binop": ((1) "expected binop, found `", "`.");
    /// Occurs when a statement is not ended by a semi-colon
    (SY005) "expected `;` to end statement": ((1) "expected `;`, found `", "`.");
    /// Occurs when the syntax expected a statement but didn't find one
    (SY006) "expected statement": ((1) "expected statement, found `", "`.");
}