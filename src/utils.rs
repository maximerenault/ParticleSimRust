use crate::particle::Particle;
use rand::Rng;

pub fn generate_random_particles(n: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Particle {
            position: [rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0)],
            velocity: [rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)],
            mass: rng.gen_range(1.0e3..1.0e4),
        })
        .collect()
}

pub fn generate_random_particles_around_attractor(n: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = (0..n-1)
        .map(|_| Particle {
            position: [rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0)],
            velocity: [rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)],
            mass: rng.gen_range(1.0e4..1.0e5),
        })
        .collect();

    let attractor = Particle {
        position: [400.0, 300.0],
        velocity: [0.0, 0.0],
        mass: 1.0e7,
    };
    particles.push(attractor);

    particles
}