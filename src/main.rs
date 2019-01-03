extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod player;
mod map;

pub struct App {
    gl: GlGraphics,     // OpenGL drawing backend.
    player: player::Player,      // Player struct.
    map: map::Map,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        
        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });
        self.map.render(args);
        self.player.render(args);
    }

    fn update(&mut self, args: &UpdateArgs) -> bool {


        self.player.update(&self.map);

        true
    }

}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "game",
            [600, 400]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: player::new(),
        map: map::new(600.0, 400.0, 25.0)
    };

    app.map.set_block_type(3, 0, map::BlockType::Ground);
    app.map.set_block_type(3, 1, map::BlockType::Ground);
    app.map.set_block_type(3, 2, map::BlockType::Ground);
    app.map.set_block_type(3, 3, map::BlockType::Ground);
    app.map.set_block_type(3, 4, map::BlockType::Ground);
    app.map.set_block_type(3, 5, map::BlockType::Ground);
    app.map.set_block_type(3, 6, map::BlockType::Ground);
    app.map.set_block_type(3, 7, map::BlockType::Ground);
    app.map.set_block_type(3, 8, map::BlockType::Ground);
    app.map.set_block_type(3, 9, map::BlockType::Ground);
    app.map.set_block_type(3, 10, map::BlockType::Ground);
    app.map.set_block_type(5, 4, map::BlockType::Ground);
    app.map.set_block_type(5, 5, map::BlockType::Ground);
    app.map.set_block_type(5, 6, map::BlockType::Ground);
    app.map.set_block_type(5, 7, map::BlockType::Ground);
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.player.pressed(&k.button);
            }
        }
        if let Some(u) = e.update_args() {
            if !app.update(&u) {
                break;
            }
        }
        

    }
}
