extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

pub mod map_model{
    use opengl_graphics::{GlGraphics,OpenGL};
    use piston::input::*;

    pub enum BlockType{

        empty,
        ground,

    }

    pub struct Block{
        x: f64,
        y: f64,
        size: f64,
        block_type: BlockType,
        passable: bool
    }

    pub struct Map {
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
                    block_type: BlockType::empty,
                    passable: true
                });
            }
            block_vec.push(inner_block_vec);
        }
        let new_map = Map {
            map_width: block_height,
            map_height: block_width,
            grid_size: block_size,
            blocks: block_vec

        };


        println!("width: {}",new_map.blocks.len());
        println!("height: {}", new_map.blocks[0].len());


        for row in 0..new_map.map_width{
            for col in 0..new_map.map_height {
                println!("block[{}][{}] coordinates: ({},{})",row,col,new_map.blocks[row][col].x,new_map.blocks[row][col].y);
            }
        }

        new_map

    }

}