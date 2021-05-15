use indoc::indoc;
use num_bigint::BigUint;

use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::macros::MacroAssign;
use crate::ast::hir::Hir;
use crate::ast::variant::UInt;
use crate::ast::verbs::OperatorVerb;
use crate::build::Builder;
use crate::errors::StdResult;
use crate::flags::CompileFlags;
use crate::types::LineNo;
use crate::utils::private_identifier;

pub(crate) fn box_ident(ident: String) -> Box<Hir> {
    Box::new(Hir::Expr(Expr::Ident(ident)))
}

// Macro expansion for x := y
pub(crate) fn lower_assign_to_ident(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Expr,
    rhs: &Expr,
) -> StdResult<Expr> {
    let lhs = match lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };
    let rhs = match rhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let instruction = format! { indoc! {"
        {} := {} + 0
        "}, lhs, rhs};

    Builder::ext_parse_and_compile(instruction.as_str(), context.clone(), Some(lno))
}

// Macro expansion for x := 0
pub(crate) fn lower_assign_to_zero(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Expr,
) -> StdResult<Expr> {
    let lhs = match lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let instruction = if context.flags.contains(CompileFlags::OPT_ZERO) {
        format!(
            indoc! {"
            {lhs} := _zero + 0;
            "},
            lhs = lhs
        )
    } else {
        format!(
            indoc! {"
            LOOP {lhs} DO
                {lhs} := {lhs} - 1
            END
            "},
            lhs = lhs
        )
    };

    Builder::ext_parse_and_compile(instruction.as_str(), context.clone(), Some(lno))
}

// Macro expansion for x := n
pub(crate) fn lower_assign_to_value(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Expr,
    rhs: &Expr,
) -> StdResult<Expr> {
    let lhs = match lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let rhs = match rhs.clone() {
        Expr::NaturalNumber(UInt(n)) => n,
        _ => unreachable!(),
    };

    let instruction = if context.flags.contains(CompileFlags::OPT_ZERO) {
        format!(
            indoc! {"
            {lhs} := _zero + {rhs}
            "},
            lhs = lhs,
            rhs = rhs.to_string()
        )
    } else {
        format!(
            indoc! {"
            {lhs} := 0
            {lhs} := {lhs} + {rhs}
            "},
            lhs = lhs,
            rhs = rhs.to_string()
        )
    };

    Builder::ext_parse_and_compile(instruction.as_str(), context.clone(), Some(lno))
}

// Macro expansion for x := y +/- z
fn expand_assign_to_ident_simple_ident(
    lno: LineNo,
    context: &CompileContext,
    x: String,
    y: String,
    op: OperatorVerb,
    z: String,
) -> StdResult<Expr> {
    let instruction = format!(
        indoc! {"
        {x} := {y}
        LOOP {b} DO
            {x} := {x} {op} 1
        END
        "},
        x = x,
        y = y,
        op = op.display(),
        b = z
    );

    Builder::ext_parse_and_compile(instruction.as_str(), context.clone(), Some(lno))
}

// Macro expansion for x := y * z
fn expand_assign_to_ident_mul_ident(
    lno: LineNo,
    context: &CompileContext,
    x: String,
    y: String,
    z: String,
) -> StdResult<Expr> {
    let instruction = format!(
        indoc! {"
        {x} := 0
        LOOP {y} DO
            {x} := {x} + {z}
        END
        "},
        x = x,
        y = y,
        z = z
    );

    Builder::ext_parse_and_compile(instruction.as_str(), context.clone(), Some(lno))
}

// Macro expansion for x := y (+|-|*) z
pub(crate) fn lower_assign_to_ident_binop_ident(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Expr,
    rhs: &MacroAssign,
) -> StdResult<Expr> {
    let lhs = match lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_lhs = match *rhs.lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_rhs = match *rhs.rhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_op = rhs.verb.clone();

    match binop_op {
        OperatorVerb::Multiply => {
            expand_assign_to_ident_mul_ident(lno, context, lhs, binop_lhs, binop_rhs)
        }
        OperatorVerb::Plus | OperatorVerb::Minus => {
            expand_assign_to_ident_simple_ident(lno, context, lhs, binop_lhs, binop_op, binop_rhs)
        }
    }
}

// Macro expansion for x := y * n
fn expand_assign_to_ident_mul_value(
    lno: LineNo,
    context: &mut CompileContext,
    x: String,
    y: String,
    n: BigUint,
) -> StdResult<Expr> {
    let tmp = private_identifier(context);

    let instruction = format!(
        indoc! {"
        {tmp} := {n}
        {x} := {y} * {tmp}
        "},
        x = x,
        y = y,
        n = n.to_string(),
        tmp = tmp
    );

    Builder::ext_parse_and_compile(instruction.as_str(), context.clone(), Some(lno))
}

// Macro expansion for x := y (*|...) n
pub(crate) fn lower_assign_to_ident_extbinop_value(
    lno: LineNo,
    context: &mut CompileContext,
    lhs: &Expr,
    rhs: &MacroAssign,
) -> StdResult<Expr> {
    let lhs = match lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_lhs = match *rhs.lhs.clone() {
        Expr::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_rhs = match *rhs.rhs.clone() {
        Expr::NaturalNumber(UInt(n)) => n,
        _ => unreachable!(),
    };

    let binop_op = rhs.verb.clone();

    match binop_op {
        OperatorVerb::Multiply => {
            expand_assign_to_ident_mul_value(lno, context, lhs, binop_lhs, binop_rhs)
        }
        _ => unreachable!(),
    }
}
