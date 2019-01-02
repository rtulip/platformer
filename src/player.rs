extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

pub mod player_model {
    
    use opengl_graphics::{GlGraphics,OpenGL};
    use piston::input::RenderArgs;

    pub struct Player {

        pub gl: GlGraphics,
        pub rotation: f64,
        pub size: f64,
        pub state: PlayerState,
        pub pos: Position,
        pub vel: Velocity,
        pub rules: PlayerMovement,

    }

    pub struct PlayerMovement {

        pub friction: f64,
        pub max_velocity: Velocity,

    }

    pub struct Position {

        pub x: f64,
        pub y: f64

    }
    
    pub struct Velocity {

        pub x: f64,
        pub y: f64

    }

    pub enum PlayerState{

        Stopped,
        Walking,
        Falling

    }

    pub fn new() -> Player{
        let opengl = OpenGL::V3_2;

        let new_player = Player {
            gl: GlGraphics::new(opengl),
            rotation: 0.0,
            size: 25.0,
            state: PlayerState::Stopped,
            pos: Position {
                x: 50.0,
                y: 50.0
            },
            vel: Velocity {
                x: 0.0,
                y: 0.0
            },
            rules: PlayerMovement {
                friction: 0.03,
                max_velocity: Velocity {
                    x: 10.0,
                    y: 10.0
                }
            }
        };

        new_player
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
        
        pub fn update(&mut self){
            self.pos.x = self.pos.x + self.vel.x;
            self.pos.y = self.pos.y + self.vel.y;

            self.decelerate();
        }

        pub fn moving(&mut self) -> bool {

            if self.vel.x.abs() != 0.0 {
                true
            } else {
                false
            }

        }

        pub fn decelerate(&mut self){

            let mut scale = self.vel.x.abs().round();
            if scale == 0.0{
                scale = 1.0;
            }

            if self.vel.x.abs() <= self.rules.friction {
                
                self.vel.x = 0.0;
            
            } else if self.vel.x > self.rules.friction {
                
                self.vel.x -= self.rules.friction*scale;
            
            } else if self.vel.x < self.rules.friction {
            
                self.vel.x += self.rules.friction*scale;
            
            }
        }

    }
}