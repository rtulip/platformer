extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use opengl_graphics::{GlGraphics,OpenGL};
use piston::input::*;

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
    pub gravity: f64

}

pub struct Position {

    pub x: f64,
    pub y: f64

}

pub struct Velocity {

    pub x: f64,
    pub y: f64

}
#[derive(Debug)]
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
            friction: 0.5,
            max_velocity: Velocity {
                x: 10.0,
                y: 10.0
            },
            gravity: -0.5
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

    pub fn pressed(&mut self, btn: &Button) {
        match self.state {

            PlayerState::Stopped => {
                
                // Start movement & switch State
                match btn {
                    &Button::Keyboard(Key::Right) => {
                        self.vel.x += 1.5;
                        self.state = PlayerState::Walking;
                    },
                    &Button::Keyboard(Key::Left) => {
                        self.vel.x -= 1.5;
                        self.state = PlayerState::Walking
                    },
                    _ => {}
                }
                
            },
            PlayerState::Falling => {
                // Allow movement
            },
            PlayerState::Walking => {
                
                match btn {
                    &Button::Keyboard(Key::Right) => {
                        self.vel.x += 1.0;
                    },
                    &Button::Keyboard(Key::Left) => {
                        self.vel.x -= 1.0;
                    },
                    _ => {}  
                }

            }

        }        
    }
    
    pub fn update(&mut self, args: &super::map::Map){
        match self.state {
            PlayerState::Walking => {
                self.pos.x = self.pos.x + self.vel.x;
                self.pos.y = self.pos.y + self.vel.y;
                self.decelerate();
            },
            PlayerState::Stopped => {
                self.vel.x = 0.0;
                self.vel.y = 0.0;
            },
            PlayerState::Falling => {
                self.vel.y -= self.rules.gravity;
                self.pos.x = self.pos.x + self.vel.x;
                self.pos.y = self.pos.y + self.vel.y;
                self.decelerate();
            }
        }
        self.check_collision(args);
    }

    pub fn decelerate(&mut self){

        let mut scale = self.vel.x.abs().round();
        if scale == 0.0{
            scale = 1.0;
        }

        if self.vel.x.abs() <= self.rules.friction {
            
            self.vel.x = 0.0;
            self.state = PlayerState::Stopped;
        
        } else if self.vel.x > self.rules.friction {
            
            self.vel.x -= self.rules.friction*scale;
        
        } else if self.vel.x < self.rules.friction {
        
            self.vel.x += self.rules.friction*scale;
        
        }
    }

    pub fn check_collision(&mut self, map: &super::map::Map){

        // Find blocks surrounding player

        let col1 = (self.pos.x / map.grid_size).trunc() as usize;
        let row1 = (self.pos.y / map.grid_size).trunc() as usize;
        let col2 = col1 + 1;
        let row2 = row1 + 1;

        let mut b11 = map.blocks[row1][col1];
        let mut b12 = map.blocks[row1][col2];
        let mut b21 = map.blocks[row2][col1];
        let mut b22 = map.blocks[row2][col2];

        if b21.is_passable() && b22.is_passable(){

            self.state = PlayerState::Falling;

        } else {

            match self.state {

                PlayerState::Falling => {
                    self.state = PlayerState::Walking;
                },
                _ => {
                    
                }
            }

            if self.pos.y + self.size > b21.y{
                self.pos.y = b21.y - self.size;
                println!("Setting Position! {},{}",self.pos.y, b21.y);
            }
            

        }

        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.vel.x = -1.0 * self.vel.x / 4.0;
        } else if self.pos.x + self.size > map.map_width as f64 * map.grid_size {
            self.pos.x = map.map_width as f64 * map.grid_size - self.size;
            self.vel.x = -1.0 * self.vel.x / 4.0; 
        } else if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.vel.y = -1.0 * self.vel.y / 4.0;
        } else if self.pos.y + self.size > map.map_height as f64 * map.grid_size {
            self.pos.y = map.map_height as f64 * map.grid_size - self.size;
            self.vel.y = -1.0 * self.vel.y / 4.0
        }

    }

    
}
