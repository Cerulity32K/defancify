//! This module provides character sets that are ASCII substitutes for Unicode blocks
//! Thanks to [symbl.cc](https://www.symbl.cc) for supplying characters.

use crate::types::Rep::{self, *};

/// Latin Extended-A (`U+0100` to `U+017F`)
pub const LATIN_EXTENDED_A: [Rep; 128] = [
    // European Latin
    Chr('A'), Chr('a'), Chr('A'), Chr('a'), Chr('A'), Chr('a'), Chr('C'), Chr('c'),
    Chr('C'), Chr('c'), Chr('C'), Chr('c'), Chr('C'), Chr('c'), Chr('D'), Chr('d'),
    Chr('D'), Chr('d'), Chr('E'), Chr('e'), Chr('E'), Chr('e'), Chr('E'), Chr('e'),
    Chr('E'), Chr('e'), Chr('E'), Chr('e'), Chr('G'), Chr('g'), Chr('G'), Chr('g'),
    Chr('G'), Chr('g'), Chr('G'), Chr('g'), Chr('H'), Chr('h'), Chr('H'), Chr('h'),
    Chr('I'), Chr('i'), Chr('I'), Chr('i'), Chr('I'), Chr('i'), Chr('I'), Chr('i'),
    Chr('I'), Chr('i'), Str("IJ"),Str("ij"),Chr('J'), Chr('j'), Chr('K'), Chr('k'),
    Chr('K'), Chr('L'), Chr('l'), Chr('L'), Chr('l'), Chr('L'), Chr('l'), Chr('L'),
    Chr('l'), Chr('L'), Chr('l'), Chr('N'), Chr('n'), Chr('N'), Chr('n'), Chr('N'),
    Chr('n'),
    
    // Deprecated Character
    Chr('n'),

    // European Latin
    Chr('N'), Chr('n'), Chr('O'), Chr('o'), Chr('O'), Chr('o'), Chr('O'), Chr('o'),
    Str("OE"),Str("oe"),Chr('R'), Chr('r'), Chr('R'), Chr('r'), Chr('R'), Chr('r'),
    Chr('S'), Chr('s'), Chr('S'), Chr('s'), Chr('S'), Chr('s'), Chr('S'), Chr('s'),
    Chr('T'), Chr('t'), Chr('T'), Chr('t'), Chr('T'), Chr('t'), Chr('U'), Chr('u'),
    Chr('U'), Chr('u'), Chr('U'), Chr('u'), Chr('U'), Chr('u'), Chr('U'), Chr('u'),
    Chr('U'), Chr('u'), Chr('W'), Chr('w'), Chr('Y'), Chr('y'), Chr('Y'), Chr('Z'),
    Chr('z'), Chr('Z'), Chr('z'), Chr('Z'), Chr('z'), Chr('l')
];

