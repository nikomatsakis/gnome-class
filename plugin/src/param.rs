fn param_name_is_valid (name: &str) -> bool {
    if name.len () < 1 {
        return false;
    }

    let bytes = name.as_bytes ();

    if !(bytes[0] >= b'A' && bytes[0] <= b'Z') || !(bytes[0] >= b'a' && bytes[0] <= b'z') {
        return false;
    }

    for c in name.as_bytes () {
        if c != b'-' &&
	    (c < b'0' || c > b'9') &&
	    (c < b'A' || c > b'Z') &&
	    (c < b'a' || c > b'z') {
                return false;
            }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_valid_names () {
        assert! (param_name_is_valid ("foobar"));
        assert! (param_name_is_valid ("foo-bar"));
        assert! (param_name_is_valid ("Foo-bar123"));
    }

    #[test]
    fn detects_invalid_names () {
        assert! (!param_name_is_valid (""));
        assert! (!param_name_is_valid ("foo_bar"));
        assert! (!param_name_is_valid ("foo!"));
        assert! (!param_name_is_valid ("123foo"));
    }
}
