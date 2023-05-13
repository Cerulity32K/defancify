//! The allocation-based module. Contains implementations and functions which depend on `alloc`.
//! No other modules should contain `extern crate alloc;`.
//! To disable this crate, you can delete `mod allocs;` in `lib.rs`
extern crate alloc;
use core::fmt::Display;

use alloc::string::{String, ToString};
use crate::types::{Rep, BlockFlags, RepNoneContext};
use crate::chmatch;

/// Intakes a function that can mutate the output string when a disabled/unmapped character is met, along with the context.\
/// The function should push characters; you receive the full string in the function.
/// Other defancifying functions in this crate use preset functions.
pub fn defancify(st: &str, flags: Option<BlockFlags>, f: fn(&mut String, char, RepNoneContext)) -> String {
    let mut out: String = String::new();
    let bf_inf = flags.unwrap_or(BlockFlags::default());
    for i in st.chars() {
        match chmatch(i, &bf_inf) {
            Some(sub) => match sub {
                Rep::Chr(ch) => out.push(ch),
                Rep::Str(st) => out.push_str(st),
                Rep::AmbiChr(choices) => {
                    match choices.get(0) {
                        Some(ch) => out.push(*ch),
                        None => { f(&mut out, i, RepNoneContext::NoChrChoices) }
                    }
                }
                Rep::AmbiStr(choices) => {
                    match choices.get(0) {
                        Some(st) => out.push_str(*st),
                        None => { f(&mut out, i, RepNoneContext::NoStrChoices) }
                    }
                },
                Rep::None => f(&mut out, i, RepNoneContext::NoSubstitution)
            },
            None => { f(&mut out, i, RepNoneContext::NoSubstitution) }
        }
    }
    out
}
/// Keeps all disabled/unmapped characters.
pub fn defancify_keep(st: &str, flags: Option<BlockFlags>) -> String {
    defancify(st, flags, |s, ch, _|s.push(ch))
}
/// Discards all disabled/unmapped characters.
pub fn defancify_discard(st: &str, flags: Option<BlockFlags>) -> String {
    defancify(st, flags, |_, _, _|())
}
/// Replaces all disabled/unmapped characters with '?'.
pub fn defancify_lossy(st: &str, flags: Option<BlockFlags>) -> String {
    defancify(st, flags, |s, _, _|s.push('?'))
}

impl<'a, 'b> Rep<'a, 'b> {
    /// *alloc function*\
    /// Converts a 
    pub fn to_str(&self, ambi_idx: usize) -> String {
        match self {
            Rep::Chr(ch) => ch.to_string(),
            Rep::Str(st) => st.to_string(),
            Rep::AmbiChr(choices) => choices[ambi_idx.clamp(0, choices.len())].to_string(),
            Rep::AmbiStr(choices) => choices[ambi_idx.clamp(0, choices.len())].to_string(),
            Rep::None => String::new()
        }
    }
}
impl<'a, 'b> Display for Rep<'a, 'b> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_str(0))
    }
}
