fn cpp11(cc: &mut cc::Build) {
    cc.cpp(true);
    if cfg!(not(all(windows, target_env = "msvc"))) {
        cc.flag("-std=c++11");
    }
}

fn feature_define(cc: &mut cc::Build, feature: &str, define: &str) {
    let key = format!("CARGO_FEATURE_{}", feature.to_uppercase());
    let val = if std::env::var_os(&key).is_some() { "1" } else { "0" };
    cc.define(define, val);
}

fn main() {
    println!("cargo:rerun-if-changed=cpp/");

    let mut cc = cc::Build::new();
    cpp11(&mut cc);

    feature_define(&mut cc, "DETECT_ENCODING", "SIMDUTF_FEATURE_DETECT_ENCODING");
    feature_define(&mut cc, "ASCII", "SIMDUTF_FEATURE_ASCII");
    feature_define(&mut cc, "LATIN1", "SIMDUTF_FEATURE_LATIN1");
    feature_define(&mut cc, "UTF8", "SIMDUTF_FEATURE_UTF8");
    feature_define(&mut cc, "UTF16", "SIMDUTF_FEATURE_UTF16");
    feature_define(&mut cc, "UTF32", "SIMDUTF_FEATURE_UTF32");
    feature_define(&mut cc, "BASE64", "SIMDUTF_FEATURE_BASE64");

    cc.file("cpp/simdutfrs.cpp").compile("simdutfrs");
}
