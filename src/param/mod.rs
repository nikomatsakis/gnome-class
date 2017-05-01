mod test;

fn param_name_is_valid (name: &str) -> bool {
    if name.len () < 1 {
        return false;
    }

    let bytes = name.as_bytes ();

    let first_byte_valid =
        (bytes[0] >= b'A' && bytes[0] <= b'Z') ||
        (bytes[0] >= b'a' && bytes[0] <= b'z');

    first_byte_valid &&
        bytes[1..]
            .iter()
            .all(|&c| {
                c == b'-' ||
	                (c >= b'0' && c <= b'9') ||
	                (c >= b'A' && c <= b'Z') ||
	            (c >= b'a' && c <= b'z')
            })
}
