//! The tests module. This module is `alloc`.
#![cfg(test)]
use crate::allocs::defancify_keep;
use crate::str_subs::*;
extern crate std;
use std::*;

#[test]
fn latin_extended_a() {
    assert_eq!(&defancify_keep("\u{0124}\u{011b}\u{013a}\u{0140}\u{014d}", None)[..], "Hello");
}

#[test]
fn length() {
    println!("Total characters mapped: {}",
        LATIN_EXTENDED_A.len() +
        LATIN_EXTENDED_B.len() +
        IPA_EXTENSIONS.len() +
        SPACING_MODIFIER_LETTERS.len()
    );
}
