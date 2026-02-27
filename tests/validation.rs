#![cfg(feature = "ascii")]

#[test]
fn ascii() {
    let cases: &[&[u8]] = &[
        "hello".as_bytes(),  //
        &[0xff, 0xff, 0xff], //
    ];

    for s in cases {
        let output = simdutf::validate_ascii(s);
        let expected = s.is_ascii();
        assert_eq!(output, expected);
    }
}
