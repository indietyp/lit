use rand::Rng;

use crate::ast::context::CompileContext;

pub fn private_identifier(context: &mut CompileContext) -> String {
    let mut id = String::new();
    id.push('_');
    id.push_str(context.incr().to_string().as_str());

    id.clone()
}

pub fn private_random_identifier() -> String {
    let mut rng = rand::thread_rng();
    let mut id = String::new();
    id.push('_');

    for _ in 0..10 {
        // we cannot do continuous range, because there are several characters between uppercase and lowercase
        if rng.gen_bool(0.5) {
            id.push(rng.gen_range('A'..='Z'))
        } else {
            id.push(rng.gen_range('a'..='z'))
        }
    }

    id.clone()
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
