use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::grid2D_controller::Grid2DController;
use crate::grid2D::Grid2D;

use piston::input::{GenericEvent, Button, MouseButton};

const MANDELBROT_COUNT_COLOR_LOOP: i32 = 20;


/// Stores gameboard view settings.
pub struct Grid2DViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    /// Amount each edge increases/decreases on each scroll event
    pub scroll_zoom: f64,
    /// Amount to shift the point of zoom towards the center on each scroll event as a fractional of the size of the screen
    pub zoom_shift: [f64; 2],
}

impl Grid2DViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> Grid2DViewSettings {
        Grid2DViewSettings {
            position: [0.0; 2],
            size: 1.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 0.0,
            scroll_zoom: 0.9,
            zoom_shift: [0.5, 0.5],
        }
    }
}

pub struct Grid2DView {
    pub grid: Grid2D,
    /// Stores gameboard view settings.
    pub settings: Grid2DViewSettings,
     /// Cursor position
     cursor_pos: [f64; 2],
    colors: Vec<[f32; 4]>,
}

impl Grid2DView {
    /// Creates a new gameboard view.
    pub fn new(grid: Grid2D, settings: Grid2DViewSettings) -> Grid2DView {
        let colordim = 100 * 100 * 100;
        let mut color_grid = vec![[0.0, 0.0, 0.0, 0.0]; 4096];
        let mut counter = 0;
        for r in (100..255).step_by(16) {
            for g in (100..255).step_by(16) {
                for b in (100..255).step_by(16) {
                    color_grid[counter] = [(r as f32)/255.0, (g as f32)/255.0, (b as f32)/255.0, 1.0];
                    counter = counter + 1;
                }
            }
        }
        Grid2DView {
            grid: grid,
            settings: settings,
            cursor_pos: [0.0, 0.0],
            colors: color_grid,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            // println!("{:?}", pos);
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.grid.recenter(self.cursor_pos);
        }
        if let Some(Button::Mouse(MouseButton::Right)) = e.press_args() {
            self.grid.max_count = self.grid.max_count * 10;
            self.grid.update_fractal_set();
        }
        if let Some(scroll_pos) = e.mouse_scroll_args() {
            // println!("scroll info {:?}", scroll_pos);
            self.grid.zoom(scroll_pos, self.settings.scroll_zoom);
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::{Rectangle};

        println!("Center: {:?}", self.grid.center);

        let ref settings = self.settings;

        // let mut rect = [settings.position[0], settings.position[1], settings.size, settings.size];

        // Rectangle::new(settings.background_color).draw(rect, &c.draw_state, c.transform, g);

        for i in 0..self.grid.width {
            for j in 0..self.grid.height {
                let rect = [settings.position[0] + (i as f64), settings.position[1] + (j as f64), settings.size, settings.size];
                let fractal_val = self.grid.cells[i][j];
                let color = self.get_fractal_color(fractal_val);//[fractal_val as f32, fractal_val as f32, fractal_val as f32, 1.0];  // //[0.0, 0.0, fractal_val as f32, 1.0];
                
                Rectangle::new(color).draw(rect, &c.draw_state, c.transform, g);
            }
        }
    }

    fn get_fractal_color(&self, fractal_val: f64) -> Color {
    //     let count_max = self.grid.max_count;

    //     if(fractal_val <= (0.1 * count_max as f64)) {
    //         // println!("Less than 0.1");
    //         return [0.9, 0.9, 0.9, 1.0]
    //     } else if fractal_val >= count_max as f64 {
    //         return [0.0, 0.0, 0.0, 1.0];
    //     }

    //     let rgbIncrements= ((count_max as f64 / 7.0) as f64);
    //     let caseNum = (fractal_val / rgbIncrements).floor() as i32;
    //     let remainNum = fractal_val % rgbIncrements;
        
    //     // if(caseNum != 0) {
    //         // println!("case num {}", caseNum);
    //     // }

    //     match caseNum {
    //         0 => return [0.0, ((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 0.0, 1.0],
    //         1 => return [0.0, 1.0, ((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 1.0],
    //         2 => return [((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 1.0, 1.0, 1.0],
    //         3 => return [((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 0.0, 1.0, 1.0],
    //         4 => return [1.0, ((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 1.0, 1.0],
    //         5 => return [1.0, ((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 0.0, 1.0],
    //         6 => return [1.0, 1.0, ((1.0 / (rgbIncrements as f32)) * (remainNum as f32)), 1.0],
    //         _ => return return [1.0, 0.0, 0.0, 1.0],
    //     }

        if fractal_val >= self.grid.max_count as f64 {
            return [0.0, 0.0, 0.0, 1.0];
        }

        return self.colors[((fractal_val as i32 % 3375)) as usize];
    }

}