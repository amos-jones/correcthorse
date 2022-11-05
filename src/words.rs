// file: words.rs

pub const WORDS_SHORT: &[&str] = &include!(concat!(env!("OUT_DIR"), "/short.in.rs"));
pub const WORDS_MEDIUM: &[&str] = &include!(concat!(env!("OUT_DIR"), "/medium.in.rs"));
pub const WORDS_LONG: &[&str] = &include!(concat!(env!("OUT_DIR"), "/long.in.rs"));

pub const fn short() -> &'static[&'static str] {
    WORDS_SHORT
}
pub const fn medium() -> &'static[&'static str] {
    WORDS_MEDIUM
}
pub const fn long() -> &'static[&'static str] {
    WORDS_LONG
}

pub fn words(is_short: bool, is_medium: bool, is_long: bool) -> Vec<&'static str> {
    let mut words = Vec::new();
    let all_or_none = is_short == is_medium && is_medium == is_long;
    if is_short || all_or_none {
        words.extend_from_slice(short());
    }
    if is_medium || all_or_none {
        words.extend_from_slice(medium());
    }
    if is_long || all_or_none {
        words.extend_from_slice(long());
    }
    words
}

// words.rs
