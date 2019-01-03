extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

pub mod map_model{
    use opengl_graphics::{GlGraphics,OpenGL};
    use piston::input::*;

    pub enum BlockType{

        Empty,
        Ground,

    }

    pub struct Block{
        x: f64,
        y: f64,
        size: f64,
        block_type: BlockType,
        passable: bool
    }

    pub struct Map {
        gl: GlGraphics,
        map_width: usize,
        map_height: usize,
        grid_size: f64,
        blocks: Vec<Vec<Block>>
    }

    pub fn new(width: f64, height: f64, block_size: f64) -> Map{

        
        let block_height: usize = (height/block_size) as usize;
        let block_width: usize = (width/block_size) as usize;

        let mut block_vec: Vec<Vec<Block>> = Vec::new();
        for row in 0..block_height {
            let mut inner_block_vec: Vec<Block> = Vec::new();
            for col in 0..block_width {

                inner_block_vec.push( Block{
                    x: col as f64 * block_size,
                    y: row as f64 * block_size,
                    size: block_size,
                    block_type: BlockType::Empty,
                    passable: true
                });
            }
            block_vec.push(inner_block_vec);
        }

        let opengl = OpenGL::V3_2;

        let new_map = Map {
            gl: GlGraphics::new(opengl),
            map_width: block_height,
            map_height: block_width,
            grid_size: block_size,
            blocks: block_vec

        };

        new_map

    }

    impl Map {
         
        pub fn render(&mut self, args: &RenderArgs){

            for row in 0..self.map_width {
                for col in 0..self.map_height {
                    
                    match self.blocks[row][col].block_type {
                        BlockType::Empty => {
                            // Draw nothing
                        },
                        BlockType::Ground => {
                            
                            use graphics::*;

                            const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

                            let size = self.grid_size;
                            let square = rectangle::square(0.0, 0.0, size);
                            
                            let (x, y) = (self.blocks[row][col].x,
                                        self.blocks[row][col].y);

                            self.gl.draw(args.viewport(), |c, gl| {
                                let transform = c.transform.trans(x, y);
                                rectangle(BLUE, square, transform, gl);
                            });
                        }
                    }

                }
            }

        }

        pub fn set_block_type(&mut self,row: usize, col: usize, block_type: BlockType){

            self.blocks[row][col].block_type = block_type;

        }

    }

}