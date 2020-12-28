use std::f64;
use std::ops::{Add, AddAssign, Sub};

use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::colour::Colour;

#[derive(Clone, Copy)]
pub struct TwoVec {
    x: f64,
    y: f64,
}

impl TwoVec {
    /* Create TwoVec with given x and y coordinates. */
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /* Create a vector with random direction and given magnitude. */
    pub fn random(magnitude: f64) -> Self {
        let phi = Math::random() * f64::consts::TAU;

        Self::new(phi.cos() * magnitude, phi.sin() * magnitude)
    }

    /* Get the x coordinate. */
    pub const fn x(&self) -> f64 {
        self.x
    }

    /* Get the y coordinate. */
    pub const fn y(&self) -> f64 {
        self.y
    }

    /* Get a zero vector. */
    pub const fn zero() -> Self {
        Self { x: 0., y: 0. }
    }
}

impl Add for &TwoVec {
    type Output = TwoVec;

    fn add(self, rhs: Self) -> TwoVec {
        TwoVec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&TwoVec> for TwoVec {
    fn add_assign(&mut self, rhs: &TwoVec) {
        *self = &*self + &rhs;
    }
}

impl Sub for TwoVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Into<(f64, f64)> for &TwoVec {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

/* Convert an rgb triple and alpha value to a CSS colour. */
fn rgba_to_color(rgb: Colour, alpha: f64) -> String {
    format!(
        "rgba({},{},{},{})",
        rgb.0,
        rgb.1,
        rgb.2,
        alpha.max(0.).min(1.)
    )
}

/* This struct represents a particle with position, velocity and acceleration. */
pub struct Particle {
    pos: TwoVec,
    vel: TwoVec,
    acc: TwoVec,
}

impl Particle {
    /* Create a particle with given postion and velocity. */
    pub const fn new(pos: TwoVec, vel: TwoVec) -> Self {
        Self {
            pos,
            vel,
            acc: TwoVec::zero(),
        }
    }

    /* Create a particle at a given point with given speed and random velocity. */
    pub fn random_at(pos: TwoVec, speed: f64) -> Particle {
        Self {
            pos: pos,
            vel: TwoVec::random(speed),
            acc: TwoVec::zero(),
        }
    }

    /* Apply a force to a particle. */
    pub fn apply_force(&mut self, force: TwoVec) {
        self.acc += &force;
    }

    /* Perform one step of a simulation and reset acceleration. */
    pub fn step(&mut self) {
        self.vel += &self.acc;
        self.pos += &self.vel;

        self.acc = TwoVec::zero();
    }

    /* Draw the particle on a canvas with a given color. */
    pub fn draw(&self, context: &CanvasRenderingContext2d, color: Colour, radius: f64) {
        self.draw_rgba(context, color, 1., radius);
    }

    /* Draw the particle on a canvas with a given color and translucency. */
    pub fn draw_rgba(
        &self,
        context: &CanvasRenderingContext2d,
        color: Colour,
        alpha: f64,
        radius: f64,
    ) {
        context.begin_path();

        context.set_fill_style(&wasm_bindgen::JsValue::from_str(&rgba_to_color(
            color, alpha,
        )));

        context
            .arc(
                self.pos.x(),
                self.pos.y(),
                radius,
                0.,
                std::f64::consts::TAU,
            )
            .unwrap();

        context.fill();
    }

    pub fn pos(&self) -> &TwoVec {
        &self.pos
    }

    pub fn set_pos(&mut self, new_pos: TwoVec) {
        self.pos = new_pos;
    }

    pub fn vel(&self) -> &TwoVec {
        &self.vel
    }

    pub fn set_vel(&mut self, new_vel: TwoVec) {
        self.vel = new_vel;
    }
}
