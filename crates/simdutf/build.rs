fn cpp11(cc: &mut cc::Build) {
    cc.cpp(true);
    if cfg!(not(all(windows, target_env = "msvc"))) {
        cc.flag("-std=c++11");
    }
}

fn main() {
    let mut cc = cc::Build::new();
    cpp11(&mut cc);
    cc.file("cpp/simdutfrs.cpp").compile("simdutfrs");
}
