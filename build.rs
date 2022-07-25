fn main() {
    let mut cc = cc::Build::new();

    cc.cpp(true).file("cpp/simdutfrs.cpp");

    if cfg!(all(windows, target_env = "msvc")) {
        cc.flag("/std:c++11");
    } else {
        cc.flag("-std=c++11");
    }

    cc.compile("simdutfrs");
}
