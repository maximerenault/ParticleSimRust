use crate::particle::Particle;
use rand::Rng;

pub fn generate_random_particles(n: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            Particle::new(
                [rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)],
                [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
                rng.gen_range(1.0..10.0),
            )
        })
        .collect()
}
