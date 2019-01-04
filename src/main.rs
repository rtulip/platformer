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
        map: map::new(600.0, 425.0, 25.0)
    };

    for i in 0..16 {
        app.map.set_block_type(3, i, map::BlockType::Ground);
    }

    for i in 5..24 {
        app.map.set_block_type(7, i, map::BlockType::Ground);
    
    }

    for i in 0..16 {
        app.map.set_block_type(11, i, map::BlockType::Ground);
    }

    for i in 0..24 {
        app.map.set_block_type(16, i, map::BlockType::Ground);
    }

    app.map.set_block_type(6, 4, map::BlockType::Ground);
    app.map.set_block_type(2, 9, map::BlockType::Ground);
    app.map.set_block_type(10, 16, map::BlockType::Ground);
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
