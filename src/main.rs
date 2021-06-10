fn main() {
	println!("Hello, world!");
}

#[derive(PartialEq, Copy, Clone)]
enum Player { Cross, Circle, None }

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
}