/// Latin Extended-B (`U+0180` to `U+024F`)
pub const LATIN_EXTENDED_B: [Rep; 208] = [
    // Non-European and historic Latin
    Chr('b'), Chr('B'), Chr('b'), Chr('b'), Chr('b'), Chr('b'), Chr('C'), Chr('C'),
    Chr('c'), Chr('D'), Chr('D'), Chr('d'), Chr('d'), Chr('g'), Chr('E'), Chr('e'),
    Chr('E'), Chr('F'), Chr('f'), Chr('G'), Chr('V'), Str("hu"),Chr('l'), Chr('I'),
    Chr('K'), Chr('k'), Chr('I'), Chr('y'), Chr('m'), Chr('N'), Chr('n'), Chr('O'),
    Chr('O'), Chr('o'), Str("OI"),Str("oi"),Chr('P'), AmbiChr(&['p', 'b']), Chr('R'), Chr('S'),
    Chr('s'), Chr('E'), Chr('l'), Chr('t'), Chr('T'), Chr('t'), Chr('T'), Chr('U'),
    Chr('u'), Chr('u'), Chr('U'), Chr('Y'), Chr('y'), Chr('Z'), Chr('z'), Chr('3'),
    AmbiChr(&['3', 'E']), AmbiChr(&['3', 'e']), Chr('3'), Chr('2'), Chr('5'), Chr('5'), AmbiChr(&['s', 't']), Chr('p'),

    // African letters for clicks
    Chr('|'), AmbiStr(&["||", "|"]), AmbiChr(&['|', '#']), Chr('!'),
    
    // Latin diagraphs matching Serbian Cyrillic letters
    Str("DZ"),Str("Dz"),Str("dz"),Str("LJ"),Str("Lj"),Str("lj"),Str("NJ"),Str("Nj"),
    Str("nj"),

    // Pinyin diacritic-vowel combinations
    Chr('A'), Chr('a'), Chr('I'), Chr('i'), Chr('O'), Chr('o'), Chr('U'), Chr('u'),
    Chr('U'), Chr('u'), Chr('U'), Chr('u'), Chr('U'), Chr('u'), Chr('U'), Chr('u'),

    // Phonetic and historic letters
    Chr('e'), Chr('A'), Chr('a'), Chr('A'), Chr('a'), Str("AE"), Str("ae"), Str("G"),
    Str("g"), Str("G"), Str("g"), Str("K"), Str("k"), AmbiChr(&['O', 'Q']), AmbiChr(&['o', 'q']), AmbiChr(&['O', 'Q']),
    AmbiChr(&['o', 'q']), Chr('3'), Chr('3'), Chr('j'), Str("DZ"), Str("Dz"), Str("dz"), Chr('G'),
    Chr('g'), AmbiStr(&["Hi", "Hu"]), Chr('p'), Chr('N'), Chr('n'), Chr('A'), Chr('a'), Str("AE"),
    Str("ae"), Chr('O'), Chr('o'),

    // Additions for Slovenian and Croation
    Chr('A'), Chr('a'), Chr('A'), Chr('a'), Chr('E'), Chr('e'), Chr('E'), Chr('e'),
    Chr('I'), Chr('i'), Chr('I'), Chr('i'), Chr('O'), Chr('o'), Chr('O'), Chr('o'),
    Chr('R'), Chr('r'), Chr('R'), Chr('r'), Chr('U'), Chr('u'), Chr('U'), Chr('u'),

    // Additions for Romanian
    Chr('S'), Chr('s'), Chr('T'), Chr('t'),

    // Miscellaneous additions
    Chr('3'), Chr('3'), Chr('H'), Chr('h'), Chr('n'), Chr('d'), Chr('8'), Chr('8'),
    Chr('Z'), Chr('z'), Chr('A'), Chr('a'), Chr('E'), Chr('e'),

    // Additions for Livonian
    Chr('O'), Chr('o'), Chr('O'), Chr('o'), Chr('O'), Chr('o'), Chr('O'), Chr('o'),
    Chr('Y'), Chr('y'),

    // Additions for Sinology
    Chr('l'), Chr('n'), Chr('t'),

    // Miscellaneous addition
    Chr('l'),

    // Additions for Africanist linguistics
    Str("db"), Str("qp"),

    // Additions for Sencoten
    Chr('A'), Chr('C'), Chr('c'), Chr('L'), Chr('T'),

    // Additions for Africanist linguistics
    Chr('s'), Chr('z'),

    // Miscellaneous additions
    Chr('?'), Chr('?'), Chr('B'), Chr('U'), Chr('A'), Chr('E'), Chr('e'), Chr('J'),
    Chr('j'), Chr('Q'), Chr('q'), Chr('R'), Chr('r'), Chr('Y'), Chr('y'),
];

