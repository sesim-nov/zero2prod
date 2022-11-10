use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_token() -> String {
    let rng = thread_rng();
    rng.sample_iter(Alphanumeric)
        .map(char::from)
        .take(25)
        .collect()
}
