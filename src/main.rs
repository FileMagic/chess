use bitvec::prelude::*;

#[derive(Default)]
// Other docs: https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/rep.html
struct Board {
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

#[derive(PartialEq, Debug)]
enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

#[derive(Debug)]
struct Move {
  from: Square,
  to: Square,
  piece_type: PieceType,
  capture: bool,
  promotion: Option<PieceType>
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/enums.html
#[derive(Clone, Copy, Debug)]
enum Square {
  A1=0,  B1=1,  C1=2,  D1=3,  E1=4,  F1=5,  G1=6,  H1=7,
  A2=8,  B2=9,  C2=10, D2=11, E2=12, F2=13, G2=14, H2=15,
  A3=16, B3=17, C3=18, D3=19, E3=20, F3=21, G3=22, H3=23,
  A4=24, B4=25, C4=26, D4=27, E4=28, F4=29, G4=30, H4=31,
  A5=32, B5=33, C5=34, D5=35, E5=36, F5=37, G5=38, H5=39,
  A6=40, B6=41, C6=42, D6=43, E6=44, F6=45, G6=46, H6=47,
  A7=48, B7=49, C7=50, D7=51, E7=52, F7=53, G7=54, H7=55,
  A8=56, B8=57, C8=58, D8=59, E8=60, F8=61, G8=62, H8=63
}

// Square mapping: https://www.chessprogramming.org/Square_Mapping_Considerations
// think about writing this: https://www.chessprogramming.org/Bitboard_Board-Definition#Denser_Board
// Then write this: https://www.chessprogramming.org/Square_Attacked_By#LegalityTest
impl Board {
  fn default()-> Board {
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
  
  fn create_move(&self, from: Square, to: Square) -> Option<Move> {
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

    // Determine if the move is a capture
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
      promotion: promotion
    }) 
  }
  
}
impl std::fmt::Display for Move{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "from: {:#?}, to: {:#?}, piece_type: {:#?}, capture: {:#?}, promotion: {:#?},", self.from, self.to, self.piece_type, self.capture, self.promotion)
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
                      println!("light: {:#} {:#}, str: {}",file, rank, piece_str );
                  }
                  println!("{:#}",piece_str);
                  s.push_str(format!("{}+", piece_str).as_str());
              } else if self.dark_pieces & (1 << square) != 0 {
                  if self.pawns & (1 << square) != 0 {
                      piece_str = "♟︎".to_owned();
                      println!("dark: {:#} {:#}",file, rank);
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
                  println!("{:#}",piece_str);
                  s.push_str(format!("{}-", piece_str).as_str());
              }
          }
          s.push_str("\n");
      }
      s.push_str("  a b c d e f g h\n");
      write!(f, "{}", s)
  }
}


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // fucking make the board
    let board = Board::default();
    
    // Print the board state.
    println!("{}", board);
    
    // Move b2 to b4.
    let created_move = board.create_move(Square::B2, Square::B5);

    println!("{:?}", created_move);
    Ok(())
}