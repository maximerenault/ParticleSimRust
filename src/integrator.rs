use crate::particle::Particle;

pub const EULER: i32 = 0;
pub const LEAPFROG: i32 = 1;
pub const RK4: i32 = 2;
pub const MIDPOINT: i32 = 3;

pub fn time_integration(particle: &mut Particle, force: &[f64; 2], dt: f64, integrator_type: i32) {
    if integrator_type == EULER {
        euler_step(particle, *force, dt);
    } else if integrator_type == LEAPFROG {
        leapfrog_step(particle, *force, dt);
    } else if integrator_type == RK4 {
        rk4_step(particle, *force, dt);
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

fn rk4_step(p: &mut Particle, force: [f64; 2], dt: f64) {
    let half_step = 0.5 * dt;

    let k1_v = force;
    let k1_p = p.velocity;

    let k2_v = [
        force[0] + k1_v[0] * half_step / p.mass,
        force[1] + k1_v[1] * half_step / p.mass,
    ];
    let k2_p = [
        p.velocity[0] + k1_v[0] * half_step,
        p.velocity[1] + k1_v[1] * half_step,
    ];

    let k3_v = [
        force[0] + k2_v[0] * half_step / p.mass,
        force[1] + k2_v[1] * half_step / p.mass,
    ];
    let k3_p = [
        p.velocity[0] + k2_v[0] * half_step,
        p.velocity[1] + k2_v[1] * half_step,
    ];

    let k4_v = [
        force[0] + k3_v[0] * dt / p.mass,
        force[1] + k3_v[1] * dt / p.mass,
    ];
    let k4_p = [p.velocity[0] + k3_v[0] * dt, p.velocity[1] + k3_v[1] * dt];

    p.position[0] += (k1_p[0] + 2.0 * k2_p[0] + 2.0 * k3_p[0] + k4_p[0]) * dt / 6.0;
    p.position[1] += (k1_p[1] + 2.0 * k2_p[1] + 2.0 * k3_p[1] + k4_p[1]) * dt / 6.0;
    p.velocity[0] += (k1_v[0] + 2.0 * k2_v[0] + 2.0 * k3_v[0] + k4_v[0]) * dt / (6.0 * p.mass);
    p.velocity[1] += (k1_v[1] + 2.0 * k2_v[1] + 2.0 * k3_v[1] + k4_v[1]) * dt / (6.0 * p.mass);
}

fn leapfrog_step(p: &mut Particle, force: [f64; 2], dt: f64) {
    p.velocity[0] += force[0] * dt / (2.0 * p.mass);
    p.velocity[1] += force[1] * dt / (2.0 * p.mass);

    p.position[0] += p.velocity[0] * dt;
    p.position[1] += p.velocity[1] * dt;

    p.velocity[0] += force[0] * dt / (2.0 * p.mass);
    p.velocity[1] += force[1] * dt / (2.0 * p.mass);
}
