extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use opengl_graphics::{GlGraphics,OpenGL};
use piston::input::*;

#[derive(Clone,Copy)]
pub enum BlockType{

    Empty,
    Ground,

}

#[derive(Clone,Copy)]
pub struct Block{
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub block_type: BlockType,
}

impl Block {
    pub fn is_passable(&mut self) -> bool{

        match self.block_type{
            BlockType::Empty  => true,
            BlockType::Ground => false
        }

    }
}
pub struct Map {
    gl: GlGraphics,
    pub map_width: usize,
    pub map_height: usize,
    pub grid_size: f64,
    pub blocks: Vec<Vec<Block>>
}

pub fn new(width: f64, height: f64, block_size: f64) -> Map{

    
    let block_height: usize = (height/block_size) as usize;
    let block_width: usize = (width/block_size) as usize;

    let mut block_vec: Vec<Vec<Block>> = Vec::new();
    for row in 0..block_width {
        let mut inner_block_vec: Vec<Block> = Vec::new();
        for col in 0..block_height {

            inner_block_vec.push( Block{
                x: col as f64 * block_size,
                y: row as f64 * block_size,
                size: block_size,
                block_type: BlockType::Empty,
            });
        }
        block_vec.push(inner_block_vec);
    }

    let opengl = OpenGL::V3_2;

    let new_map = Map {
        gl: GlGraphics::new(opengl),
        map_width: block_width,
        map_height: block_height,
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
