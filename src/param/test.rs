#![cfg(test)]

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
