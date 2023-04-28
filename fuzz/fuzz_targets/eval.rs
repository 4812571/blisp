#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use blisp;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    code: String,
    expr: String,
}

fuzz_target!(|data: &[u8]| {
    let unstructured = &mut Unstructured::new(data);
    
    let input: FuzzInput = match Arbitrary::arbitrary(unstructured) {
        Ok(input) => input,
        Err(_) => return,
    };

    let cleaned_code = input.code.chars().filter(|c| c.is_ascii()).collect::<String>();
    let cleaned_expr = input.expr.chars().filter(|c| c.is_ascii()).collect::<String>();

    let exprs = match blisp::init(&cleaned_code, vec![]) {
        Ok(exprs) => exprs,
        Err(_) => return,
    };

    let ctx = match blisp::typing(exprs) {
        Ok(ctx) => ctx,
        Err(_) => return,
    };

    let _ = blisp::eval(&cleaned_expr, &ctx);
});