use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod fireworks;

static mut GRAPHICS: Option<fireworks::Graphics> = None;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn test() {
    console_log!("Hello World!");
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("fireworks").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    let mut graphics = fireworks::Graphics::new(canvas);

    graphics.init();

    unsafe {
        GRAPHICS = Some(graphics);
    }
}

#[wasm_bindgen]
pub fn draw() {
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        graphics.draw();

        graphics.step();
    }
}

#[wasm_bindgen]
pub fn spawn_firework() {
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        graphics.spawn_firework();
    }
}
