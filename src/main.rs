use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    // Output PPM image
    let path = Path::new("out/hello_world.ppm");
    let mut file = File::create(path)?;

    let width = 200;
    let height = 100;

    write!(file, "P3\n{} {}\n255\n", width, height)?;

    for x in (0..height).rev() {
        for y in 0..width {
            let r = y as f64 / width as f64;
            let g = x as f64 / height as f64;
            let b = 0.2;

            let ir = (255.0 * r) as u8;
            let ig = (255.0 * g) as u8;
            let ib = (255.0 * b) as u8;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    Ok(())
}
