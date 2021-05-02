use crate::build::Builder;
use crate::eval::types::Variables;
use crate::flags::CompilationFlags;

use indoc::indoc;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ErrorCode {
    StepLimitExceeded,
}

fn run(
    snip: &str,
    limit: Option<usize>,
    locals: Option<Variables>,
    flags: Option<CompilationFlags>,
) -> Result<Variables, ErrorCode> {
    let mut exec = Builder::all2(snip, flags, locals);

    let mut steps: usize = 0;
    while exec.is_running() && (limit.map(|l| steps < l).unwrap_or(true)) {
        exec.step();

        steps += 1;
    }

    if limit.is_some() && steps >= limit.unwrap() {
        Err(ErrorCode::StepLimitExceeded)
    } else {
        Ok(exec.context())
    }
}

fn assert_is_int(value: Option<&BigUint>, expect: usize) {
    assert_is_biguint(value, BigUint::from(expect))
}

fn assert_is_biguint(value: Option<&BigUint>, expect: BigUint) {
    assert!(value.is_some());
    assert!(
        value.unwrap().eq(&expect),
        "Expected evaluated value to be {}, was {}",
        expect.to_string(),
        value.unwrap()
    );
}

fn assert_result_ok(result: &Result<Variables, ErrorCode>) {
    assert!(
        result.is_ok(),
        "run() helper returned ErrorCode {:?}",
        result.clone().err().unwrap()
    )
}

#[test]
fn test_simple_assign() {
    let snip = indoc! {"
    x := x + 1
    "};

    let result = run(snip, Some(50), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 1);
}

#[test]
fn test_macro_assign() {
    let snip = indoc! {"
    x := 5
    "};

    let result = run(snip, Some(50), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 5);
}

#[test]
fn test_initial_values() {
    let snip = indoc! {"
    x := x + 1
    "};
    let mut locals: Variables = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(5u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 6);
}

#[test]
fn test_while() {
    let snip = indoc! {"
    WHILE x != 0 DO
        x := x - 1
    END
    "};

    let mut locals: Variables = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(5u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 0);
}

#[test]
fn test_loop() {
    let snip = indoc! {"
    LOOP y DO
        x := x + 1
    END
    "};

    let mut locals: Variables = HashMap::new();
    locals.insert("y".to_string(), BigUint::from(5u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 5)
}

#[test]
fn test_compressed_compressed() {
    let snip = "LOOP y DO;x:=x+1;END";

    let mut locals: Variables = HashMap::new();
    locals.insert("y".to_string(), BigUint::from(5u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 5)
}

#[test]
fn test_decompile() {
    let snip = indoc! {"
    LOOP x DO
        x := x + 1
    END
    "};

    let mut locals: Variables = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(5u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 10);

    let ast = Builder::parse_and_compile(
        snip,
        Some(CompilationFlags::WHILE | CompilationFlags::RETAIN_LNO),
    );

    assert_eq!(
        ast.display(4, None),
        indoc! {"\
        _0 := x + 0
        WHILE _0 != 0 DO
            x := x + 1
            _0 := _0 - 1
        END"}
    )
}

#[test]
fn test_if_not_zero_body_skip() {
    let zero = BigUint::zero();
    let snip = indoc! {"
    IF x != 0 THEN
        y := y + 1
    END
    "};

    let result = run(snip, Some(50), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y").or(Some(&zero));

    assert_is_int(y, 0);
}

#[test]
fn test_if_not_zero_exec() {
    let snip = indoc! {"
    IF x != 0 THEN
        y := y + 1
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(8u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");

    assert_is_int(y, 1)
}

#[test]
fn test_if_else_else_body() {
    let snip = indoc! {"
    IF x > y THEN
        z := 1
    ELSE
        z := 2
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(16u8));
    locals.insert("y".to_string(), BigUint::from(32u8));

    let result = run(snip, Some(150), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let z = locals.get("z");

    assert_is_int(z, 2)
}

#[test]
fn test_if_else_if_body() {
    let snip = indoc! {"
    IF x > y THEN
        z := 1
    ELSE
        z := 2
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(32u8));
    locals.insert("y".to_string(), BigUint::from(16u8));

    let result = run(snip, Some(150), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let z = locals.get("z");

    assert_is_int(z, 1)
}

#[test]
fn test_macro_ident_mul_ident() {
    let snip = indoc! {"
    x := y * z
    "};

    let mut locals = HashMap::new();
    locals.insert("y".to_string(), BigUint::from(2u8));
    locals.insert("z".to_string(), BigUint::from(3u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 6);
}

#[test]
fn test_macro_ident_add_ident() {
    let snip = indoc! {"
    x := y + z
    "};

    let mut locals = HashMap::new();
    locals.insert("y".to_string(), BigUint::from(2u8));
    locals.insert("z".to_string(), BigUint::from(3u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 5);
}

#[test]
fn test_macro_ident_mul_val() {
    let snip = indoc! {"
    x := y * 3
    "};

    let mut locals = HashMap::new();
    locals.insert("y".to_string(), BigUint::from(2u8));

    let result = run(snip, Some(50), Some(locals), None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 6);
}
