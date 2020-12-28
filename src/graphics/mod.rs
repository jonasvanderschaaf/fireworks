mod colour;
mod fireworks;
mod sim;

use core::panic;

use js_sys::Math;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use fireworks::{ColourShiftFirework, Firework, StandardFirework};
use sim::{Particle, TwoVec};

const STAR_RADIUS: f64 = 2.;
const STAR_COUNT: u32 = 20;

pub struct Graphics {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    stars: Vec<Particle>,
    fireworks: Vec<Box<dyn Firework>>,
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

    pub fn resize(&mut self, width: u32, height: u32) {
        let (old_width, old_height) = (self.canvas.width(), self.canvas.height());
        let width_ratio = width as f64 / old_width as f64;
        let height_ratio = height as f64 / old_height as f64;

        for star in &mut self.stars {
            let old_pos = star.pos();

            let (new_x, new_y) = (old_pos.x() * width_ratio, old_pos.y() * height_ratio);

            star.set_pos(TwoVec::new(new_x, new_y));
        }

        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    /* Create the stars. */
    pub fn init(&mut self) {
        // Generate the stars
        self.create_stars(STAR_COUNT, self.canvas.width(), self.canvas.height());
    }

    /* Spawn a firework, depending on what types of firework there are. */
    pub fn spawn_firework(&mut self) {
        match self.fireworks.len() % 2 {
            0 => {
                self.fireworks.push(Box::from(StandardFirework::new(
                    self.canvas.width(),
                    self.canvas.height(),
                )));
            }
            1 => {
                self.fireworks.push(Box::from(ColourShiftFirework::new(
                    self.canvas.width(),
                    self.canvas.height(),
                )));
            }
            _ => {
                panic!("This shouldn't happen ever.");
            }
        }
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

        self.context.set_fill_style(&JsValue::from_str("yellow"));
        self.context.set_text_baseline("middle");
        self.context.set_text_align("center");
        self.context.set_font("70px sans-serif");

        self.context
            .fill_text(
                "Gelukkig Nieuwjaar!",
                self.canvas.width() as f64 / 2.,
                self.canvas.height() as f64 / 2.,
            )
            .unwrap();
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
            star.draw(&self.context, colour::YELLOW, STAR_RADIUS);
        }
    }
}
