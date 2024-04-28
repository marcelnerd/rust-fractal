use crate::grid2D::Grid2D;

use piston::input::GenericEvent;

pub struct Grid2DController {
    pub grid: Grid2D,
}

impl Grid2DController {
    pub fn new(grid: Grid2D) -> Grid2DController {
        Grid2DController {
            grid: grid,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            //self.cursor_pos = pos;
        }
    }
}