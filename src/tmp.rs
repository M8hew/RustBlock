use rand::distributions::Uniform;
use rand::Rng;

pub fn rand_time(from: f64, to: f64) -> f64 {
    let urand = Uniform::new(from, to);
    let mut rng = rand::thread_rng();
    rng.sample(urand)
}
