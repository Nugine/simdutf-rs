use super::*;

#[test]
fn test_validate_ascii() {
    let cases: &[&[u8]] = &[
        "hello".as_bytes(),  //
        &[0xff, 0xff, 0xff], //
    ];

    for s in cases {
        let output = validate_ascii(s);
        let expected = s.is_ascii();
        assert_eq!(output, expected);
    }
}

#[test]
fn test_utf8_to_utf16() {
    let cases: &[&str] = &[
        "hello",                            //
        "世界",                           //
        "lycoris recoil 莉可丽丝 😗", //
    ];

    for s in cases {
        {
            let output = validate_utf8(s.as_bytes());
            let expected = true;
            assert_eq!(output, expected);
        }

        let utf16: Vec<u16> = s.encode_utf16().collect();

        {
            let output = unsafe { count_utf16_from_utf8(s.as_bytes()) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        let mut buf: Vec<u16> = Vec::with_capacity(utf16.len());

        {
            let len = s.len();
            let src = s.as_bytes().as_ptr();
            let dst = buf.as_mut_ptr();
            let output = unsafe { convert_arbitrary_utf8_to_utf16le(src, len, dst) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        unsafe { buf.set_len(utf16.len()) };

        {
            let output = &buf;
            let expected = &utf16;
            assert_eq!(output, expected);
        }

        {
            let len = s.len();
            let src = s.as_bytes().as_ptr();
            let dst = buf.as_mut_ptr();
            let output = unsafe { convert_valid_utf8_to_utf16le(src, len, dst) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        {
            let output = &buf;
            let expected = &utf16;
            assert_eq!(output, expected);
        }
    }
}
