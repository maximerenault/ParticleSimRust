use crate::particle::Particle;
use rand::Rng;

pub fn generate_random_particles(n: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Particle {
            position: [rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0)],
            velocity: [
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            ],
            mass: rng.gen_range(1.0e3..1.0e4),
        })
        .collect()
}
