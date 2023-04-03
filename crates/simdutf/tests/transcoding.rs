#![allow(clippy::bool_assert_comparison)]

#[test]
fn utf8_to_utf16() {
    let cases: &[&str] = &[
        "hello",
        "世界",
        "Puella Magi Madoka Magica 魔法少女小圆 魔法少女まどか☆マギカ 😗",
    ];

    for s in cases {
        assert_eq!(simdutf::validate_utf8(s.as_bytes()), true);

        let utf16: Vec<u16> = s.encode_utf16().collect();

        assert_eq!(unsafe { simdutf::utf16_length_from_utf8(s.as_bytes()) }, utf16.len());

        let mut buf: Vec<u16> = Vec::with_capacity(utf16.len());

        {
            let len = s.len();
            let src = s.as_bytes().as_ptr();
            let dst = buf.as_mut_ptr();
            let output = unsafe { simdutf::convert_utf8_to_utf16le(src, len, dst) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        unsafe { buf.set_len(utf16.len()) };
        assert_eq!(&buf, &utf16);

        {
            let len = s.len();
            let src = s.as_bytes().as_ptr();
            let dst = buf.as_mut_ptr();
            let output = unsafe { simdutf::convert_valid_utf8_to_utf16le(src, len, dst) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        assert_eq!(&buf, &utf16);
    }
}
