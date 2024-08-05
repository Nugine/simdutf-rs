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
        )
    };
    assert_eq!(result.error, simdutf::ErrorCode::Success);
    assert_eq!(out_len, output.len());
    assert_eq!(&out_buf[..out_len], output);
}
