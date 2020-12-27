const ORANGE: (u8, u8, u8) = (255, 200, 0);

const PARTICLE_COUNT: u32 = 20;

const PARTICLE_LIFETIME: u32 = 70;

use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::sim::{Particle, TwoVec};

pub struct Firework {
    rocket: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    color: (u8, u8, u8),
    lifetime: u32,
}

impl Firework {
    /* Create new firework at random position on the bottom, with random colour. */
    pub fn new(width: u32, height: u32) -> Self {
        let (vel_min, vel_max) = Self::vel_min_max(height);

        Self {
            rocket: Particle::new(
                TwoVec::new(Math::random() * width as f64, height as f64),
                TwoVec::new(0., vel_min + (vel_max - vel_min) * Math::random()),
            ),
            exploded: false,
            particles: Vec::new(),
            color: (
                (Math::random() * 255.) as u8,
                (Math::random() * 255.) as u8,
                (Math::random() * 255.) as u8,
            ),
            lifetime: PARTICLE_LIFETIME,
        }
    }

    /* Reset the firework. */
    pub fn reset(&mut self, width: u32, height: u32) {
        let (vel_min, vel_max) = Self::vel_min_max(height);

        self.particles = Vec::new();

        self.exploded = false;
        self.rocket
            .set_pos(TwoVec::new(Math::random() * width as f64, height as f64));
        self.rocket.set_vel(TwoVec::new(
            0.,
            vel_min + (vel_max - vel_min) * Math::random(),
        ));
        self.color = (
            (Math::random() * 255.) as u8,
            (Math::random() * 255.) as u8,
            (Math::random() * 255.) as u8,
        );
        self.lifetime = PARTICLE_LIFETIME;
    }

    pub fn step(&mut self, width: u32, height: u32) {
        /* If the rocket has exploded, simulate the rocket, else its explosion. */
        if !self.exploded {
            self.rocket.step();

            /* If the rocket is at the top of its arc, explode it. */
            if self.rocket.vel().y() > 0. {
                self.exploded = true;

                for _ in 0..PARTICLE_COUNT {
                    let particle =
                        Particle::random_at(self.rocket.pos().clone(), 2. + Math::random() * 0.5);
                    self.particles.push(particle);
                }
            }
        } else {
            self.particles
                .iter_mut()
                .for_each(|particle| particle.step());

            self.lifetime -= 1;

            if self.lifetime == 0 {
                self.reset(width, height);
            }
        }
    }

    /* Draw the rocket or its explosion. */
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        if !self.exploded {
            self.rocket.draw(context, ORANGE, 2.3);
        } else {
            for particle in &self.particles {
                particle.draw_rgba(
                    context,
                    self.color,
                    (self.lifetime as f64) / (PARTICLE_LIFETIME as f64),
                    2.4,
                );
            }
        }
    }

    /* Calculate the min and max starting velocity based on screen height. */
    fn vel_min_max(height: u32) -> (f64, f64) {
        (height as f64 / -120., height as f64 / -90.)
    }
}
