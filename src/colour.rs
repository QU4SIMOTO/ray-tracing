use crate::{interval::Interval, Vec3};
use std::io::Write;

pub type Colour = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_colour(out: &mut impl Write, pixel_color: &Colour) -> Result<(), std::io::Error> {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;

    // Write out the pixel color components.
    write!(out, "{rbyte} {gbyte} {bbyte}\n")
}
