use rand::Rng;

pub struct Board {
  values: std::vec::Vec<std::vec::Vec<u32>>,
  width: usize,
  height: usize,
  num_mines: u32
}

pub fn new(width: usize, height: usize) -> Board {
  let mut board = Board{
    values: vec![vec![0; width]; height],
    width,
    height,
    num_mines: 10,
  };

  board.place_mines();
  board.set_neighbors();
  board
}

impl Board {
  pub fn print_board(&self) {
    for i in self.values.iter() {
      for j in i.iter() {
        print!("{} ", j);
      }
      print!("\n");
    }
  }

  fn place_mines(&mut self) {
    for _i in 0..self.num_mines {
      self.place_one_mine();
    }
  }

  fn place_one_mine(&mut self) {
    let x = rand::thread_rng().gen_range(0, self.width);
    let y = rand::thread_rng().gen_range(0, self.height);

    if self.values[y][x] == 9 {
      self.place_one_mine();
    } else {
      self.values[y][x] = 9;
    }
  }

  fn set_neighbors(&mut self) {
    for i in 0..self.height {
      for j in 0..self.width {
        if self.values[i][j] != 9 {
          self.values[i][j] = self.calculate_mines_around(i, j);
        }
      }
    }
  }

  fn calculate_mines_around(&mut self, x: usize, y: usize) -> u32 {
    let initial_x = if x == 0 {0} else {x-1};
    let final_x = if x+1 > self.height-1 {self.height-1} else {x+1};
    let initial_y = if y == 0 {0} else {y-1};
    let final_y = if y+1 > self.width-1 {self.width-1} else {y+1};

    let mut counter = 0;

    for i in initial_x..final_x+1 {
      for j in initial_y..final_y+1 {
        if self.values[i][j] == 9 {
          counter += 1;
        }
      }
    }

    counter
  }
}

