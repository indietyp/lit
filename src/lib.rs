use rand::{random, Rng};

pub fn random_identifier() -> String {
    let mut rng = rand::thread_rng();
    let mut id = String::new();
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
