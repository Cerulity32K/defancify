//! A crate for cleaning and converting Unicode text to ASCII* text. Used mainly for screen readers.
//! *There are methods for converting to standard alphabets, and not just ASCII.
//! no_std, but requires alloc.

#![no_std]

use types::{Rep, BlockFlags};
use str_subs::*;

pub mod str_subs;
pub mod types;
pub mod allocs;
mod tests;

macro_rules! sub {
    ($ch:expr, $base:tt, $set:tt) => {
        $set.get($ch as usize - $base as usize).cloned()
    };
}

type StaticRep = Rep<'static, 'static>;

pub fn chmatch(i: char, blocks: &BlockFlags) -> Option<StaticRep> {
    match i {
        '\u{0000}'..='\u{00ff}' => Some(Rep::Chr(i)),
        '\u{0100}'..='\u{017f}' if blocks.latin_extended_a => sub!(i, '\u{0100}', LATIN_EXTENDED_A),
        '\u{0180}'..='\u{024f}' if blocks.latin_extended_b => sub!(i, '\u{0180}', LATIN_EXTENDED_B),
        '\u{0250}'..='\u{02af}' if blocks.ipa_extensions => sub!(i, '\u{0250}', IPA_EXTENSIONS),
        '\u{02b0}'..='\u{02ff}' if blocks.spacing_modifier_letters => sub!(i, '\u{02b0}', SPACING_MODIFIER_LETTERS),
        _ => None
    }
}
