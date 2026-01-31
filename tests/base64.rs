#[test]
fn base64_decoding() {
    let input = "aGVsbG8gd29ybGQK";
    let len = input.len();
    let output = b"hello world\n";
    let mut out_buf = [0u8; 1024];
    let mut out_len = out_buf.len();
    let result = unsafe {
        simdutf::base64_to_binary_safe(
            input.as_ptr(),
            len,
            out_buf.as_mut_ptr(),
            &mut out_len,
            simdutf::Base64Options::Default,
            simdutf::LastChunkHandlingOptions::Loose,
            false,
        )
    };
    assert_eq!(result.error, simdutf::ErrorCode::Success);
    assert_eq!(out_len, output.len());
    assert_eq!(&out_buf[..out_len], output);
}

#[test]
fn base64_encoding() {
    let input = "hello world\n";
    let len = input.len();
    let output = b"aGVsbG8gd29ybGQK";
    let mut out_buf = [0u8; 1024];
    let out_len = unsafe {
        simdutf::binary_to_base64(input.as_ptr(), len, out_buf.as_mut_ptr(), simdutf::Base64Options::Default)
    };
    assert_eq!(out_len, output.len());
    assert_eq!(&out_buf[..out_len], output);
}

// https://github.com/tc39/test262/blob/f0dc15c6c7ec095ba3caf3acc0f8665394665841/test/built-ins/Uint8Array/fromBase64/last-chunk-invalid.js
#[test]
fn tc39_illegal_padded_chunks() {
    let test_cases = [
        "=", "==", "===", "====", "=====", "A=",
        "A==", "A===", "A====", "A=====", "AA====", "AA=====",
        "AAA==", "AAA===", "AAA====", "AAA=====", "AAAA=", "AAAA==",
        "AAAA===", "AAAA====", "AAAA=====", "AAAAA=", "AAAAA==", "AAAAA===",
        "AAAAA====", "AAAAA=====",
    ];
    let options = [
        simdutf::LastChunkHandlingOptions::Strict,
        simdutf::LastChunkHandlingOptions::Loose,
        simdutf::LastChunkHandlingOptions::StopBeforePartial,
    ];
    for input in test_cases {
        let len = input.len();
        for option in options {
            let mut out_buf = [0u8; 255];
            let mut out_len = out_buf.len();
            let result = unsafe {
                simdutf::base64_to_binary_safe(
                    input.as_ptr(),
                    len,
                    out_buf.as_mut_ptr(),
                    &mut out_len,
                    simdutf::Base64Options::Default,
                    option,
                    true,
                )
            };
            assert_ne!(result.error, simdutf::ErrorCode::Success);
        }
    }
}

// https://github.com/tc39/test262
#[test]
fn tc39_4a() {
    let test_cases = [
        ("ZXhhZg==", vec![101, 120, 97, 102]),
        ("ZXhhZg", vec![101, 120, 97]),
        ("ZXhhZh==", vec![101, 120, 97, 102]),
        ("ZXhhZh", vec![101, 120, 97]),
        ("ZXhhZg=", vec![101, 120, 97]),
    ];
    for (input, expected) in test_cases {
        let len = input.len();
        let mut out_buf = vec![255u8; expected.len()];
        let mut out_len = out_buf.len();
        let result = unsafe {
            simdutf::base64_to_binary_safe(
                input.as_ptr(),
                len,
                out_buf.as_mut_ptr(),
                &mut out_len,
                simdutf::Base64Options::Default,
                simdutf::LastChunkHandlingOptions::StopBeforePartial,
                true,
            )
        };
        assert_eq!(result.error, simdutf::ErrorCode::Success);
        assert_eq!(out_len, expected.len());
        assert_eq!(&out_buf[..out_len], expected.as_slice());
    }
}

// https://github.com/tc39/test262
#[test]
fn tc39_3a() {
    let input = "ZXhhZg===";
    let len = input.len();
    let mut out_buf = [255u8; 5];
    let mut out_len = out_buf.len();
    let result = unsafe {
        simdutf::base64_to_binary_safe(
            input.as_ptr(),
            len,
            out_buf.as_mut_ptr(),
            &mut out_len,
            simdutf::Base64Options::Default,
            simdutf::LastChunkHandlingOptions::StopBeforePartial,
            true,
        )
    };
    assert_ne!(result.error, simdutf::ErrorCode::Success);
}

// https://github.com/tc39/test262
#[test]
fn tc39_1a() {
    let input = "Zm9vYmE=";
    let len = input.len();
    let expected = [102, 111, 111, 98, 97];
    let mut out_buf = [255u8; 5];
    let mut out_len = out_buf.len();
    let result = unsafe {
        simdutf::base64_to_binary_safe(
            input.as_ptr(),
            len,
            out_buf.as_mut_ptr(),
            &mut out_len,
            simdutf::Base64Options::Default,
            simdutf::LastChunkHandlingOptions::StopBeforePartial,
            true,
        )
    };
    assert_eq!(result.error, simdutf::ErrorCode::Success);
    assert_eq!(result.count, 8);
    assert_eq!(out_len, 5);
    assert_eq!(out_buf, expected);
}

// https://github.com/tc39/test262
#[test]
fn tc39_2() {
    let input = "Zm9vYmE";
    let len = input.len();
    let expected = [102, 111, 111, 255, 255];
    let mut out_buf = [255u8; 5];
    let mut out_len = out_buf.len();
    let result = unsafe {
        simdutf::base64_to_binary_safe(
            input.as_ptr(),
            len,
            out_buf.as_mut_ptr(),
            &mut out_len,
            simdutf::Base64Options::Default,
            simdutf::LastChunkHandlingOptions::StopBeforePartial,
            true,
        )
    };
    assert_eq!(result.error, simdutf::ErrorCode::Success);
    assert_eq!(result.count, 4);
    assert_eq!(out_len, 3);
    assert_eq!(out_buf, expected);
}

// https://bugs.webkit.org/show_bug.cgi?id=290829
// Test decode_up_to_bad_char parameter - should decode up to first invalid character
#[test]
fn decode_up_to_bad_char() {
    let input = "MjYyZg==="; // Invalid: too much padding
    let len = input.len();
    let expected = [0x32, 0x36, 0x32]; // "262" decoded
    let options = [
        simdutf::LastChunkHandlingOptions::Strict,
        simdutf::LastChunkHandlingOptions::Loose,
        simdutf::LastChunkHandlingOptions::StopBeforePartial,
    ];
    for option in options {
        let mut out_buf = [0u8; 5];
        let mut out_len = out_buf.len();
        let result = unsafe {
            simdutf::base64_to_binary_safe(
                input.as_ptr(),
                len,
                out_buf.as_mut_ptr(),
                &mut out_len,
                simdutf::Base64Options::Default,
                option,
                true, // decode_up_to_bad_char
            )
        };
        // Should fail but decode up to the invalid character
        assert_ne!(result.error, simdutf::ErrorCode::Success);
        assert_eq!(result.count, 6); // Position of invalid character
        assert_eq!(out_len, 3);
        assert_eq!(out_buf[..3], expected);
    }
}
