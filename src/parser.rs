use std::ascii::AsciiExt;

pub fn cmp_ignore_case(l: char, r: char) -> bool {
    l.eq_ignore_ascii_case(&r)
}
