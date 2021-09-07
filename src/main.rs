extern crate image;
use std::fs;
use std::path::Path;
use std::cmp;

/*
setup
creates a grid of pre set dimensions as A=1,B=0
set the starting coordinates for A=0,B=1

main loop
either save off the grid raw for processing later or convert into an image
*/

struct Cell {
    pub a: f64,
    pub b: f64, 
    a_buf: f64, 
    b_buf: f64
}

const DIFF_A: f64 = 1.0; // diffusion of A 
const DIFF_B: f64 = 0.5;
const FEED: f64 = 0.055;
const KILL: f64 = 0.062;
const DELTA_T: f64 = 1.0;

const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;

impl Cell {
    fn new() -> Cell {
        Cell { a: 0.0, b: 0.0, a_buf: 0.0, b_buf: 0.0 }
    }

    fn update(&mut self, lap_a: f64, lap_b: f64) {
        // updates the buffers, takes in A and B laplacian functions
        self.a_buf = self.a + DELTA_T * ( DIFF_A * lap_a - self.a * self.b * self.b + FEED * ( 1.0 - self.a ) );
        self.b_buf = self.b + DELTA_T * ( DIFF_B * lap_b + self.a * self.b * self.b - self.b * ( KILL + FEED ) );

    }

    fn set(&mut self, a: f64, b: f64) {
      self.a_buf = a;
      self.b_buf = b;
    }

    fn buf_swap(&mut self) {
      self.a = self.a_buf;
      self.b = self.b_buf;
    }

    fn colour(&mut self) -> [u8; 3] {
      // returns the colour of the cell

      // for testing purposes
      [ 0 , 0, cmp::min((self.b * 255.0).trunc() as u8, 255) ]
    }
}

struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

impl Grid {
  fn new() -> Grid {
    Grid { 
      grid: {
        let mut t = Vec::new();
        for _x in 0..WIDTH {
          let mut t2 = Vec::new();
          for _y in 0..HEIGHT {
            t2.push(Cell::new());
          }
          t.push(t2);
        }
        t
      }
    }
  }

  fn update(&mut self,) {
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        // extra code needed to wrap around
        /*
        let lap_a = 0.05 * 
            (self.grid[((x+1) as i64).rem_euclid(WIDTH as i64) as usize][((y-1) as i64).rem_euclid(WIDTH as i64) as usize].a 
            + self.grid[((x+1) as i64).rem_euclid(WIDTH as i64) as usize][((y+1) as i64).rem_euclid(WIDTH as i64) as usize].a 
            + self.grid[((x-1) as i64).rem_euclid(WIDTH as i64) as usize][((y-1) as i64).rem_euclid(WIDTH as i64) as usize].a 
            + self.grid[((x-1) as i64).rem_euclid(WIDTH as i64) as usize][((y+1) as i64).rem_euclid(WIDTH as i64) as usize].a
          ) 
          + 0.2 * 
            (self.grid[((x+1) as i64).rem_euclid(WIDTH as i64) as usize][((y) as i64).rem_euclid(WIDTH as i64) as usize].a 
            + self.grid[((x-1) as i64).rem_euclid(WIDTH as i64) as usize][((y) as i64).rem_euclid(WIDTH as i64) as usize].a 
            + self.grid[((x) as i64).rem_euclid(WIDTH as i64) as usize][((y-1) as i64).rem_euclid(WIDTH as i64) as usize].a 
            + self.grid[((x) as i64).rem_euclid(WIDTH as i64) as usize][((y+1) as i64).rem_euclid(WIDTH as i64) as usize].a
          )
          - self.grid[x][y].a;

        let lap_b = 0.05 * 
            (self.grid[((x+1) as i64).rem_euclid(WIDTH as i64) as usize][((y-1) as i64).rem_euclid(WIDTH as i64) as usize].b 
            + self.grid[((x+1) as i64).rem_euclid(WIDTH as i64) as usize][((y+1) as i64).rem_euclid(WIDTH as i64) as usize].b
            + self.grid[((x-1) as i64).rem_euclid(WIDTH as i64) as usize][((y-1) as i64).rem_euclid(WIDTH as i64) as usize].b
            + self.grid[((x-1) as i64).rem_euclid(WIDTH as i64) as usize][((y+1) as i64).rem_euclid(WIDTH as i64) as usize].b
          ) 
          + 0.2 * 
            (self.grid[((x+1) as i64).rem_euclid(WIDTH as i64) as usize][((y) as i64).rem_euclid(WIDTH as i64) as usize].b 
            + self.grid[((x-1) as i64).rem_euclid(WIDTH as i64) as usize][((y) as i64).rem_euclid(WIDTH as i64) as usize].b
            + self.grid[((x) as i64).rem_euclid(WIDTH as i64) as usize][((y-1) as i64).rem_euclid(WIDTH as i64) as usize].b 
            + self.grid[((x) as i64).rem_euclid(WIDTH as i64) as usize][((y+1) as i64).rem_euclid(WIDTH as i64) as usize].b
          )
          - self.grid[x][y].b;
        */

        let lap_a = 0.05 * 
            (self.grid[((x + WIDTH) + 1) % WIDTH][((y + WIDTH) - 1) % WIDTH].a 
            + self.grid[((x + WIDTH) + 1) % WIDTH][((y + WIDTH) + 1) % WIDTH].a 
            + self.grid[((x + WIDTH) - 1) % WIDTH][((y + WIDTH) - 1) % WIDTH].a 
            + self.grid[((x + WIDTH) - 1) % WIDTH][((y + WIDTH) + 1) % WIDTH].a
          ) 
          + 0.2 * 
            (self.grid[((x + WIDTH) + 1) % WIDTH][y].a 
            + self.grid[((x + WIDTH) - 1) % WIDTH][y].a 
            + self.grid[x][((y + WIDTH) - 1) % WIDTH].a 
            + self.grid[x][((y + WIDTH) + 1) % WIDTH].a
          )
          - self.grid[x][y].a;

        let lap_b = 0.05 * 
            (self.grid[((x + WIDTH) + 1) % WIDTH][((y + WIDTH) - 1) % WIDTH].b 
            + self.grid[((x + WIDTH) + 1) % WIDTH][((y + WIDTH) + 1) % WIDTH].b 
            + self.grid[((x + WIDTH) - 1) % WIDTH][((y + WIDTH) - 1) % WIDTH].b 
            + self.grid[((x + WIDTH) - 1) % WIDTH][((y + WIDTH) + 1) % WIDTH].b
          ) 
          + 0.2 * 
            (self.grid[((x + WIDTH) + 1) % WIDTH][y].b 
            + self.grid[((x + WIDTH) - 1) % WIDTH][y].b 
            + self.grid[x][((y + WIDTH) - 1) % WIDTH].b 
            + self.grid[x][((y + WIDTH) + 1) % WIDTH].b
          )
          - self.grid[x][y].b;

        self.grid[x][y].update(lap_a, lap_b);
      }
    }

    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        self.grid[x][y].buf_swap();
      }
    }
  }

  fn render(&mut self, frame: u64) {
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
      *pixel = image::Rgb(self.grid[x as usize][y as usize].colour());
    }

    // saves the frame 
    let path_str = format!("./output/{}.png", frame);
    let path = Path::new(&path_str);
    imgbuf.save(path).unwrap();
  }

  fn starting_configure(&mut self) {
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        if x % 100 >= 80 && y % 100 >= 80 {
          self.grid[x][y].set( 0.0, 1.0 );
          self.grid[x][y].buf_swap();
        }
      }
    }
  }
}

