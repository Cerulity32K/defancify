//! This module provides types for the crate.
//! This module has no `alloc` based implementations. `allocs` contains implementations which require `alloc`.

/// For use in the main `defancify` function, provides context to None arms
pub enum RepNoneContext {
    /// For when a `Rep::AmbiStr` value has no choices
    NoStrChoices,
    /// For when a `Rep::AmbiChr` value has no choices
    NoChrChoices,
    /// For when no substitution was found (through chars mapped to no range or through blocks explicitly disabled)
    NoSubstitution
}

/// Awesome and helpful and cool macro for flag definitions
macro_rules! flag {
    ($name:tt) => {
        pub fn $name(&mut self, enable: bool) {
            self.$name = enable;
        }
    };
}

/// Builder-like struct for enabling/disabling conversions for different Unicode blocks.
pub struct BlockFlags {
    pub latin_extended_a: bool,
    pub latin_extended_b: bool,
    pub ipa_extensions: bool,
    pub spacing_modifier_letters: bool
}
impl Default for BlockFlags {
    fn default() -> Self {
        BlockFlags {
            latin_extended_a: true,
            latin_extended_b: true,
            ipa_extensions: true,
            spacing_modifier_letters: true
        }
    }
}
impl BlockFlags {
    flag!{latin_extended_a}
    flag!{latin_extended_b}
    flag!{ipa_extensions}
    flag!{spacing_modifier_letters}
}

/// Substitution enum. Some substitutions require multiple characters, but most do not and can be represented as chars.
#[derive(Clone, Copy)]
pub enum Rep<'a, 'b> {
    /// For multi-ASCII Unicode characters, such as ligatures
    Str(&'a str),
    /// For simple replacements
    Chr(char),
    /// For ambiguous multi-ASCII Unicode characters, such as ligatures or special symbols which can be interpreted multiple ways.
    AmbiStr(&'b[&'a str]),
    /// For characters that can be interpreted multiple ways.
    AmbiChr(&'b[char]),
    /// For illegible, zero-sized, or blank Unicode characters.
    None
}
impl<'a, 'b> Rep<'a, 'b> {
    /// Returns None for non-ambiguous substitutions, otherwise returns the number of ambiguous choices.
    /// 
    /// See `choices` for infallibility.
    pub fn ambi_choices(&self) -> Option<usize> {
        match self {
            Rep::AmbiStr(choices) => Some(choices.len()),
            Rep::AmbiChr(choices) => Some(choices.len()),
            _ => None
        }
    }
    /// Returns 1 for non-ambiguous substitutions, otherwise returns the number of ambiguous choices.
    /// 
    /// See `choices_fallible` for fallibility.
    pub fn choices(&self) -> usize {
        match self {
            Rep::AmbiStr(choices) => choices.len(),
            Rep::AmbiChr(choices) => choices.len(),
            Rep::None => 0,
            _ => 1
        }
    }
}
