extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

pub mod player_model {
    
    use opengl_graphics::GlGraphics;
    use piston::input::RenderArgs;

    pub struct Player {

        pub gl: GlGraphics,
        pub rotation: f64,
        pub size: f64,
        pub pos: Position,
        pub vel: Velocity

    }

    pub struct Position {

        pub x: f64,
        pub y: f64

    }
    
    pub struct Velocity {

        pub x: f64,
        pub y: f64

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
        }

        pub fn moving(&mut self) -> bool {

            if self.vel.x.abs() != 0.0 {
                true
            } else {
                false
            }

        }

        pub fn decelerate(&mut self, deceleration_factor: f64){

            let mut scale = self.vel.x.abs().round();
            if scale == 0.0{
                scale = 1.0;
            }
            if self.vel.x.abs() <= deceleration_factor {
                self.vel.x = 0.0;
                println!("Reset x_vel: {}",self.vel.x);
            } else if self.vel.x > deceleration_factor {
                self.vel.x -= deceleration_factor*scale;
            } else if self.vel.x < deceleration_factor {
                self.vel.x += deceleration_factor*scale;
            }
        }
    }
}