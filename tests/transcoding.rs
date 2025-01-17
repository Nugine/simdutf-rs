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

        assert_eq!(simdutf::utf16_length_from_utf8(s.as_bytes()), utf16.len());

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

#[test]
fn utf16_to_utf8() {
    let expected = "hello你好";
    let input: Vec<u16> = expected.encode_utf16().collect();

    let mut buf = vec![0; expected.len()];
    let written = unsafe {
        let len = input.len();
        let src = input.as_ptr();
        let dst = buf.as_mut_ptr();
        simdutf::convert_valid_utf16_to_utf8(src, len, dst)
    };

    assert_eq!(written, expected.len());
    assert_eq!(&buf[..written], expected.as_bytes());
}

#[test]
fn latin1_to_utf8() {
    let expected = "hello¢";
    let mut input: Vec<u8> = "hello".as_bytes().to_vec();
    input.push(0xA2);

    let mut buf = vec![0; expected.len()];
    let written = unsafe {
        let len = input.len();
        let src = input.as_ptr();
        let dst = buf.as_mut_ptr();
        simdutf::convert_latin1_to_utf8(src, len, dst)
    };

    assert_eq!(written, expected.len());
    assert_eq!(&buf[..written], expected.as_bytes());
}

#[test]
fn utf8_to_latin1() {
    let mut expected = "hello".as_bytes().to_vec();
    expected.push(0xA2);
    let input = "hello¢";

    let mut buf = vec![0; input.len()];
    let written = unsafe {
        let len = input.len();
        let src = input.as_ptr();
        let dst = buf.as_mut_ptr();
        simdutf::convert_utf8_to_latin1(src, len, dst)
    };

    assert_eq!(written, expected.len());
    assert_eq!(&buf[..written], expected);
}
