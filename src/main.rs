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
            let color = Vector3::new(
                y as f64 / width as f64,
                x as f64 / height as f64,
                0.2
            );

            let ir = (255.0 * color.x) as u8;
            let ig = (255.0 * color.y) as u8;
            let ib = (255.0 * color.z) as u8;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    Ok(())
}

struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
}
