mod lib;
use lib::board::{Board, Player};

fn main() {
	let mut board = Board::new();
	board.player_stroke(Player::Circle, 5);
	board.player_stroke(Player::Cross, 1);
	board.player_stroke(Player::Circle, 3);
	board.player_stroke(Player::Cross, 4);
	board.player_stroke(Player::Circle, 6);
	board.player_stroke(Player::Cross, 7);
	board.display();
	println!("is full: {}", board.is_full());
	println!("check winner: '{}'", board.check_winner());
}