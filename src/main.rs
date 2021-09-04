fn main() {
    println!("Hello, world!");
}


/*
setup
creates a grid of pre set dimensions as A=1,B=0
set the starting coordinates for A=0,B=1

main loop
update grid into second grid, replace grid 1 with new grid
either save off the grid raw for processing later or convert into an image

what is needed: 
a struct for the grid, which contains: 
  a 2d vector - public? - mainly for graphics
  its height, and width, both public, edge is included in this 
  its constants - private
and has the functions for 
  new - takes in dimensions and constants
  next iteration
    edges skipped and kept at 0, only updates non edge cells
  update constants 
*/

struct Cell {
    a: f64,
    b: f64
}


const DIFF_A: f64 = 1.0; // diffusion of A 
const DIFF_B: f64 = 1.0;
const FEED: f64 = 0.055;
const KILL: f64 = 0.062;

impl Cell {
    fn new(a: f64, b: f64) -> Cell {
        Cell { a, b }
    }

    fn update(&mut self, lap_a: f64, lap_b: f64) {
        // updates the current cell, takes in A and B laplacian functions

    }
}

struct Grid<'a> {
    pub grid: &'a Vec<Vec<Cell>>,
    pub height: i64, // edges included
    pub width: i64,
}
