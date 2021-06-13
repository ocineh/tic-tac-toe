mod lib;

use lib::game;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
enum Mode {
	#[structopt(name = "player")]
	Player,
	#[structopt(name = "computer")]
	Computer,
}

#[derive(StructOpt, Debug)]
#[structopt()]
enum Opt {
	#[structopt(name = "against")]
	Against(Mode),
	#[structopt(name = "random")]
	Random {
		#[structopt(short, long = "thread", default_value = "1")]
		thread: u8,
		#[structopt(short, long = "round", default_value = "10")]
		round: u128,
	},
}

fn main() {
	let opt: Opt = Opt::from_args();
	match opt {
		Opt::Against(mode) => {
			match mode {
				Mode::Player => game::against_another_player(),
				Mode::Computer => game::against_computer(),
			}
		}
		Opt::Random { thread, round } => game::random::launch_thread_rounds(thread, round),
	}
}