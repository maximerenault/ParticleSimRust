use crate::forces::GRAVIT_CONST;
use crate::particle::Particle;
use rand::Rng;

pub fn generate_random_particles(n: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Particle {
            position: [rng.gen_range(0.0..1500.0), rng.gen_range(0.0..900.0)],
            velocity: [rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0)],
            mass: rng.gen_range(10.0..100.0),
        })
        .collect()
}

pub fn generate_random_particles_around_attractor(n: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = Vec::with_capacity(n);

    let attractor_position = [750.0, 450.0];
    let attractor_mass = 1.0e6;
    let attractor = Particle {
        position: attractor_position,
        velocity: [0.0, 0.0],
        mass: attractor_mass,
    };

    for _ in 0..n - 1 {
        // Random position around the attractor
        let radius = rng.gen_range(50.0..700.0);
        let angle = rng.gen_range(0.0..std::f64::consts::TAU);
        let position = [
            attractor_position[0] + radius * angle.cos(),
            attractor_position[1] + radius * angle.sin(),
        ];

        // Circular orbital velocity sqrt(G * M / r)
        let velocity_magnitude = (GRAVIT_CONST * attractor_mass / radius).sqrt();
        let velocity_direction = [-angle.sin(), angle.cos()];
        let random_perturbation = [rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)];
        let velocity = [
            velocity_magnitude * (velocity_direction[0] + random_perturbation[0]),
            velocity_magnitude * (velocity_direction[1] + random_perturbation[1]),
        ];

        let mass = rng.gen_range(0.1..10.0);

        particles.push(Particle {
            position,
            velocity,
            mass,
        });
    }

    particles.push(attractor);

    particles
}
