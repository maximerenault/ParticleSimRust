use crate::forces::GRAVIT_CONST;
use crate::particle::Particle;

#[derive(Debug)]
pub struct QuadTree {
    boundary: [f64; 4], // [x_min, y_min, x_max, y_max]
    total_mass: f64,
    center_of_mass: [f64; 2],
    finalized: bool,
    particle: Option<Particle>,
    children: Option<Box<[QuadTree; 4]>>, // 4 children for 2D quadtree
}

impl QuadTree {
    pub fn new(boundary: [f64; 4]) -> Self {
        QuadTree {
            boundary,
            total_mass: 0.0,
            center_of_mass: [0.0, 0.0],
            finalized: false,
            particle: None,
            children: None,
        }
    }
}

impl QuadTree {
    pub fn insert(&mut self, particle: Particle) -> bool {
        // Check if the particle is out of bounds
        if !self.contains(&particle) {
            return false;
        }

        // If the node is empty, insert the particle
        if self.particle.is_none() && self.children.is_none() {
            self.particle = Some(particle);
            self.add_mass(particle);
            return true;
        }

        // If the node is already subdivided, pass the particle to the children
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.insert(particle) {
                    self.add_mass(particle);
                    return true;
                }
            }
        }

        // If the node contains a particle, subdivide and redistribute
        if self.particle.is_some() {
            self.subdivide();
            let existing_particle = self.particle.take().unwrap();
            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.insert(existing_particle) {
                    break;
                }
            }
            return self.insert(particle); // Try to insert the new particle again
        }

        false
    }

    fn contains(&self, particle: &Particle) -> bool {
        let [x_min, y_min, x_max, y_max] = self.boundary;
        (particle.position[0] >= x_min)
            && (particle.position[0] <= x_max)
            && (particle.position[1] >= y_min)
            && (particle.position[1] <= y_max)
    }

    fn subdivide(&mut self) {
        let [x_min, y_min, x_max, y_max] = self.boundary;
        let mid_x = (x_min + x_max) / 2.0;
        let mid_y = (y_min + y_max) / 2.0;

        self.children = Some(Box::new([
            QuadTree::new([x_min, y_min, mid_x, mid_y]),
            QuadTree::new([mid_x, y_min, x_max, mid_y]),
            QuadTree::new([x_min, mid_y, mid_x, y_max]),
            QuadTree::new([mid_x, mid_y, x_max, y_max]),
        ]));
    }

    fn add_mass(&mut self, particle: Particle) {
        self.center_of_mass[0] += particle.mass * particle.position[0];
        self.center_of_mass[1] += particle.mass * particle.position[1];
        self.total_mass += particle.mass;
    }

    pub fn finalize(&mut self) {
        if self.total_mass != 0.0 {
            self.center_of_mass[0] /= self.total_mass;
            self.center_of_mass[1] /= self.total_mass;
        }

        if !self.children.is_none() {
            for child in self.children.as_mut().unwrap().iter_mut() {
                child.finalize();
            }
        }

        self.finalized = true;
    }

    pub fn compute_force(&self, particle: &Particle, theta: f64) -> [f64; 2] {
        if !self.finalized {
            panic!("QuadTree was not finalized!");
        }

        if self.total_mass == 0.0 {
            return [0.0, 0.0];
        }

        let dx = self.center_of_mass[0] - particle.position[0];
        let dy = self.center_of_mass[1] - particle.position[1];
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt();

        // If the node is far enough, use approximation
        if dist > 0.0 && (self.boundary[2] - self.boundary[0]) / dist < theta {
            let dist_sq = dist_sq.max(1.0);
            let force = GRAVIT_CONST * self.total_mass * particle.mass / dist_sq;
            return [force * dx / dist, force * dy / dist];
        }

        // Otherwise, traverse into children
        if let Some(children) = &self.children {
            let mut total_force = [0.0, 0.0];
            for child in children.iter() {
                let child_force = child.compute_force(particle, theta);
                total_force[0] += child_force[0];
                total_force[1] += child_force[1];
            }
            return total_force;
        }

        [0.0, 0.0]
    }
}
