use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_random_token(size: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(size).collect()
}
