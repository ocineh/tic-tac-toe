pub mod board {
	use std::fmt;
	use rand::prelude::SliceRandom;

	#[derive(PartialEq, Copy, Clone)]
	pub enum Player { Cross, Circle, None }

	impl fmt::Display for Player {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match self {
				Player::Cross => write!(f, "X"),
				Player::Circle => write!(f, "O"),
				Player::None => write!(f, " "),
			}
		}
	}

	pub struct Board([[Player; 3]; 3], Vec<i8>);

	impl Board {
		pub fn new() -> Board {
			Board([[Player::None; 3]; 3], vec![1,2,3,4,5,6,7,8,9])
		}
		pub fn is_full(&self) -> bool {
			for row in 0..3 {
				for col in 0..3 {
					if self.0[row][col] == Player::None { return false; }
				}
			}
			true
		}
		pub fn display(&self) {
			println!("+---+---+---+");
			for row in self.0.iter() {
				println!("| {} | {} | {} |", row[0], row[1], row[2]);
				println!("+---+---+---+");
			}
		}
		pub fn check_winner(&self) -> Player {
			if self.0[0][0] == Player::Cross && self.0[0][1] == Player::Cross && self.0[0][2] == Player::Cross { Player::Cross }
			else if self.0[0][0] == Player::Cross && self.0[0][1] == Player::Cross && self.0[0][2] == Player::Cross { Player::Cross }
			else if self.0[2][0] == Player::Cross && self.0[2][1] == Player::Cross && self.0[2][2] == Player::Cross { Player::Cross }
			else if self.0[0][0] == Player::Cross && self.0[1][0] == Player::Cross && self.0[2][0] == Player::Cross { Player::Cross }
			else if self.0[0][1] == Player::Cross && self.0[1][1] == Player::Cross && self.0[2][1] == Player::Cross { Player::Cross }
			else if self.0[0][2] == Player::Cross && self.0[1][2] == Player::Cross && self.0[2][2] == Player::Cross { Player::Cross }
			else if self.0[0][0] == Player::Cross && self.0[1][1] == Player::Cross && self.0[2][2] == Player::Cross { Player::Cross }
			else if self.0[2][0] == Player::Cross && self.0[1][1] == Player::Cross && self.0[0][2] == Player::Cross { Player::Cross }
			else if self.0[0][0] == Player::Circle && self.0[0][1] == Player::Circle && self.0[0][2] == Player::Circle { Player::Circle }
			else if self.0[1][0] == Player::Circle && self.0[1][1] == Player::Circle && self.0[1][2] == Player::Circle { Player::Circle }
			else if self.0[2][0] == Player::Circle && self.0[2][1] == Player::Circle && self.0[2][2] == Player::Circle { Player::Circle }
			else if self.0[0][0] == Player::Circle && self.0[1][0] == Player::Circle && self.0[2][0] == Player::Circle { Player::Circle }
			else if self.0[0][1] == Player::Circle && self.0[1][1] == Player::Circle && self.0[2][1] == Player::Circle { Player::Circle }
			else if self.0[0][2] == Player::Circle && self.0[1][2] == Player::Circle && self.0[2][2] == Player::Circle { Player::Circle }
			else if self.0[0][0] == Player::Circle && self.0[1][1] == Player::Circle && self.0[2][2] == Player::Circle { Player::Circle }
			else if self.0[2][0] == Player::Circle && self.0[1][1] == Player::Circle && self.0[0][2] == Player::Circle { Player::Circle }
			else { Player::None }
		}
		pub fn player_stroke(&mut self, player: Player, pos: i8) -> bool {
			if !self.1.contains(&pos) { return false; }
			let row: usize = ((pos - 1) / 3) as usize;
			let col: usize = ((pos - 1) % 3) as usize;
			if self.0[row][col] == Player::None {
				self.0[row][col] = player;
				let index = self.1.iter().position(|&x| x == pos).unwrap();
				self.1.remove(index);
				true
			}
			else { false }
		}
		pub fn random_stroke(&mut self, player: Player) -> bool {
			match self.1.len() {
				n if n > 1 => self.player_stroke(player, *self.1.choose(&mut rand::thread_rng()).unwrap()),
				n if n == 1 => self.player_stroke(player, self.1[0]),
				_ => false
			}
		}
	}
}

pub mod game {
	use super::board::{Board, Player};
	use std::io::{Write, self};

	fn ask_cell(current_player: Player) -> Result<i8, std::num::ParseIntError> {
		match current_player {
			Player::Cross => print!("The player with the cross must choose an empty cell: "),
			Player::Circle => print!("The player with the circle must choose an empty cell: "),
			Player::None => panic!("The function must always be called with a current player being either the cross or the circle."),
		}
		io::stdout().flush().unwrap();

		let mut cell = String::new();
		io::stdin()
			.read_line(&mut cell)
			.expect("Error reading user input.");

		cell.trim().parse::<i8>()
	}

	pub fn against_another_player() {
		let mut board = Board::new();
		let mut current_player = Player::Cross;

		while !board.is_full() && board.check_winner() == Player::None {
			print!("{esc}[2J{esc}[0;0H", esc = 27 as char);
			println!("Current game:");
			board.display();

			if let Ok(cell) = ask_cell(current_player) {
				if board.player_stroke(current_player, cell) {
					current_player = if current_player == Player::Cross { Player::Circle } else { Player::Cross };
				}
			}
		}

		print!("{esc}[2J{esc}[0;0H", esc = 27 as char);
		println!("Game over");
		board.display();

		match board.check_winner() {
			Player::Cross => println!("Victory of the player with the cross."),
			Player::Circle => println!("Victory of the player with the circle."),
			Player::None => println!("The game ended in a draw."),
		}
	}

	pub fn against_computer() {
		let mut board = Board::new();

		while !board.is_full() && board.check_winner() == Player::None {
			print!("{esc}[2J{esc}[0;0H", esc = 27 as char);
			println!("Current game:");
			board.display();

			match ask_cell(Player::Cross) {
				Ok(cell) => {
					if !board.player_stroke(Player::Cross, cell) { continue }
				}
				Err(_) => continue,
			};

			board.random_stroke(Player::Circle);
		}

		print!("{esc}[2J{esc}[0;0H", esc = 27 as char);
		println!("Game over");
		board.display();

		match board.check_winner() {
			Player::Cross => println!("Victory of the player with the cross."),
			Player::Circle => println!("Victory of the player with the circle."),
			Player::None => println!("The game ended in a draw."),
		}
	}

	pub mod random {
		use super::{Board, Player};

		pub fn round() -> Player {
			let mut board = Board::new();
			let mut current_player = Player::Cross;

			while !board.is_full() && board.check_winner() == Player::None {
				board.random_stroke(current_player);
				current_player = if current_player == Player::Cross { Player::Circle } else { Player::Cross };
			}
			board.check_winner()
		}

		#[derive(Debug)]
		pub struct RoundsResult {
			cross: u128,
			circle: u128,
			none: u128,
		}

		impl RoundsResult {
			fn new() -> RoundsResult {
				RoundsResult { cross: 0, circle: 0, none: 0 }
			}
			fn increment(&mut self, player: Player) {
				match player {
					Player::Cross => self.cross += 1,
					Player::Circle => self.circle += 1,
					Player::None => self.none += 1,
				}
			}
		}

		pub fn rounds(n: u128) -> RoundsResult {
			let mut res = RoundsResult::new();
			for _ in 0..n {
				let player = round();
				res.increment(player);
			}
			res
		}
	}
}