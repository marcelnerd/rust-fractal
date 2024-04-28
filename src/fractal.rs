use std::f64::INFINITY;

// const MANDELBROT_ORIGIN_X: f64 = 2.0;
// const MANDELBROT_ORIGIN_Y: f64 = 1.5;

// dim is the actual calculation dimensions, resolution is the size of the data / window
pub fn mandelbrot(center: [f64; 2], dim: [f64; 2], resolution: [usize; 2], count_max: i32) -> Vec<Vec<f64>> {
    let x2 = center[0] + (dim[0] / 2.0);
    let x1 = center[0] - (dim[0] / 2.0);
    let y2 = center[1] + (dim[1] / 2.0);
    let y1 = center[1] - (dim[1] / 2.0);
    
    let x_precision = (x2 - x1) / (resolution[0] as f64);
    let y_precision = (y2 - y1) / (resolution[1] as f64);

    let mut fractal_set = vec![vec![0.0; resolution[1]]; resolution[0]];

    for x in 0..resolution[0] {
        for y in 0..resolution[1] {
            let mut zx = 0.0;
            let mut zy = 0.0;
            let cx = (x as f64) * x_precision + x1;
            let cy = (y as f64) * y_precision + y1;
            let mut count = 0;

            while ((zx * zx) + (zy * zy)) <= 4.0 && count < count_max {
                let x_new = (zx * zx) - (zy * zy) + cx;
                zy = 2.0 * zx * zy + cy;
                zx = x_new;

                count += 1;
            }

            fractal_set[x][y] = count as f64; //(count_max as f64) % (count as f64);
        }
    }

    return fractal_set
}