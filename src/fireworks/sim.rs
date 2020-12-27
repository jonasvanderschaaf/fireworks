use std::f64;
use std::ops::{Add, AddAssign, Mul, Sub};

use js_sys::Math;
use web_sys::CanvasRenderingContext2d;

use super::GRAVITY;

#[derive(Clone, Copy)]
pub struct TwoVec {
    x: f64,
    y: f64,
}

impl TwoVec {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn random(speed: f64) -> Self {
        let phi = Math::random() * f64::consts::TAU;

        Self::new(phi.cos() * speed, phi.sin() * speed)
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

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

impl Mul<f64> for &TwoVec {
    type Output = TwoVec;

    fn mul(self, rhs: f64) -> TwoVec {
        TwoVec {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Into<(f64, f64)> for &TwoVec {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

fn rgba_to_color(rgb: (u8, u8, u8), alpha: f64) -> String {
    format!(
        "rgba({},{},{},{})",
        rgb.0,
        rgb.1,
        rgb.2,
        alpha.max(0.).min(1.)
    )
}

pub struct Particle {
    pos: TwoVec,
    vel: TwoVec,
    acc: TwoVec,
}

impl Particle {
    pub const fn new(x: f64, y: f64, vel_x: f64, vel_y: f64) -> Self {
        Self {
            pos: TwoVec::new(x, y),
            vel: TwoVec::new(vel_x, vel_y),
            acc: TwoVec::zero(),
        }
    }

    pub fn random_at(pos: TwoVec, speed: f64) -> Particle {
        Self {
            pos: pos,
            vel: TwoVec::random(speed),
            acc: TwoVec::zero(),
        }
    }

    pub fn apply_force(&mut self, force: TwoVec) {
        self.acc += &force;
    }

    pub fn step(&mut self) {
        self.apply_force(GRAVITY);

        self.vel += &self.acc;
        self.pos += &self.vel;

        self.acc = TwoVec::zero();
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, color: (u8, u8, u8), radius: f64) {
        self.draw_rgba(context, color, 1., radius);
    }

    pub fn draw_rgba(
        &self,
        context: &CanvasRenderingContext2d,
        color: (u8, u8, u8),
        alpha: f64,
        radius: f64,
    ) {
        context.begin_path();

        context.set_fill_style(&wasm_bindgen::JsValue::from_str(&rgba_to_color(
            color, alpha,
        )));

        context
            .ellipse(
                self.pos.x(),
                self.pos.y(),
                radius,
                radius,
                0.,
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
