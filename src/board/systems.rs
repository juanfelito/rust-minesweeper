use bevy::prelude::*;
use rand::Rng;

use super::{Board, BoardConfig, ClosedEmpty};

pub fn populate_board(
	mut commands: Commands,
	config: Res<BoardConfig>
) {
	let mut board_creator = BoardCreator {
		values: vec![vec![0; config.width]; config.height],
		width: config.width,
		height: config.height,
		num_mines: config.mines
	};

	board_creator.populate();

	print_board(&board_creator.values); // debug only

	commands.insert_resource(Board{values: board_creator.values});
}

pub fn handle_remaining_tiles(
	remaining: Res<ClosedEmpty>
) {
	if remaining.is_changed() {
		println!("{}", remaining.count);
	}
}

pub fn print_board(board: &Vec<Vec<u8>>) {
    for i in board.iter() {
		for j in i.iter() {
			print!("{} ", j);
		}
		print!("\n");
    }
}

struct BoardCreator {
	values: Vec<Vec<u8>>,
	width: usize,
	height: usize,
	num_mines: u32
}

impl BoardCreator {
	fn populate(&mut self) {
		self.place_mines();
		self.set_neighbors();
	}

	fn place_mines(&mut self) {
		for _i in 0..self.num_mines {
			self.place_one_mine();
		}
	}

	fn place_one_mine(&mut self) {
		let x = rand::thread_rng().gen_range(0..self.width);
		let y = rand::thread_rng().gen_range(0..self.height);
	
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

	fn calculate_mines_around(&mut self, x: usize, y: usize) -> u8 {
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
