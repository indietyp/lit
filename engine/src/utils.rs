use either::Either;
use itertools::Itertools;

use crate::ast::context::CompileContext;
use crate::errors::Error;

pub fn private_identifier(context: &mut CompileContext) -> String {
    let mut id = String::new();
    id.push('_');
    id.push_str(context.incr().to_string().as_str());

    id
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn check_errors<T>(maybe: &[Result<T, Vec<Error>>]) -> Result<Vec<T>, Vec<Error>> {
    let (ok, err): (Vec<_>, Vec<_>) = maybe.iter().partition_map(|r| match r {
        Ok(r) => Either::Left(r.clone()),
        Err(r) => Either::Right(r.clone()),
    });
    let err: Vec<_> = err.iter().flat_map(|f| f.clone()).collect_vec();

    if !err.is_empty() {
        Err(err)
    } else {
        Ok(ok)
    }
}
