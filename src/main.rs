use std::fmt;

fn main() {
	let board = Board::new();
	board.display();
}

#[derive(PartialEq, Copy, Clone)]
enum Player { Cross, Circle, None }

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Player::Cross => write!(f, "X"),
			Player::Circle => write!(f, "O"),
			Player::None => write!(f, " "),
		}
	}
}

struct Board([[Player; 3]; 3]);

impl Board {
	fn new() -> Board {
		Board([[Player::None; 3]; 3])
	}
	fn is_full(&self) -> bool {
		for row in 0..3 {
			for col in 0..3 {
				if self.0[row][col] == Player::None { return false; }
			}
		}
		true
	}
	fn display(&self) {
		println!("+---+---+---+");
		for row in self.0.iter() {
			println!("| {} | {} | {} |", row[0], row[1], row[2]);
			println!("+---+---+---+");
		}
	}
}