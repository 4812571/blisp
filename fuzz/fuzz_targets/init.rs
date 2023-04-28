#![no_main]

use libfuzzer_sys::fuzz_target;
use blisp;

fuzz_target!(|data: &[u8]| {
    let code = String::from_utf8_lossy(data);

    let cleaned_code = code.chars().filter(|c| c.is_ascii()).collect::<String>();

    let _ = blisp::init(&cleaned_code, vec![]);
});