/// IPA Extensions (`U+0250` to `U+02AF`)
pub const IPA_EXTENSIONS: [Rep; 96] = [
    // IPA Extensions
    Chr('a'), Chr('a'), Chr('a'), AmbiChr(&['b', 'a']), Chr('c'), Chr('c'), Chr('d'), Chr('d'),
    Chr('e'), Chr('e'), Chr('e'), Chr('E'), Chr('3'), Chr('3'), AmbiChr(&['B', 'e']), Chr('f'),
    Chr('g'), Chr('g'), Chr('G'), Chr('v'), Chr('v'), Chr('h'), Chr('h'), Chr('h'),
    Chr('i'), Chr('i'), Chr('i'), Chr('l'), Chr('l'), Chr('l'), Str("l3"), Chr('m'),
    AmbiStr(&["m", "hm"]), Chr('m'), Chr('n'), Chr('n'), Chr('N'),  Chr('o'),  Str("OE"), AmbiChr(&['w', 'm']),
    AmbiChr(&['I', 'O']),  Chr('r'), Chr('r'), AmbiChr(&['r', 'j']), Chr('r'), AmbiChr(&['r', 'j']), Chr('r'), Chr('j'),
    Chr('R'), Chr('R'), Chr('S'), Chr('S'), Chr('S'), Chr('S'), Chr('S'), Chr('t'),
    Chr('t'), Chr('u'), Chr('u'), Chr('v'), Chr('v'), Chr('w'), Chr('y'), Chr('Y'),
    Chr('Z'), Chr('Z'), Chr('3'), Chr('3'), Chr('?'), Chr('?'), Chr('?'), Chr('C'),
    Chr('O'), Chr('B'), AmbiChr(&['3', 'B']), Chr('G'), Chr('H'), Chr('j'), Chr('k'), Chr('L'),
    AmbiChr(&['q', 'd']), Chr('?'), Chr('?'), Str("dz"), Str("d3"), Str("dz"), Str("ts"), Str("ts"),
    Str("tc"),

    // IPA characters for disordered speech
    Str("fn"), Str("ls"), Str("lz"), Str("ww"), Chr('_'),

    // Additions for Sinology
    Chr('h'), Chr('h')
];

// Spacing Modifier Letters (`U+02B0` to `U+02FF`)
pub const  SPACING_MODIFIER_LETTERS: [Rep; 80] = [
    // Latin superscript modifier letters
    Chr('h'), Chr('h'), Chr('j'), Chr('r'), Chr('r'), AmbiChr(&['r', 'j']), Chr('R'), Chr('w'),
    Chr('y'),

    // Miscellaneous phonetic modifiers
    Chr('\''),Chr('"'), Chr('\''),Chr('\''),Chr('\''),Chr('\''),Chr('\''),Chr('?'),
    Chr('?'), Chr('<'), Chr('>'), Chr('^'), Chr('V'), Chr('^'), Chr('v'), Chr('\''),
    Chr('_'), Chr('\''),Chr('\''),Chr(','), Chr('_'), Chr(','), Chr(','), Chr(':'),
    Chr('\''),Chr(','), Chr(','), Chr('.'), Chr('T'), Chr('+'), Chr('-'),

    // Spacing clones of diacritics
    Chr('_'), Chr('.'), Chr('o'), Chr(','), Chr('~'), Chr('"'),

    // Additions based on 1989 IPA
    Chr(','), Chr('x'), Chr('v'), Chr('l'), Chr('s'), Chr('x'), Chr('?'),

    // Tone letters
    Chr('L'), Chr('t'), Chr('+'), Chr('t'), Chr('L'),

    // Extended Bopomofo tone marks
    Chr('L'), Chr('+'),

    // IPA Modifiers
    Chr(','), Chr('='),
    
    // Other modifier letter
    Chr('"'),

    // UPA modifiers
    Chr(','), Chr(','), Chr('<'), Chr('>'), Chr('o'), Chr('\''), Chr('"'), Chr('"'),
    Chr('~'), Chr(':'), Chr('L'), Chr('L'), Chr('L'), Chr('L'), Chr('_'), Chr('_'),
    Str("<-"),
];
