use crate::board;

const AI_PLAYER: char = board::PLAYER_2;
const HUMAN: char = board::PLAYER_1;
const EMPTY: char = board::EMPTY;

pub fn minimax(board: &mut board::Board, player: char) -> (i8, i32) {
  // Check for endgame
  if board.check_game().1 {
    if board.check_game().0 == AI_PLAYER {
      return (-1, 10);
    } else if board.check_game().0 == HUMAN {
      return (-1, -10);
    } else if board.check_game().0 == EMPTY {
      return (-1, 0);
    }
  }

  let mut moves: Vec<(i8, i32)> = vec![];

  for i in 0..board.get_size() {
    if board.get_val(i) == EMPTY {
      board.board[i] = player;

      let mut mve: (i8, i32) = (i as i8, 0);
      if player == AI_PLAYER {
        mve.1 = minimax(board, HUMAN).1;
      } else {
        mve.1 = minimax(board, AI_PLAYER).1;
      }
      
      board.board[i] = EMPTY;

      moves.push(mve);
    }
  }

  let mut best_move = 0;
  if player == AI_PLAYER {
    let mut best_score = -1000;
    for i in 0..moves.len() {
      if moves[i].1 > best_score {
        best_score = moves[i].1;
        best_move = i;
      }
    }
  } else {
    if player == AI_PLAYER {
      let mut best_score = 1000;
      for i in 0..moves.len() {
        if moves[i].1 < best_score {
          best_score = moves[i].1;
          best_move = i;
        }
      }
    }
  }

  return moves[best_move];
}
