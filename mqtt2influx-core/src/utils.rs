use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_random_token(size: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(size).collect()
}

pub fn generate_random_number(min: usize, max: usize) -> usize {
    thread_rng().gen_range(min, max)
}
