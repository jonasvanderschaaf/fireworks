mod colour_shift;
mod standard;

use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::colour;
use super::sim::{Particle, TwoVec};

const PARTICLE_COUNT: u32 = 20;

const PARTICLE_LIFETIME: u32 = 70;

const ROCKET_EXPLODE_SPEED: f64 = -0.5;

pub const GRAVITY: TwoVec = TwoVec::new(0., 0.07);

pub trait Firework {
    /* Reset the firework. */
    fn reset(&mut self, width: u32, height: u32);

    /* Simulate one step of the firework. */
    fn step(&mut self, width: u32, height: u32);

    /* Draw the firework. */
    fn draw(&self, context: &CanvasRenderingContext2d);
}

/* A rocket is a simple firework which will shoot straight up and explode in
 * way left up to the implementation. When it has exploded, it will respawn
 * on the bottom of the screen and shoot up again. */
pub trait Rocket {
    fn rocket_mut(&mut self) -> &mut Particle;
    fn rocket(&self) -> &Particle;

    fn exploded(&self) -> bool;
    fn explode(&mut self) -> ();

    fn sim_explosion(&mut self, width: u32, height: u32) -> ();

    fn draw_explosion(&self, context: &CanvasRenderingContext2d) -> ();

    fn reset_explosion(&mut self) -> ();
}

/* Implement the firework trait for a struct with the Rocket trait.. */
impl<T> Firework for T
where
    T: Rocket,
{
    /* Reset the rocket in its entirety. */
    fn reset(&mut self, width: u32, height: u32) {
        let (vel_min, vel_max) = vel_min_max(height);

        self.rocket_mut()
            .set_pos(TwoVec::new(Math::random() * width as f64, height as f64));
        self.rocket_mut().set_vel(TwoVec::new(
            0.,
            vel_min + (vel_max - vel_min) * Math::random(),
        ));

        self.reset_explosion();
    }

    /* Simulate one step of the rocket. */
    fn step(&mut self, width: u32, height: u32) -> () {
        if !self.exploded() {
            self.rocket_mut().apply_force(GRAVITY);
            self.rocket_mut().step();

            /* If the rocket is at the top of its arc, explode it. */
            if self.rocket().vel().y() > ROCKET_EXPLODE_SPEED {
                self.explode();
            }
        } else {
            self.sim_explosion(width, height);
        }
    }

    /* Draw the rocket or its explosion. */
    fn draw(&self, context: &CanvasRenderingContext2d) -> () {
        if !self.exploded() {
            self.rocket().draw(context, colour::ORANGE, 2.3);
        } else {
            self.draw_explosion(context);
        }
    }
}

/* Calculate the min and max starting velocity based on screen height. */
fn vel_min_max(height: u32) -> (f64, f64) {
    let height_root = (height as f64).sqrt();
    (height_root / -4., height_root / -3.)
}

pub use colour_shift::ColourShiftFirework;
pub use standard::StandardFirework;
