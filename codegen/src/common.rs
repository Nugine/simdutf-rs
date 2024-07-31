pub const ENCODINGS: [&str; 7] = ["ascii", "utf8", "utf16", "utf16be", "utf16le", "utf32", "latin1"];

pub fn map_cpp_char_type(encoding: &str) -> &str {
    match encoding {
        "ascii" => "char",
        "utf8" => "char",
        "utf16" => "char16_t",
        "utf16be" => "char16_t",
        "utf16le" => "char16_t",
        "utf32" => "char32_t",
        "latin1" => "char",
        _ => unimplemented!(),
    }
}

pub fn map_rs_char_type(encoding: &str) -> &str {
    match encoding {
        "ascii" => "u8",
        "utf8" => "u8",
        "utf16" => "u16",
        "utf16be" => "u16",
        "utf16le" => "u16",
        "utf32" => "u32",
        "latin1" => "u8",
        _ => unimplemented!(),
    }
}

pub fn map_doc_name(encoding: &str) -> &str {
    match encoding {
        "ascii" => "ASCII",
        "utf8" => "UTF-8",
        "utf16" => "UTF-16",
        "utf16be" => "UTF-16BE",
        "utf16le" => "UTF-16LE",
        "utf32" => "UTF-32",
        "latin1" => "Latin1",
        _ => unimplemented!(),
    }
}

pub fn for_each_validate(mut f: impl FnMut(&str)) {
    for encoding in ENCODINGS {
        if matches!(encoding, "latin1") {
            continue;
        }
        f(encoding);
    }
}

pub fn for_each_count(mut f: impl FnMut(&str)) {
    for encoding in ENCODINGS {
        if matches!(encoding, "ascii" | "utf32" | "latin1") {
            continue;
        }
        f(encoding);
    }
}

pub fn for_each_transcoding_length(mut f: impl FnMut(&str, &str)) {
    for to in ENCODINGS {
        for from in ENCODINGS {
            if from == "ascii" || to == "ascii" {
                continue;
            }
            if from == "latin1" || to == "latin1" {
                continue;
            }
            if from == to {
                continue;
            }
            if matches!(to, "utf16le" | "utf16be") {
                continue;
            }
            if from.starts_with("utf16") && to.starts_with("utf16") {
                continue;
            }
            f(from, to)
        }
    }
}

pub fn for_each_transcoding_convert(mut f: impl FnMut(&str, &str)) {
    for from in ENCODINGS {
        for to in ENCODINGS {
            if from == "ascii" || to == "ascii" {
                continue;
            }
            if from == to {
                continue;
            }
            if from.starts_with("utf16") && to.starts_with("utf16") {
                continue;
            }
            f(from, to)
        }
    }
}
