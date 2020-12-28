use colour::random_colour;
use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::super::colour;
use super::super::colour::Colour;
use super::super::sim::{Particle, TwoVec};

use super::{vel_min_max, Firework, Rocket, GRAVITY, PARTICLE_COUNT, PARTICLE_LIFETIME};

/* This struct represents firework of which the explosion gradually changes
 * colour over its lifetime. */
pub struct ColourShiftFirework {
    rocket: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    first_colour: Colour,
    second_colour: Colour,
    lifetime: u32,
}

/* Implement the standard rocket behaviour for this struct. */
impl Rocket for ColourShiftFirework {
    fn rocket_mut(&mut self) -> &mut Particle {
        &mut self.rocket
    }

    fn rocket(&self) -> &Particle {
        &self.rocket
    }

    fn exploded(&self) -> bool {
        self.exploded
    }

    /* Dictate the explosion behaviour. */
    fn explode(&mut self) -> () {
        self.exploded = true;

        let radius = 1.5 + Math::random() * 1.5;

        /* Create the explosion. */
        for _ in 0..PARTICLE_COUNT {
            let mut particle =
                Particle::random_at(self.rocket.pos().clone(), radius + Math::random() * 0.5);
            particle.set_vel(particle.vel() + self.rocket.vel());
            self.particles.push(particle);
        }
    }

    /* Simulate the explosion for one step. */
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

    /* Draw the explosion on a given canvas. */
    fn draw_explosion(&self, context: &CanvasRenderingContext2d) -> () {
        for particle in &self.particles {
            let lifetime_frac = (self.lifetime as f64) / (PARTICLE_LIFETIME as f64);
            let colour_shift = 1. - lifetime_frac.powi(6);
            let alpha = lifetime_frac.powi(2);

            particle.draw_rgba(
                context,
                /* Mix the two colours together in the right amount. */
                colour::colour_add(
                    &colour::colour_mul(&self.first_colour, colour_shift),
                    &colour::colour_mul(&self.second_colour, 1. - colour_shift),
                ),
                alpha,
                2.4,
            );
        }
    }

    /* Reset the explosion. */
    fn reset_explosion(&mut self) -> () {
        self.exploded = false;
        self.particles.clear();
        self.first_colour = random_colour();
        self.second_colour = random_colour();
        self.lifetime = PARTICLE_LIFETIME;
    }
}

impl ColourShiftFirework {
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
            first_colour: random_colour(),
            second_colour: random_colour(),
            lifetime: PARTICLE_LIFETIME,
        }
    }
}
