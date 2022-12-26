use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::super::colour::Colour;
use super::super::sim::{Particle, TwoVec};
use super::{Firework, Rocket, PARTICLE_COUNT, PARTICLE_LIFETIME};

use super::vel_min_max;

const SPARKLE_COLOUR: Colour = (255, 238, 150);

/* This struct represents a sparkling firework. */
pub struct SparkleFirework {
    rocket: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    phases: Vec<f64>,
    lifetime: u32,
}

/* Implement the sparkling behaviour for this struct. */
impl Rocket for SparkleFirework {
    fn rocket_mut(&mut self) -> &mut Particle {
        &mut self.rocket
    }

    fn rocket(&self) -> &Particle {
        &self.rocket
    }

    fn exploded(&self) -> bool {
        self.exploded
    }

    /* Explode the firework. */
    fn explode(&mut self) -> () {
        self.exploded = true;

        let radius = 1.5 + Math::random() * 1.5;

        /* Create the explosion. */
        for _ in 0..(PARTICLE_COUNT * 10) {
            let mut particle = Particle::random_at(
                self.rocket.pos() + &TwoVec::random(70. * Math::random().powf(0.7)),
                radius + Math::random() * 0.5,
            );
            particle.set_vel(particle.vel() + self.rocket.vel());
            self.particles.push(particle);
        }

        self.phases = (0..(PARTICLE_COUNT * 10))
            .into_iter()
            .map(|_| Math::random() * 10.)
            .collect();
    }

    /* Simulate one step of the explosion. */
    fn sim_explosion(&mut self, width: u32, height: u32) -> () {
        self.lifetime -= 1;

        if self.lifetime == 0 {
            self.reset(width, height);
        }
    }

    /* Draw the explosion. */
    fn draw_explosion(&self, context: &CanvasRenderingContext2d) -> () {
        for (particle, phase) in self.particles.iter().zip(&self.phases) {
            particle.draw_rgba(
                context,
                SPARKLE_COLOUR,
                (self.lifetime as f64) / (PARTICLE_LIFETIME as f64)
                    * ((self.lifetime as f64 / 5. + phase).cos() + 1.)
                    / 2.,
                1.5,
            );
        }
    }

    /* Reset the explosion. */
    fn reset_explosion(&mut self) -> () {
        self.exploded = false;
        self.particles.clear();
        self.lifetime = PARTICLE_LIFETIME;
    }
}

impl SparkleFirework {
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
            phases: Vec::new(),
            lifetime: PARTICLE_LIFETIME,
        }
    }
}
