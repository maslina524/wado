pub fn str_to_utf16(s: &str, buf: &mut [u16]) -> usize {
    let mut i = 0;
    for ch in s.encode_utf16() {
        if i >= buf.len() - 1 { break; }
        buf[i] = ch;
        i += 1;
    }
    if i < buf.len() {
        buf[i] = 0;
    }
    i
}