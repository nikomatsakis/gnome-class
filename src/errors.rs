use syn::synom::ParseError;

error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    //
    // It is also possible to leave this section out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        Parse(ParseError);
    }

    errors {
        LexError(offset: usize, msg: &'static str) {
            description("invalid token in the input")
            display("invalid token at offset {}: {}", offset, msg)
        }
        InstancePrivateError(msg: String) {
            description("zero or one InstancePrivate types expected")
            display("at most one InstancePrivate type definitions expected: {}", msg)
        }
    }
}
