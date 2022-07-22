mod bindings;

#[inline]
pub fn validate_utf8(data: &[u8]) -> bool {
    let buf = data.as_ptr();
    let len = data.len();
    unsafe { self::bindings::simdutf_validate_utf8(buf, len) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_utf8() {
        let cases: &[&[u8]] = &[
            "hello世界".as_bytes(), //
            &[0xff, 0xff, 0xff],      //
        ];

        for s in cases {
            let output = validate_utf8(s);
            let expected = core::str::from_utf8(s).is_ok();
            assert_eq!(output, expected);
        }
    }
}
