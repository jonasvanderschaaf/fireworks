mod fireworks;
mod sim;

use js_sys::Math;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use fireworks::Firework;
use sim::{Particle, TwoVec};

const STAR_RADIUS: f64 = 2.;
const STAR_COUNT: u32 = 20;

const YELLOW: (u8, u8, u8) = (200, 200, 0);

pub struct Graphics {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    stars: Vec<Particle>,
    fireworks: Vec<Firework>,
}

impl Graphics {
    /* Create a new firework simulation on the given canvas. */
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

    /* Create the stars. */
    pub fn init(&mut self) {
        // Generate the stars
        self.create_stars(STAR_COUNT, self.canvas.width(), self.canvas.height());
    }

    pub fn spawn_firework(&mut self) {
        self.fireworks
            .push(Firework::new(self.canvas.width(), self.canvas.height()));
    }

    /* Draw the firework and stars. */
    pub fn draw(&self) {
        /* Clear the canvas. */
        self.context.clear_rect(
            0.,
            0.,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        /* Draw the stars. */
        self.draw_stars();

        /* Draw the fireworks. */
        for firework in &self.fireworks {
            firework.draw(&self.context);
        }
    }

    /* Simulate the fireworks. */
    pub fn step(&mut self) {
        for firework in &mut self.fireworks {
            firework.step(self.canvas.width(), self.canvas.height());
        }
    }

    /* Create stars at random positions on the canvas. */
    fn create_stars(&mut self, count: u32, canvas_width: u32, canvas_height: u32) {
        for _ in 0..count {
            let pos = Particle::new(
                TwoVec::new(
                    Math::random() * canvas_width as f64,
                    Math::random() * canvas_height as f64,
                ),
                TwoVec::zero(),
            );

            self.stars.push(pos);
        }
    }

    /* This function draws the stars on the canvas. */
    fn draw_stars(&self) {
        for star in &self.stars {
            star.draw(&self.context, YELLOW, STAR_RADIUS);
        }
    }
}
