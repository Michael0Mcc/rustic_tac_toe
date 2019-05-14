mod board;
use board::Board as Board;

fn main() {
    let board = Board::new();

    board.draw();
    init_turn(board);
}

fn init_turn(mut board: Board) {
  println!("{}'s turn, where would you like to go? e.g. A1 ", board.player);
  let pos = get_user_input().to_lowercase();
  board.set_val(pos, board.player, -1);
  board.swap_player();

  check_game(board);
}

fn get_user_input() -> String {
    use std::io;

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    return input;
}

fn check_game(board: Board) {
  if board.check_game().1 {
    game_over(board.check_game().0);
  } else {
    init_turn(board);
  }
}

fn game_over(winner: char) {
  println!("Game Over!");
  println!("{} has won!", winner.to_uppercase());
}