fn main() {
	println!("Hello, world!");
}

enum Player { Cross, Circle, None }

struct Board([[Player; 3]; 3]);

impl Board {
	fn new() -> Board {
		Board([[Player::None; 3]; 3])
	}
}