const MAX_FRAMES: usize = 10;

fn main() {
  fs::create_dir_all("./output").unwrap();

  println!("creating");
  let mut array = Grid::new();
  array.starting_configure();

  for i in 0..MAX_FRAMES {
    array.update();
    println!("saving frame {}", i);
    // array.render(i as u64);
    println!("done with frame {}", i);
  }
  array.render(666);
  println!("done");
}

#[cfg(test)]
mod checks_cell_update {
    use super::*;
    #[test]
    fn a_surround() {
        /*
        0.05  0.2  0.05 
        0.2   -1   0.2 
        0.05  0.2  0.05 
        */ // goes to 1 if centre is 0

        let mut cell = Cell::new();
        cell.update(1.0, 0.0);
        cell.buf_swap();

        assert_eq!(cell.a, 1.055);
        assert_eq!(cell.b, 0.0);
    }

    #[test]
    fn b_surround() {
      /*
      0.05  0.2  0.05 
      0.2   -1   0.2 
      0.05  0.2  0.05 
      */ // goes to 1 if centre is 0

      let mut cell = Cell::new();
      cell.update(0.0, 1.0);
      cell.buf_swap();

      assert_eq!(cell.a, 0.055);
      assert_eq!(cell.b, 0.5);
    }
    #[test]
    fn a_in() {
      /*
      0.05  0.2  0.05 
      0.2   -1   0.2 
      0.05  0.2  0.05 
      */ // goes to 1 if centre is 0

      let mut cell = Cell::new();
      cell.set(1.0, 0.0);
      cell.buf_swap();
      cell.update(0.0, 1.0);
      cell.buf_swap();

      assert_eq!(cell.a, 1.0);
      assert_eq!(cell.b, 0.5);
    }

    #[test]
    fn b_in() {
      /*
      0.05  0.2  0.05 
      0.2   -1   0.2 
      0.05  0.2  0.05 
      */ // goes to 1 if centre is 0

      let mut cell = Cell::new();
      cell.set(0.0, 1.0);
      cell.buf_swap();
      cell.update(1.0, 0.0);
      cell.buf_swap();

      assert_eq!(cell.a, 1.055 );
      assert_eq!(cell.b, 0.883 );
  }

  #[test]
  fn both_in() {
    /*
    0.05  0.2  0.05 
    0.2   -1   0.2 
    0.05  0.2  0.05 
    */ // goes to 1 if centre is 0

    let mut cell = Cell::new();
    cell.set(1.0, 1.0);
    cell.buf_swap();
    cell.update(1.0, 1.0);
    cell.buf_swap();

    assert_eq!(cell.a, 1.0);
    assert_eq!(cell.b, 2.383);
  }
}
