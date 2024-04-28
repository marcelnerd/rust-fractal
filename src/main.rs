mod fractal;
mod grid2D;
mod grid2D_controller;
mod grid2D_view;

use crate::grid2D::Grid2D;
use crate::grid2D_controller::Grid2DController;
use crate::grid2D_view::{Grid2DView, Grid2DViewSettings};


// use image::{ImageBuffer, RgbImage, Rgb, ImageFormat};
// use show_image::{event, create_window};

// extern crate glutin_window;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}
fn main() {
    let width = 900;
    let height = 900;

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("average fractal enjoyer", [width as f64, height as f64])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);
  
    let grid2D = Grid2D::new([width as usize, height as usize]);
    // let mut grid2D_controller = Grid2DController::new(grid2D);
    let grid2D_view_settings = Grid2DViewSettings::new();
    let mut grid2D_view = Grid2DView::new(grid2D, grid2D_view_settings);
  
    while let Some(e) = events.next(&mut window) {
        grid2D_view.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
  
                // clear([1.0; 4], g);
                grid2D_view.draw(&c, g);
            });
        }
    }
}