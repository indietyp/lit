use rand::Rng;

use crate::ast::context::CompileContext;

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
