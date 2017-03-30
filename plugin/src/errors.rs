error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    //
    // It is also possible to leave this section out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        LexError(offset: usize, msg: &'static str) {
            description("invalid token in the input")
            display("invalid token at offset {}: {}", offset, msg)
        }
    }
}
