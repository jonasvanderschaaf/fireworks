use colour::random_colour;
use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::super::colour;
use super::super::colour::Colour;
use super::super::sim::{Particle, TwoVec};
use super::{Firework, Rocket, GRAVITY, PARTICLE_COUNT, PARTICLE_LIFETIME};

use super::vel_min_max;

pub struct StandardFirework {
    rocket: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    colour: Colour,
    lifetime: u32,
}

impl Rocket for StandardFirework {
    fn rocket_mut(&mut self) -> &mut Particle {
        &mut self.rocket
    }

    fn rocket(&self) -> &Particle {
        &self.rocket
    }

    fn exploded(&self) -> bool {
        self.exploded
    }

    fn explode(&mut self) -> () {
        self.exploded = true;

        /* Create the explosion. */
        for _ in 0..PARTICLE_COUNT {
            let particle =
                Particle::random_at(self.rocket.pos().clone(), 2. + Math::random() * 0.5);
            self.particles.push(particle);
        }
    }

    fn sim_explosion(&mut self, width: u32, height: u32) -> () {
        self.particles.iter_mut().for_each(|particle| {
            particle.apply_force(GRAVITY);
            particle.step();
        });

        self.lifetime -= 1;

        if self.lifetime == 0 {
            self.reset(width, height);
        }
    }

    fn draw_explosion(&self, context: &CanvasRenderingContext2d) -> () {
        for particle in &self.particles {
            particle.draw_rgba(
                context,
                self.colour,
                (self.lifetime as f64) / (PARTICLE_LIFETIME as f64),
                2.4,
            );
        }
    }

    fn reset_explosion(&mut self) -> () {
        self.exploded = false;
        self.particles.clear();
        self.colour = colour::random_colour();
        self.lifetime = PARTICLE_LIFETIME;
    }
}

impl StandardFirework {
    /* Create new firework at random position on the bottom, with random colour. */
    pub fn new(width: u32, height: u32) -> Self {
        let (vel_min, vel_max) = vel_min_max(height);

        Self {
            rocket: Particle::new(
                TwoVec::new(Math::random() * width as f64, height as f64),
                TwoVec::new(0., vel_min + (vel_max - vel_min) * Math::random()),
            ),
            exploded: false,
            particles: Vec::new(),
            colour: random_colour(),
            lifetime: PARTICLE_LIFETIME,
        }
    }
}
