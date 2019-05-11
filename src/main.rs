fn main() {
    // println!("You typed: {}", get_user_input());
    let manager = Manager::new('x', 'o', '▢');
    let board = Board::new(manager);

    board.draw();
    init_turn(board);
}

fn init_turn(mut board: Board) {
  println!("{}'s turn, where would you like to go? e.g. A1 ", board.manager.player);
  let pos = get_user_input().to_lowercase();
  board.set_val(pos, board.manager.player, -1);
  board.manager.swap_player();

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

/******************
*** Board Class ***
******************/

struct Board {
  board: [char; 9],
  manager: Manager,
}

impl Board {
  fn new(manager: Manager) -> Board {
    Board {
      board: [manager.empty; 9],
      manager: manager,
    }
  }

  fn get_size(&self) -> usize {
      self.board.len()
  }

  fn get_val(&self, pos: usize) -> char {
    self.board[pos]
  }

  fn set_val(&mut self, pos: String, player: char, mut i: i8) {
    if i == -1 {
      if pos.contains("a1") { i = 0; }
      else if pos.contains("a2") { i = 1; }
      else if pos.contains("a3") { i = 2; }
      else if pos.contains("b1") { i = 3; }
      else if pos.contains("b2") { i = 4; }
      else if pos.contains("b3") { i = 5; }
      else if pos.contains("c1") { i = 6; }
      else if pos.contains("c2") { i = 7; }
      else if pos.contains("c3") { i = 8; }
      else { i = -2; }
    }

    if i != -2 && self.board[i as usize] == self.manager.empty {
      self.board[i as usize] = player;
      self.manager.swap = true;
      self.draw();
    } else {
      println!("Position doesn't exist or spot is already taken. Please chose another position.");
      self.draw();
    }
  }

  fn draw(&self) {
    println!("  1 2 3");
    println!("A {0} {1} {2}", self.board[0], self.board[1], self.board[2]);
    println!("B {0} {1} {2}", self.board[3], self.board[4], self.board[5]);
    println!("C {0} {1} {2}", self.board[6], self.board[7], self.board[8]);
  }

  fn check_game(&self) -> (char, bool) {
      let mut w = self.board[4];
      if w == self.manager.empty { w = '-'; }

      if self.board[0] == w && self.board[8] == w { return (w, true); } // LT-RB Diagonal
      else if self.board[2] == w && self.board[6] == w { return (w, true); } // LB-RT Diagonal
      else if self.board[3] == w && self.board[5] == w { return (w, true); } // Center Horizontal
      else if self.board[1] == w && self.board[7] == w { return (w, true); } // Center Vertical

      w = self.board[0];
      if w == self.manager.empty { w = '-'; }

      if self.board[1] == w && self.board[2] == w { return (w, true); } // Top Horizontal
      else if self.board[3] == w && self.board[6] == w { return (w, true ); } // Left Vertical

      w = self.board[8];
      if w == self.manager.empty { w = '-'; }

      if self.board[6] == w && self.board[7] == w { return (w, true); } // Bottom Horizontal
      else if self.board[2] == w && self.board[5] == w { return (w, true ); } // Right Vertical

      // Tie Check
      let mut is_full = true;
      for pos in &self.board {
        if pos == &self.manager.empty { is_full = false; }
      }

      if is_full {
        return (self.manager.empty, true);
      } else {
        return (self.manager.empty, false);
      }

  }
}

/********************
*** Manager Class ***
********************/

struct Manager {
  player: char,
  player_1: char,
  player_2: char,
  empty: char,
  swap: bool,
}

impl Manager {
  fn new(player_1: char, player_2: char, empty: char) -> Manager {
    Manager {
      player: player_1,
      player_1: player_1,
      player_2: player_2,
      empty: empty,
      swap: false,
    }
  }

  fn swap_player(&mut self) {
    if self.swap {
      if self.player == self.player_1 {
        self.player = self.player_2;
      } else {
        self.player = self.player_1;
      }
      self.swap = false;
    }
  }
}

/***************
*** AI Class ***
***************/