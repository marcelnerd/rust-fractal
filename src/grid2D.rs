use crate::fractal::mandelbrot;

const ORIGIN: [f64; 2] = [0.0, 0.0];
const MANDELBROT_ORIGIN_X: f64 = 2.5;
// const MANDELBROT_ORIGIN_Y: f64 = 1.125;
const MANDELBROT_COUNT_MAX: i32 = 20;

pub struct Grid2D {
    pub cells: Vec<Vec<f64>>,
    pub width: usize,
    pub height: usize,
    pub calc_width: f64,
    pub calc_height: f64,
    pub center: [f64; 2],
    pub x: f64,
    pub y: f64,
    pub max_count: i32,
}

impl Grid2D {
    pub fn new(resolution: [usize; 2])-> Grid2D {
        let y_origin = MANDELBROT_ORIGIN_X / ((resolution[0] as f64) / (resolution[1] as f64));

        let mandelbrot_set = mandelbrot(ORIGIN, [MANDELBROT_ORIGIN_X, y_origin], resolution, MANDELBROT_COUNT_MAX);

        Grid2D {
            cells: mandelbrot_set,
            width: resolution[0],
            height: resolution[1],
            calc_width: MANDELBROT_ORIGIN_X * 2.0,
            calc_height: y_origin * 2.0,
            center: [0.0, 0.0],
            x: -MANDELBROT_ORIGIN_X,
            y: -y_origin,
            max_count: MANDELBROT_COUNT_MAX,
        }
    }

    pub fn recenter(&mut self, mouse_pos: [f64; 2]) {
        self.center[0] = self.x + self.calc_width * (mouse_pos[0] / (self.width as f64));
        self.center[1] = self.y + self.calc_height * (mouse_pos[1] / (self.height as f64));

        self.x = self.center[0] - (self.calc_width / 2.0);
        self.y = self.center[1] - (self.calc_height / 2.0);

        self.update_fractal_set();
    }

    pub fn zoom(&mut self, scroll_info: [f64; 2], zoom_factor:f64) {
        self.calc_width = self.calc_width - (self.calc_width * zoom_factor * scroll_info[1]);
        self.calc_height = self.calc_height - (self.calc_height * zoom_factor * scroll_info[1]);

        // println!("calc width {:?}", self.calc_width);

        self.x = self.center[0] - (self.calc_width / 2.0);
        self.y = self.center[1] - (self.calc_height / 2.0);

        self.update_fractal_set();
    }

    pub fn update_fractal_set(&mut self) {
        self.cells = mandelbrot(self.center, [self.calc_width, self.calc_height], [self.width, self.height], self.max_count);
    }
}