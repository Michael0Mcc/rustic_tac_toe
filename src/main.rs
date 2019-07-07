mod board;
mod ai;

fn main() {
  let mut board = board::Board::new();
  let board_ref = &mut board;

  board_ref.draw();
  init_turn(board_ref);
}

fn init_turn(board: &mut board::Board) {
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

fn check_game(board: &mut board::Board) {
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