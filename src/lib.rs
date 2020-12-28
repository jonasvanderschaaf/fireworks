use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod graphics;

static mut GRAPHICS: Option<graphics::Graphics> = None;

/* Initialize the simulation. */
#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("fireworks").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    /* Resize the canvas to take up the entire screen. */
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    /* Create a new simulation and initialize it. */
    let mut graphics = graphics::Graphics::new(canvas);
    graphics.init();

    unsafe {
        GRAPHICS = Some(graphics);
    }
}

/* Draw the current state of the simulation, and simulate one step. */
#[wasm_bindgen]
pub fn draw() {
    /* This is only unsafe when aquiring multiple muteable references,
     * but the reference is immediately dropped, so this is safe. */
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        graphics.step();

        graphics.draw();
    }
}

/* Spawn a new firework. */
#[wasm_bindgen]
pub fn spawn_firework() {
    /* This is only unsafe when aquiring multiple muteable references,
     * but the reference is immediately dropped, so this is safe. */
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        graphics.spawn_firework();
    }
}

#[wasm_bindgen]
pub fn resize_canvas() {
    /* This is only unsafe when aquiring multiple muteable references,
     * but the reference is immediately dropped, so this is safe. */
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        let window = web_sys::window().unwrap();

        graphics.resize(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
    }
}
