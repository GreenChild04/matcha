use super::errors::LexerError;
use flexar::{Flext, compiler_error};

flexar::lexer! {
    [[Token] lext, current, 'cycle]
    else flexar::compiler_error!((LX002, lext.position()) current).throw(); // replace later with symbol token

    token_types {
        // Brackets
        LParen => "(";
        RParen => ")";
        LBracket => "[";
        RBracket => "]";
        LBrace => "{";
        RBrace => "}";

        // Literals
        Int(val: u128) => val;
        Float(val: f64) => val;
        Ident(val: String) => val;
        Str(val: String) => format!("\"{val}\"");

        // Keywords
        Let => "let";
        Mut => "mut";
        Return => "return";
        
        // Symbols
        Plus => "+";
        Minus => "-";
        Mul => "*";
        Div => "/";
        Semi => ";";
        Colon => ":";
        Dot => ".";
        EQ => "=";
        EE => "==";
        GT => ">";
        LT => "<";
        GTE => ">=";
        LTE => "<=";
        Arrow => "=>";
        Comma => ",";
        Not => "!";
        NE => "!=";
    }

    // Brackets
    LParen: '(';
    RParen: ')';
    LBracket: '[';
    RBracket: ']';
    LBrace: '{';
    RBrace: '}';

    // Other symbols
    Plus: +;
    Minus: '-';
    Mul: *;
    Div: /;
    Semi: ;;
    Colon: :;
    Dot: .;
    Comma: ,;

    // Multi
    EE: (= =);
    Arrow: (= >);
    EQ: =;
    GTE: (> =);
    GT: >;
    LTE: (< =);
    LT: <;
    NE: (! =);
    Not: !;

    [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });
     
    / child {
        advance: current;
        ck (current, /) {
            rsome current {
                { if current == '\n' { lext = child; continue 'cycle } };
            };
        };
        advance:();
        done Div();
    };

    '"' child {
        { child.advance() }
        set string { String::new() };
        rsome (current, 'string) {
            { if current == '\n' { break 'string; } };
            ck (current, '"') {
                advance:();
                done Str(string);
            };
            ck (current, '\\') { // Escape characters
                advance: current;
                ck (current, 'n') {
                    advance:();
                    { string.push('\n') };
                    { continue 'string };
                };
                ck (current, 't') {
                    advance:();
                    { string.push('\t') };
                    { continue 'string };
                };
                ck (current, '\\') {
                    advance:();
                    { string.push('\\') };
                    { continue 'string };
                };
                ck (current, '"') {
                    advance:();
                    { string.push('"') };
                    { continue 'string };
                };
                { return compiler_error!((LX003, child.spawn().position()) current).throw() };
            };
            { string.push(current) };
        };
        { return compiler_error!((LX001, child.spawn().position()) current).throw() };
    };

    ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"] child {
        set ident { String::new() };
        rsome (current, 'ident) {
            set matched false;
            ck (current, ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-0123456789"]) {
                mut matched true;
                { ident.push(current) };
            };
            { if !matched { break 'ident } };
        };

        if (ident == "let") { done Let(); };
        if (ident == "mut") { done Mut(); };
        if (ident == "return") { done Return(); };

        done Ident(ident);
    };

    ["0123456789"] child {
        set number { String::new() };
        set dot false;
        rsome (current, 'number) {
            set matched false;
            ck (current, ["0123456789"]) {
                mut matched true;
                { number.push(current) };
            };
            ck (current, '.') {
                if (dot) {
                    done Float(number.parse().unwrap());
                };
                mut matched true;
                mut dot true;
                { number.push(current) };
            };
            {if !matched {break 'number}};
        };
        if (dot) { done Float(number.parse().unwrap()); };
        done Int(number.parse().unwrap());
    };
}