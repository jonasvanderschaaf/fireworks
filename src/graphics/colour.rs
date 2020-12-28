use js_sys::Math;

pub type Colour = (u8, u8, u8);

pub const YELLOW: Colour = (200, 200, 0);
pub const ORANGE: Colour = (255, 200, 0);

pub fn random_colour() -> Colour {
    (
        (Math::random() * 255.) as u8,
        (Math::random() * 255.) as u8,
        (Math::random() * 255.) as u8,
    )
}

pub fn colour_add(c1: &Colour, c2: &Colour) -> Colour {
    (
        (c1.0 + c2.0).min(255),
        (c1.1 + c2.1).min(255),
        (c1.2 + c2.2).min(255),
    )
}

pub fn colour_mul(colour: &Colour, num: f64) -> Colour {
    (
        (colour.0 as f64 * num) as u8,
        (colour.1 as f64 * num) as u8,
        (colour.2 as f64 * num) as u8,
    )
}
