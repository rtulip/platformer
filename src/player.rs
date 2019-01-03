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
            friction: 0.05,
            max_velocity: Velocity {
                x: 10.0,
                y: 10.0
            },
            gravity: 0.5
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
                        self.update_velocity(1.5, 0.0);
                        self.state = PlayerState::Walking;
                    },
                    &Button::Keyboard(Key::Left) => {
                        self.update_velocity(-1.5, 0.0);
                        self.state = PlayerState::Walking
                    },
                    &Button::Keyboard(Key::Up) => {
                        self.jump()
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
                        self.update_velocity(1.0, 0.0);
                    },
                    &Button::Keyboard(Key::Left) => {
                        self.update_velocity(-1.0, 0.0);
                    },
                    _ => {}  
                }
            }
        }        
    }
    
    pub fn update(&mut self, args: &super::map::Map){
        match self.state {
            PlayerState::Walking => {
                self.update_position(self.vel.x, self.vel.y);
                self.decelerate();
            },
            PlayerState::Stopped => {
                self.set_velocity(0.0, 0.0);
            },
            PlayerState::Falling => {
                self.update_velocity(0.0, self.rules.gravity);
                self.update_position(self.vel.x, self.vel.y);
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
            self.set_velocity(0.0, self.vel.y);
            self.state = PlayerState::Stopped;
        
        } else if self.vel.x > self.rules.friction {
            
            self.update_velocity(-1.0*self.rules.friction*scale, 0.0);
        
        } else if self.vel.x < self.rules.friction {
        
            self.update_velocity(self.rules.friction*scale, 0.0);
        
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
                self.set_position(self.pos.x, b21.y - self.size);
                println!("Setting Position! {},{}",self.pos.y, b21.y);
            }
            

        }

        if self.pos.x < 0.0 {
           
            self.set_position(0.0, self.pos.y);
            self.set_velocity(-1.0 * self.vel.x / 4.0, self.vel.y);

        } else if self.pos.x + self.size > map.map_width as f64 * map.grid_size {
            
            self.set_position(map.map_width as f64 * map.grid_size - self.size, self.pos.y);
            self.set_velocity(-1.0 * self.vel.x / 4.0, self.vel.y);
            
        } else if self.pos.y < 0.0 {
            
            self.set_position(self.pos.x, 0.0);
            self.set_velocity(self.vel.x, -1.0 * self.vel.y / 4.0);
            
        } else if self.pos.y + self.size > map.map_height as f64 * map.grid_size {
            
            self.set_position(self.pos.x, map.map_height as f64 * map.grid_size - self.size);
            self.set_velocity(self.vel.x, -1.0 * self.vel.y / 4.0);
            
        }

    }

    fn jump(&mut self){

        match self.state {

            PlayerState::Falling => {

            },
            PlayerState::Walking => {
                self.update_velocity(0.0, -7.0);
                self.state = PlayerState::Falling;
            },
            PlayerState::Stopped => {
                self.update_velocity(0.0, -7.0);
                self.state = PlayerState::Falling;
            }

        }

    }

    fn update_velocity(&mut self, delta_x: f64, delta_y: f64){

        self.vel.x += delta_x;
        self.vel.y += delta_y;

    }

    fn set_velocity(&mut self, new_x: f64, new_y: f64){

        self.vel.x = new_x;
        self.vel.y = new_y;

    }
    
    fn update_position(&mut self, delta_x: f64, delta_y: f64){

        self.pos.x += delta_x;
        self.pos.y += delta_y;

    }

    fn set_position(&mut self, new_x: f64, new_y: f64){

        self.pos.x = new_x;
        self.pos.y = new_y;

    }
}
