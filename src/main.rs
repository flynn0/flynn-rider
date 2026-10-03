fn main() {
    let width = 100;
    let height = 40;

    // ASCII characters from "empty" to "dense"
    let chars = " .:-=+*#%@";

    for y in 0..height {
        let cy = (y as f64 / height as f64) * 2.0 - 1.0;

        for x in 0..width {
            let cx = (x as f64 / width as f64) * 3.5 - 2.5;

            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iterations = 0;

            let max_iterations = 100;

            while zx * zx + zy * zy <= 4.0 && iterations < max_iterations {
                let new_zx = zx * zx - zy * zy + cx;
                let new_zy = 2.0 * zx * zy + cy;

                zx = new_zx;
                zy = new_zy;
                iterations += 1;
            }

            let index = if iterations == max_iterations {
                chars.len() - 1
            } else {
                iterations * (chars.len() - 1) / max_iterations
            };

            print!("{}", chars.chars().nth(index).unwrap());
        }

        println!();
    }
}
