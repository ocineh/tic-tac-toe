# tic-tac-toe

The tic-tac-toe game playable in the terminal against the computer or another human.

## Usage

You can use the project in three different ways, to play against the computer, to play against
another human, or to make the computer play against itself.

```bash
cargo run -- against computer # To play against the computer (random moves)
cargo run -- against human # To play against another human
cargo run -- random -r 100 -t 10 # The computer will play against itself
```

## Roadmap

- [X] Display the tray
- [X] Check if there is a winner
- [X] Play a move according to a chosen cell
- [X] Playing a random move
- [X] Request a cell phone number from a player
- [X] Playing a game against another person
- [X] Play a game against the computer with random moves
- [X] Start a random game (computer vs computer)
- [X] Run multiple random games (computer vs computer)
- [X] Launching threads that launch multiple random games
- [X] Be able to launch the program with a command line
