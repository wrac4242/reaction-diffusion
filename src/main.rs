fn main() {
    println!("Hello, world!");
}

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
const DIFF_B: f64 = 1.0;
const FEED: f64 = 0.055;
const KILL: f64 = 0.062;
const DELTA_T: f64 = 1.0;

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
      self.a = a;
      self.b = b;
    }

    fn buf_swap(&mut self) {
      self.a = self.b_buf;
      self.b = self.b_buf;
    }
}

const HEIGHT: usize = 100;
const WIDTH: usize = 100;

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
    for x in 1..WIDTH-1 {
      for y in 1..HEIGHT-1 {
        /*
        0.05  0.2  0.05 
        0.2   -1   -.2 
        0.05  0.2  0.05 
        */
        let lap_a = 0.05 * (self.grid[x+1][y-1].a + self.grid[x+1][y+1].a + self.grid[x-1][y-1].a + self.grid[x-1][y+1].a) 
          + 0.2 * (self.grid[x+1][y].a + self.grid[x-1][y].a + self.grid[x][y-1].a + self.grid[x][y+1].a)
          - self.grid[x][y].a;
        let lap_b = 0.05 * (self.grid[x+1][y-1].b + self.grid[x+1][y+1].b + self.grid[x-1][y-1].b + self.grid[x-1][y+1].b) 
          + 0.2 * (self.grid[x+1][y].b + self.grid[x-1][y].b + self.grid[x][y-1].b + self.grid[x][y+1].b)
          - self.grid[x][y].b;
        self.grid[x][y].update(lap_a, lap_b);
      }
    }

    for x in 1..WIDTH-1 {
      for y in 1..HEIGHT-1 {
        self.grid[x][y].buf_swap();
      }
    }
  }
}

