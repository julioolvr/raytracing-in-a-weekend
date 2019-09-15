use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    // Output PPM image
    let path = Path::new("out/hello_world.ppm");
    let mut file = File::create(path)?;

    file.write(b"P3\n")?;
    file.write(b"3 2\n")?;
    file.write(b"255\n")?;

    file.write(b"255   0   0     0 255   0     0   0 255\n")?;
    file.write(b"255 255   0   255 255 255     0   0   0\n")?;

    Ok(())
}
