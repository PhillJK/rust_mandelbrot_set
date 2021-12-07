use image::{ImageBuffer, Rgb};
use num::complex::Complex;

fn calculate_madelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent: f64 = img_x as f64 / width as f64;
            let y_percent: f64 = img_y as f64 / height as f64;
            let cx: f64 = x_min + (x_max - x_min) * x_percent;
            let cy: f64 = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
    let c: Complex<f64> = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c;
    }

    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>, imgx: u32, imgy: u32) {
    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (y, row) in escape_vals.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let r = ((*value as f32) % 255.0) as u8;
            let g = ((*value as f32 * 3.0) % 256.0) as u8;
            let b = ((*value as f32 * 7.0) % 256.0) as u8;
            *pixel = Rgb([r, g, b]);
        }
    }

    imgbuf.save("fractal.png").unwrap();
}

fn main() {
    let width: u32 = 1920 * 25;
    let height: u32 = 1080 * 25;

    let mandelbrot_set =
        calculate_madelbrot(256, -2.0, 1.0, -1.0, 1.0, width as usize, height as usize);

    render_mandelbrot(mandelbrot_set, width, height);
}
