use super::*;

#[test]
fn test_validate_ascii() {
    let cases: &[&[u8]] = &[
        "hello".as_bytes(),  //
        &[0xff, 0xff, 0xff], //
    ];

    for s in cases {
        let output = ASCII::validate(s);
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
            let output = UTF8::validate(s.as_bytes());
            let expected = true;
            assert_eq!(output, expected);
        }

        let codec = Transcoder::new(UTF8, UTF16LE);
        let utf16: Vec<u16> = s.encode_utf16().collect();

        {
            let output = unsafe { codec.converted_count(s.as_bytes()) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        let mut buf: Vec<u16> = Vec::with_capacity(utf16.len());

        {
            let output = unsafe { codec.convert_arbitrary(s.as_bytes(), buf.as_mut_ptr()) };
            let expected = utf16.len();
            assert_eq!(output, expected);
        }

        unsafe { buf.set_len(utf16.len()) };

        {
            let output = &buf;
            let expected = &utf16;
            assert_eq!(output, &*expected);
        }

        {
            let output = unsafe { codec.convert_valid(s.as_bytes(), buf.as_mut_ptr()) };
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
