use crate::board;

const AI_PLAYER: char = board::PLAYER_2;
const HUMAN: char = board::PLAYER_1;
const EMPTY: char = board::EMPTY;

pub fn minimax(mut board: board::Board, player: char) -> i32 {
  // Check for endgame
  if board.check_game().1 {
    if board.check_game().0 == AI_PLAYER {
      return 10;
    } else if board.check_game().0 == HUMAN {
      return -10;
    } else if board.check_game().0 == EMPTY {
      return 0;
    }
  }

  let mut mve: i8 = -1;
  let mut score: i32 = -2;

  for i in 0..board.get_size() {
    if board.get_val(i) == EMPTY {
      board.board[i] = player;
      let score_for_move = if player == AI_PLAYER { minimax(board, HUMAN) } else { minimax(board, AI_PLAYER) };
      if score_for_move > score {
        score = score_for_move;
        mve = i as i8;
      }
    }
  }

  if mve == -1 {
    return 0;
  }

  println!("{}", mve);
  return score;
}
