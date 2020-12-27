mod sim;

use js_sys::Math;
use sim::{Particle, TwoVec};

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const YELLOW: (u8, u8, u8) = (200, 200, 0);
const ORANGE: (u8, u8, u8) = (255, 200, 0);

const PARTICLE_COUNT: u32 = 20;

const PARTICLE_LIFETIME: u32 = 50;

const STAR_RADIUS: f64 = 2.;
const STAR_COUNT: u32 = 20;
const GRAVITY: TwoVec = TwoVec::new(0., 0.1);

struct Firework {
    rocket: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    color: (u8, u8, u8),
    lifetime: u32,
}

impl Firework {
    pub fn new(width: u32, height: u32) -> Self {
        let (vel_min, vel_max) = Self::vel_min_max(height);

        Self {
            rocket: Particle::new(
                Math::random() * width as f64,
                height as f64,
                0.,
                vel_min + (vel_max - vel_min) * Math::random(),
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

    pub fn init(&mut self, width: u32, height: u32) {
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
        if !self.exploded {
            self.rocket.step();

            if self.rocket.vel().y() > 0. {
                self.exploded = true;

                for _ in 0..PARTICLE_COUNT {
                    let mut particle =
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
                self.init(width, height);
            }
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        if !self.exploded {
            self.rocket.draw(context, ORANGE, 3.);
        } else {
            for particle in &self.particles {
                particle.draw_rgba(
                    context,
                    self.color,
                    (self.lifetime as f64) / (PARTICLE_LIFETIME as f64),
                    2.,
                );
            }
        }
    }

    fn vel_min_max(height: u32) -> (f64, f64) {
        (height as f64 / -100., height as f64 / -80.)
    }
}

pub struct Graphics {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    stars: Vec<Particle>,
    fireworks: Vec<Firework>,
}

impl Graphics {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let stars = Vec::new();
        let fireworks = Vec::new();

        Self {
            canvas,
            context,
            stars,
            fireworks,
        }
    }

    pub fn init(&mut self) {
        // Generate the stars
        self.create_stars(STAR_COUNT, self.canvas.width(), self.canvas.height());
    }

    pub fn spawn_firework(&mut self) {
        self.fireworks
            .push(Firework::new(self.canvas.width(), self.canvas.height()));
    }

    pub fn draw(&self) {
        // Clear the canvas.
        self.context.clear_rect(
            0.,
            0.,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.draw_stars();

        for firework in &self.fireworks {
            firework.draw(&self.context);
        }
    }

    pub fn step(&mut self) {
        for firework in &mut self.fireworks {
            firework.step(self.canvas.width(), self.canvas.height());
        }
    }

    // Create stars at random positions on screen.
    fn create_stars(&mut self, count: u32, canvas_width: u32, canvas_height: u32) {
        for _ in 0..count {
            let pos = Particle::new(
                Math::random() * canvas_width as f64,
                Math::random() * canvas_height as f64,
                0.,
                0.,
            );

            self.stars.push(pos);
        }
    }

    fn draw_stars(&self) {
        for star in &self.stars {
            star.draw(&self.context, YELLOW, STAR_RADIUS);
        }
    }
}
