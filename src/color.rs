use crate::utils::clamp;
use crate::vec3::Color;
use std::io::Write;
pub fn write_color<T: Write>(out: &mut T, pixel_color: Color, samples_per_pixel: u32) {
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    let mut r = pixel_color.x() * scale;
    let mut g = pixel_color.y() * scale;
    let mut b = pixel_color.z() * scale;

    // Apply gamma = 2 <=> raise the color to the power of 1/gamma
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    let ix = 256. * clamp(r, 0.0, 0.999);
    let iy = 256. * clamp(g, 0.0, 0.999);
    let iz = 256. * clamp(b, 0.0, 0.999);
    let outstream = format!("{} {} {}\n", ix, iy, iz);
    match out.write_all(outstream.as_bytes()) {
        Ok(_) => {}
        Err(_) => println!("Couldn't write"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Color;
    use std::io;
    #[test]
    fn test_write() {
        let p = Color::new(1. / 255., 2. / 255., 0.25);
        write_color(&mut io::stdout(), p, 100);
    }
}
