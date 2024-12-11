use crate::particle::Particle;

pub const EULER: i32 = 0;
pub const LEAPFROG: i32 = 1;
pub const MIDPOINT: i32 = 2;

pub fn time_integration(particle: &mut Particle, force: &[f64; 2], dt: f64, integrator_type: i32) {
    if integrator_type == EULER {
        euler_step(particle, *force, dt);
    } else if integrator_type == LEAPFROG {
        leapfrog_step(particle, *force, dt);
    } else if integrator_type == MIDPOINT {
        midpoint_step(particle, *force, dt);
    }
}

fn euler_step(p: &mut Particle, force: [f64; 2], dt: f64) {
    p.velocity[0] += force[0] / p.mass * dt;
    p.velocity[1] += force[1] / p.mass * dt;

    p.position[0] += p.velocity[0] * dt;
    p.position[1] += p.velocity[1] * dt;
}

fn midpoint_step(p: &mut Particle, force: [f64; 2], dt: f64) {
    let half_step = 0.5 * dt;
    let mid_velocity = [
        p.velocity[0] + force[0] * half_step / p.mass,
        p.velocity[1] + force[1] * half_step / p.mass,
    ];

    p.position[0] += mid_velocity[0] * dt;
    p.position[1] += mid_velocity[1] * dt;
    p.velocity[0] += force[0] * dt / p.mass;
    p.velocity[1] += force[1] * dt / p.mass;
}

fn leapfrog_step(p: &mut Particle, force: [f64; 2], dt: f64) {
    p.velocity[0] += force[0] * dt / (2.0 * p.mass);
    p.velocity[1] += force[1] * dt / (2.0 * p.mass);

    p.position[0] += p.velocity[0] * dt;
    p.position[1] += p.velocity[1] * dt;

    p.velocity[0] += force[0] * dt / (2.0 * p.mass);
    p.velocity[1] += force[1] * dt / (2.0 * p.mass);
}
