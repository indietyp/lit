use crate::ast::context::CompileContext;
use rand::Rng;

pub fn private_identifier(context: &mut CompileContext) -> String {
    let mut rng = rand::thread_rng();
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
