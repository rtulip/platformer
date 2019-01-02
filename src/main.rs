extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics,     // OpenGL drawing backend.
    player: Player,      // Player struct.
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
        println!("{},{}", self.player.pos.x_vel,self.player.pos.y_vel);
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
            _ if self.player.pos.x_vel < 0.0 => self.deceleration,
            _ if self.player.pos.x_vel > 0.0 => -1.0*self.deceleration,
            _                                => 0.0

        };
        let y_vel = match btn {
            &Button::Keyboard(Key::Down)     => 1.0,
            &Button::Keyboard(Key::Up)       => -1.0,
            _ if self.player.pos.y_vel < 0.0 => self.deceleration,
            _ if self.player.pos.y_vel > 0.0 => -1.0*self.deceleration,
            _                                => 0.0
        };
        self.player.pos.x_vel += x_vel;
        self.player.pos.y_vel += y_vel;
        
    }

}

pub struct Player {

    gl: GlGraphics,
    rotation: f64,
    size: f64,
    pos: Position
}

impl Player {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let size = self.size;
        let square = rectangle::square(0.0, 0.0, size);
        
        let (x, y) = (self.pos.x,
                      self.pos.y);

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });

    }
    
    fn update(&mut self){
        self.pos.x = (self.pos.x + self.pos.x_vel);
        self.pos.y = self.pos.y + self.pos.y_vel;
    }

    pub fn moving(&mut self) -> bool {

        if self.pos.x_vel.abs() > 0.1 || self.pos.y_vel.abs() > 0.1 {
            true
        } else {
            false
        }

    }

    pub fn decelerate(&mut self, deceleration_factor: f64){

        if self.pos.x_vel > 0.1 {
            self.pos.x_vel -= deceleration_factor;
        } else if self.pos.x_vel < 0.1 {
            self.pos.x_vel += deceleration_factor;
        } else if self.pos.x_vel > 0.0 && self.pos.x_vel <= 0.1 {
            self.pos.x_vel = 0.0;
        } else if self.pos.x_vel < 0.0 && self.pos.x_vel >= -0.1 {
            self.pos.x_vel = 0.0;
        }

        if self.pos.y_vel > 0.0 {
            self.pos.y_vel -= deceleration_factor;
        } else if self.pos.y_vel < 0.0 {
            self.pos.y_vel += deceleration_factor;
        } else if self.pos.y_vel > 0.0 && self.pos.y_vel <= 0.1 {
            self.pos.y_vel = 0.0;
        } else if self.pos.y_vel < 0.0 && self.pos.y_vel >= -0.1 {
            self.pos.y_vel = 0.0;
        }

    }
}

pub struct Position{

    x: f64,
    y: f64,
    x_vel: f64,
    y_vel: f64

}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let starting_position = Position {
        x: 0.0,
        y: 0.0,
        x_vel: 0.0,
        y_vel: 0.0
    };

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: Player {
            gl: GlGraphics::new(opengl),
            rotation: 0.0,
            size: 25.0,
            pos: starting_position
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
