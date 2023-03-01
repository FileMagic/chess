use super::piece::{PieceType, Move};

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/enums.html
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Square {
  A1=0,  B1=1,  C1=2,  D1=3,  E1=4,  F1=5,  G1=6,  H1=7,
  A2=8,  B2=9,  C2=10, D2=11, E2=12, F2=13, G2=14, H2=15,
  A3=16, B3=17, C3=18, D3=19, E3=20, F3=21, G3=22, H3=23,
  A4=24, B4=25, C4=26, D4=27, E4=28, F4=29, G4=30, H4=31,
  A5=32, B5=33, C5=34, D5=35, E5=36, F5=37, G5=38, H5=39,
  A6=40, B6=41, C6=42, D6=43, E6=44, F6=45, G6=46, H6=47,
  A7=48, B7=49, C7=50, D7=51, E7=52, F7=53, G7=54, H7=55,
  A8=56, B8=57, C8=58, D8=59, E8=60, F8=61, G8=62, H8=63
}

#[derive(Default)]
// Other docs: https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/rep.html
pub struct Board {
  light_pieces:         u64,
  dark_pieces:          u64,
  pawns:                u64,
  bishops:              u64,
  knights:              u64,
  rooks:                u64,
  queens:               u64,
  kings:                u64,
  a_file:               u64,
  h_file:               u64,
  first_rank:           u64,
  eighth_rank:          u64,
  a1_h8_diagonal:       u64,
  h1_a8_anti_diagonal:  u64,
}

// Square mapping: https://www.chessprogramming.org/Square_Mapping_Considerations
// think about writing this: https://www.chessprogramming.org/Bitboard_Board-Definition#Denser_Board
// Then write this: https://www.chessprogramming.org/Square_Attacked_By#LegalityTest
impl Board {
    pub fn create_move(&self, from: Square, to: Square) -> Option<Move> {
      let mut piece_type = None;
      let mut capture = false;
      let mut promotion = None;

      // Determine the piece type of the moving piece
      if self.light_pieces & (1 << (from as u64)) != 0 {
        if self.pawns & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Pawn);
        } else if self.knights & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Knight);
        } else if self.bishops & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Bishop);
        } else if self.rooks & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Rook);
        } else if self.queens & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Queen);
        } else if self.kings & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::King);
        }
      } else if self.dark_pieces & (1 << (from as u64)) != 0 {
        if self.pawns & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Pawn);
        } else if self.knights & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Knight);
        } else if self.bishops & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Bishop);
        } else if self.rooks & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Rook);
        } else if self.queens & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::Queen);
        } else if self.kings & (1 << (from as u64)) != 0 {
          piece_type = Some(PieceType::King);
        }
      }

      // Determine if the move is a capture (are we moving to a dark piece?)
      if self.light_pieces & (1 << to as u64) != 0 || self.dark_pieces & (1 << (to as u64)) != 0 {
        capture = true;
      }

      // Determine if the move is a promotion
      if (piece_type == Some(PieceType::Pawn)) && (((to as u8) < 8) || ((to as u8) > 55)) {
        promotion = Some(PieceType::Queen);
      }
      // Create the move
      Some(Move {
        from: from,
        to: to,
        piece_type: piece_type.unwrap(),
        capture: capture,
        promotion: promotion,
        en_passant: false,
      })
    }

    pub fn default()-> Board {
      // This is using the Little-Endian Rank-File Mapping
      // This was useful while writing these: https://tearth.dev/bitboard-viewer/
      Board {
        light_pieces:           0x000000000000FFFF,
        dark_pieces:            0xFFFF000000000000,
        pawns:                  0x00FF00000000FF00,
        bishops:                0x2400000000000024,
        knights:                0x4200000000000042,
        rooks:                  0x8100000000000081,
        queens:                 0x0800000000000008,
        kings:                  0x1000000000000010,
        a_file:                 0x0101010101010101,
        h_file:                 0x8080808080808080,
        first_rank:             0x00000000000000FF,
        eighth_rank:            0xFF00000000000000,
        a1_h8_diagonal:         0x8040201008040201,
        h1_a8_anti_diagonal:    0x0102040810204080,
      }
    }

    fn is_king_attacked(&self) -> bool{
      unimplemented!();
    }

    fn is_move_valid(&self, m: &Move) -> bool {
    let from_bb = 1u64 << (m.from as u64);
    let to_bb = 1u64 << (m.to as u64);

    // 1. The destination square is not occupied by a friendly piece.
    if self.light_pieces & to_bb != 0 && self.light_pieces & from_bb != 0 {
        return false;
    }
    if self.dark_pieces & to_bb != 0 && self.dark_pieces & from_bb != 0 {
        return false;
    }

    // 2. The moving piece can legally move to the destination square.
    let valid_moves = match m.piece_type {
        PieceType::Pawn => PieceType::get_pawn_moves(m.from, m.promotion.is_some(), m.en_passant),
        PieceType::Knight => PieceType::get_knight_moves(m.from),
        PieceType::Bishop => PieceType::get_bishop_moves(m.from),
        PieceType::Rook => PieceType::get_rook_moves(m.from),
        PieceType::Queen => PieceType::get_queen_moves(m.from),
        PieceType::King => PieceType::get_king_moves(m.from),
    };
    if !valid_moves.contains(m) {
        return false;
    }

    // 3. The move does not put or leave the king in check.
    let mut new_board = self.clone();
    new_board.make_move(m);
    if new_board.is_king_attacked() {
        return false;
    }

    // 4. If the move is a pawn promotion, the promoted piece is a legal one.
    if let Some(pt) = &m.promotion {
        if (*pt != PieceType::Queen) && (*pt != PieceType::Rook) && (*pt != PieceType::Bishop) && (*pt != PieceType::Knight) {
            return false;
        }
    }

    true
}

  fn make_move(&self, m: &Move) {
      unimplemented!();
    }

}

impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let mut s = String::new();
      for rank in (0..8).rev() {
          s.push_str(&format!("{} ", rank + 1));
          for file in 0..8 {
              let square = file + rank * 8;
              let mut piece_str = String::from(" ");
              if self.light_pieces & (1 << square) != 0 {
                  if self.pawns & (1 << square) != 0 {
                      piece_str = "♙".to_owned();
                  } else if self.knights & (1 << square) != 0 {
                      piece_str = "♘".to_owned();
                  } else if self.bishops & (1 << square) != 0 {
                      piece_str = "♗".to_owned();
                  } else if self.rooks & (1 << square) != 0 {
                      piece_str = "♖".to_owned();
                  } else if self.queens & (1 << square) != 0 {
                      piece_str = "♕".to_owned();
                  } else if self.kings & (1 << square) != 0 {
                      piece_str = "♔".to_owned();
                  }
                  s.push_str(format!("{}+", piece_str).as_str());
              } else if self.dark_pieces & (1 << square) != 0 {
                  if self.pawns & (1 << square) != 0 {
                      piece_str = "♟︎".to_owned();
                  } else if self.knights & (1 << square) != 0 {
                      piece_str = "♞".to_owned();
                  } else if self.bishops & (1 << square) != 0 {
                      piece_str = "♝".to_owned();
                  } else if self.rooks & (1 << square) != 0 {
                      piece_str = "♜".to_owned();
                  } else if self.queens & (1 << square) != 0 {
                      piece_str = "♛".to_owned();
                  } else if self.kings & (1 << square) != 0 {
                      piece_str = "♚".to_owned();
                  }
                  s.push_str(format!("{}-", piece_str).as_str());
              }
          }
          s.push_str("\n");
      }
      s.push_str("  a b c d e f g h\n");
      write!(f, "{}", s)
  }
}