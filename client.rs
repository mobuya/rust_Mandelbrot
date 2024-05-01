use std::{env, fs};
// you may need use std::env for parsing arguments
use std::num::ParseIntError;

use crate::image::Image; // use from another module

// uncomment and implement:
pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> {
    // i need WIDTH HEIGHT MAX_ITERATIONS
    let input_arguments: Vec<String> = env::args().collect();

    if input_arguments.len() < 3 || input_arguments.len() > 4 {
        println!(
            "Usage: {} <width> <height> <max_iterations>",
            input_arguments[0]
        );
        std::process::exit(1);
    }

    let width = input_arguments[1].parse()?;
    let height = input_arguments[2].parse()?;
    let mut max_iterations = 1024;
    if input_arguments.len() == 4 {
        max_iterations = input_arguments[3].parse()?;
    }
    Ok((width, height, max_iterations))
}

pub fn save_to_file(image: &Image, filename: &str) {
    let mut s = String::new();
    s.push_str(&format!("P3\n{} {}\n255\n", image.width, image.height));

    for y in 0..image.height {
        for x in 0..image.width {
            let pixel = image.get(x, y).unwrap();
            s.push_str(&format!("{}\n", pixel));
        }
    }

    fs::write(filename, s).expect("Error writing to disk!");
}
