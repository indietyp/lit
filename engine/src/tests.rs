use crate::ast::hir::func::fs::Directory;
use crate::build::Builder;
use crate::errors::ErrorCode::FunctionUnexpectedNumberOfArguments;
use crate::eval::types::Variables;
use crate::flags::CompileFlags;

use indoc::indoc;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
enum ErrorCode {
    StepLimitExceeded,
}

#[allow(dead_code)]
fn run(
    snip: &str,
    limit: Option<usize>,
    locals: Option<Variables>,
    flags: Option<CompileFlags>,
    fs: Option<Directory>,
) -> Result<Variables, ErrorCode> {
    let maybe_exec = Builder::ext_all(snip, flags, locals, fs);
    assert!(
        maybe_exec.is_ok(),
        "While creating the parser errors occurred: {:?}",
        maybe_exec.err().unwrap()
    );
    let mut exec = maybe_exec.ok().unwrap();

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

#[allow(dead_code)]
fn assert_is_int(value: Option<&BigUint>, expect: usize) {
    assert_is_biguint(value, BigUint::from(expect))
}

#[allow(dead_code)]
fn assert_is_biguint(value: Option<&BigUint>, expect: BigUint) {
    assert!(value.is_some());
    assert!(
        value.unwrap().eq(&expect),
        "Expected evaluated value to be {}, was {}",
        expect.to_string(),
        value.unwrap()
    );
}

#[allow(dead_code)]
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

    let result = run(snip, Some(50), None, None, None);
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

    let result = run(snip, Some(50), None, None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 10);

    let ast = Builder::parse_and_compile(
        snip,
        Some(CompileFlags::WHILE | CompileFlags::CNF_RETAIN_LNO),
        None,
    )
    .unwrap();

    assert_eq!(
        ast.display(4, None),
        indoc! {"
        _0 := x + 0
        
        WHILE _0 != 0 DO
            x := x + 1
            _0 := _0 - 1
        END"}
    )
}

#[test]
fn test_cond_not_zero_skip() {
    let zero = BigUint::zero();
    let snip = indoc! {"
    IF x != 0 THEN
        y := y + 1
    END
    "};

    let result = run(snip, Some(50), None, None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y").or(Some(&zero));

    assert_is_int(y, 0);
}

#[test]
fn test_cond_not_zero() {
    let snip = indoc! {"
    IF x != 0 THEN
        y := y + 1
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(8u8));

    let result = run(snip, Some(50), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");

    assert_is_int(y, 1)
}

#[test]
fn test_cond_not_zero_else() {
    let snip = indoc! {"
    IF x != 0 THEN
        ...
    ELSE
        y := y + 1
    END
    "};

    let result = run(snip, Some(50), None, None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");

    assert_is_int(y, 1)
}

#[test]
fn test_cond_not_zero_val() {
    let snip = indoc! {"
    IF 1 != 0 THEN
        y := y + 1
    END
    "};

    let result = run(snip, Some(50), None, None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");

    assert_is_int(y, 1)
}

#[test]
fn test_cond_gt_if() {
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

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let z = locals.get("z");

    assert_is_int(z, 1)
}

#[test]
fn test_cond_gt_else() {
    // we only need to check this once, they all call the same method gt, so this works
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

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let z = locals.get("z");

    assert_is_int(z, 2)
}

#[test]
fn test_cond_gte_val() {
    let zero = BigUint::zero();
    let snip = indoc! {"
    IF x > 2 THEN
        a := 1
    END

    IF 3 > 2 THEN
        b := 1
    END

    IF 1 > 2 THEN
        c := 1
    END

    IF 4 > x THEN
        d := 1
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(3u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();

    let a = locals.get("a");
    assert_is_int(a, 1);

    let b = locals.get("b");
    assert_is_int(b, 1);

    let c = locals.get("c").or(Some(&zero));
    assert_is_int(c, 0);

    let d = locals.get("d").or(Some(&zero));
    assert_is_int(d, 1);
}

#[test]
fn test_cond_gte_if() {
    let snip = indoc! {"
    IF x >= y THEN
        a := 1
    END

    IF x >= z THEN
        b := 1
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(2u8));
    locals.insert("y".to_string(), BigUint::from(2u8));
    locals.insert("z".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");
    let b = locals.get("b");

    assert_is_int(a, 1);
    assert_is_int(b, 1);
}

#[test]
fn test_conf_lt_if() {
    let zero = BigUint::zero();
    let snip = indoc! {"
    IF x < y THEN
        a := 1
    ELSE
        b := 1
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(1u8));
    locals.insert("y".to_string(), BigUint::from(2u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");
    let b = locals.get("b").or(Some(&zero));

    assert_is_int(a, 1);
    assert_is_int(b, 0);
}

#[test]
fn test_conf_lte_if() {
    let snip = indoc! {"
    IF x <= y THEN
        a := 1
    END

    IF x <= z THEN
        b := 1
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(1u8));
    locals.insert("y".to_string(), BigUint::from(2u8));
    locals.insert("z".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");
    let b = locals.get("b");

    assert_is_int(a, 1);
    assert_is_int(b, 1);
}

#[test]
fn test_cond_simulated_eq() {
    let snip = indoc! {"
    IF x <= y THEN
        IF x >= y THEN
           a := 1
        END
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(1u8));
    locals.insert("y".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");

    assert_is_int(a, 1);
}

#[test]
fn test_cond_eq() {
    let snip = indoc! {"
    IF x == y THEN
        a := 1
    ELSE
        a := 2
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(1u8));
    locals.insert("y".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");

    assert_is_int(a, 1);

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(2u8));
    locals.insert("y".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");

    assert_is_int(a, 2);
}

#[test]
fn test_cond_neq() {
    let snip = indoc! {"
    IF x != y THEN
        a := 1
    ELSE
        a := 2
    END
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(1u8));
    locals.insert("y".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");

    assert_is_int(a, 2);

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(2u8));
    locals.insert("y".to_string(), BigUint::from(1u8));

    let result = run(snip, Some(150), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let a = locals.get("a");

    assert_is_int(a, 1);
}

#[test]
fn test_macro_ident_mul_ident() {
    let snip = indoc! {"
    x := y * z
    "};

    let mut locals = HashMap::new();
    locals.insert("y".to_string(), BigUint::from(2u8));
    locals.insert("z".to_string(), BigUint::from(3u8));

    let result = run(snip, Some(50), Some(locals), None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
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

    let result = run(snip, Some(50), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let x = locals.get("x");

    assert_is_int(x, 6);
}

#[test]
fn test_const() {
    let snip = indoc! {"
    _zero := 1
    "};

    let result = Builder::parse(snip, None);
    assert!(result.is_ok());
    let mut result = result.ok().unwrap();

    let result = Builder::compile(
        &mut result,
        Some(CompileFlags::CNF_CONST | CompileFlags::LOOP | CompileFlags::WHILE),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn test_call_param_error() {
    let snip = indoc! {"
    FN a(b, c) -> d DECL
        ...
    END

    a := a(1, 2)
    a := a(1)
    a := a(1, 2, 3)
    "};

    let maybe_runtime = Builder::ext_all(snip, None, None, None);
    let errors = maybe_runtime.err().expect("Expected Error");
    assert_eq!(errors.len(), 2);

    let expected = vec![
        crate::errors::Error {
            lno: (6, 6),
            variant: crate::errors::ErrorVariant::ErrorCode(FunctionUnexpectedNumberOfArguments {
                module: "fs::main".to_string(),
                func: "a".to_string(),
                expected: 2,
                got: 1,
            }),
        },
        crate::errors::Error {
            lno: (7, 7),
            variant: crate::errors::ErrorVariant::ErrorCode(FunctionUnexpectedNumberOfArguments {
                module: "fs::main".to_string(),
                func: "a".to_string(),
                expected: 2,
                got: 3,
            }),
        },
    ];

    assert!(errors.eq(&expected));
}

#[test]
fn test_lazy_eval() {
    let snip = indoc! {"
    FN a(b) -> c DECL
        _ := a(b)
    END
    "};

    let maybe_runtime = Builder::ext_all(snip, None, None, None);
    assert!(maybe_runtime.is_ok());
}

#[test]
fn test_nested_recursion() {
    let snip = indoc! {"
    FN a(b) -> c DECL
        _ := b(b)
    END
    FN b(b) -> c DECL
        _ := c(b)
    END
    FN c(b) -> c DECL
        _ := a(b)
    END

    x := a(2)
    "};

    let maybe_runtime = Builder::ext_all(snip, None, None, None);
    let errors = maybe_runtime.err().expect("Expected Error");
    assert_eq!(errors.len(), 1);
    let error = errors
        .first()
        .expect("Expected at least one error, got none?");

    let (stack, module, func, count) = match error.clone().variant {
        crate::errors::ErrorVariant::ErrorCode(
            crate::errors::ErrorCode::FunctionRecursionDetected {
                stack,
                module,
                func,
                count,
            },
        ) => (stack, module, func, count),
        _ => panic!("Error war supossed to be Recursion Error!"),
    };

    assert_eq!(stack.len(), 4);
    assert_eq!(
        stack,
        vec!["fs::main::a", "fs::main::b", "fs::main::c", "fs::main::a"]
    );
    assert_eq!(module, "fs::main");
    assert_eq!(func, "a");
    assert_eq!(count, Some(2));
}

#[test]
fn test_inline_simple() {
    let snip = indoc! {"
    from std::prelude import max

    y := max(x, 3)
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(2u8));

    let result = run(snip, Some(50), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");

    assert_is_int(y, 3);
}

#[test]
fn test_inline_nested() {
    let snip = indoc! {"
    from std::prelude import max

    fn maxAddN(x, y, n) -> z decl
        maxVal := max(x, y)
        z := maxVal + n
    end

    y := maxAddN(x, y, 3)
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(2u8));
    locals.insert("y".to_string(), BigUint::from(4u8));

    let result = run(snip, Some(50), Some(locals), None, None);
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");

    assert_is_int(y, 7);
}

#[test]
fn test_inline_across_fs() {
    let module_a = indoc! {"
    from std::prelude import max

    fn maxSubN(x, y, n) -> z decl
        maxVal := max(x, y)
        z := maxVal - n
    end         
    "};

    let snip = indoc! {"
    from fs::a import *

    z := max(x, y)
    y := maxSubN(x, y, 3)
    "};

    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::from(2u8));
    locals.insert("y".to_string(), BigUint::from(4u8));

    let mut dir = Directory::new();
    dir.insert("a".into(), module_a.into());

    let result = run(snip, Some(150), Some(locals), None, Some(dir));
    assert_result_ok(&result);

    let locals = result.ok().unwrap();
    let y = locals.get("y");
    let z = locals.get("z");

    assert_is_int(y, 1);
    assert_is_int(z, 4);
}

// This is a special tests, that looks what the LIPS count is.
#[test]
#[ignore]
fn test_speed() {
    let snip = indoc! {"
    WHILE x != 0 DO
        y := y + 1
    END
    "};
    let mut locals = HashMap::new();
    locals.insert("x".to_string(), BigUint::one());

    let mut exec = Builder::ext_all(snip, None, Some(locals), None).unwrap();
    let start = SystemTime::now();
    let limit = start + Duration::new(5, 0);

    let mut steps: usize = 0;
    while SystemTime::now() < limit {
        exec.step();
        steps += 1;
    }
    let diff = SystemTime::now().duration_since(start).unwrap();

    println!("LIPS: {}", steps as f64 / diff.as_secs_f64())
}
