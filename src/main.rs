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

pub struct App {
    gl: GlGraphics,     // OpenGL drawing backend.
    player: player::player_model::Player,      // Player struct.
    deceleration: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        
        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });

        self.player.render(args);
    }

    fn update(&mut self, args: &UpdateArgs) -> bool {

        if self.player.moving() {
            self.player.decelerate(self.deceleration);
        }

        self.player.update();

        true
    }

    fn pressed(&mut self, btn: &Button) {
        let x_vel = match btn {
            &Button::Keyboard(Key::Right)    => 1.0,
            &Button::Keyboard(Key::Left)     => -1.0,
            _                                => 0.0

        };
        let y_vel = match btn {
            &Button::Keyboard(Key::Down)     => 1.0,
            &Button::Keyboard(Key::Up)       => -1.0,
            _                                => 0.0
        };
        self.player.vel.x += x_vel;
        self.player.vel.y += y_vel;
        
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
        player: player::player_model::Player {
            gl: GlGraphics::new(opengl),
            rotation: 0.0,
            size: 25.0,
            pos: player::player_model::Position {
                x: 0.0,
                y: 0.0
            },
            vel: player::player_model::Velocity {
                x: 0.0,
                y: 0.0
            }
        },
        deceleration: 0.1
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            if !app.update(&u) {
                break;
            }
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.pressed(&k.button);
            }
        }

    }
}
