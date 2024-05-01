use crate::{
    complex::Complex,
    image::{Image, Pixel},
};

pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
    // default iterations is 1024, this is handeled at input
    // returns number of iterations

    let mut iteration_counter = 0;
    let mut z = Complex { re: 0.0, im: 0.0 };

    while iteration_counter < max_iterations {
        z = z * z + c;

        if z.mag() > 4.0 {
            // number not in the mandelbrot set
            return Some(iteration_counter); // set the pixel to white
        }
        iteration_counter += 1;
    }
    // number is in set; do nothing in the generate_image method
    None
}
// in my image now everything is black, and if its not in the set i make the pixel white

pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
    let mut return_image = Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 / width as f64 - 0.75) * 3.5;
            let cy = (y as f64 / height as f64 - 0.5) * 2.0;

            let c = Complex { re: cx, im: cy };

            let finished_check = check_pixel(c, max_iterations);

            if finished_check.is_some() {
                let invalid_pixel = return_image.get_mut(x, y).unwrap();
                *invalid_pixel = Pixel {
                    r: 255,
                    b: 255,
                    g: 255,
                };
            }
        }
    }

    return_image
}
