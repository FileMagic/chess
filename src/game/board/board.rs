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

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/enums.html
enum Square {
  a1=0,  b1=1,  c1=2,  d1=3,  e1=4,  f1=5,  g1=6,  h1=7,
  a2=8,  b2=9,  c2=10, d2=11, e2=12, f2=13, g2=14, h2=15,
  a3=16, b3=17, c3=18, d3=19, e3=20, f3=21, g3=22, h3=23,
  a4=24, b4=25, c4=26, d4=27, e4=28, f4=29, g4=30, h4=31,
  a5=32, b5=33, c5=34, d5=35, e5=36, f5=37, g5=38, h5=39,
  a6=40, b6=41, c6=42, d6=43, e6=44, f6=45, g6=46, h6=47,
  a7=48, b7=49, c7=50, d7=51, e7=52, f7=53, g7=54, h7=55,
  a8=56, b8=57, c8=58, d8=59, e8=60, f8=61, g8=62, h8=63
}

// Square mapping: https://www.chessprogramming.org/Square_Mapping_Considerations
// think about writing this: https://www.chessprogramming.org/Bitboard_Board-Definition#Denser_Board
// Then write this: https://www.chessprogramming.org/Square_Attacked_By#LegalityTest
impl Board{
  fn default()-> Board {
    // This is using the Little-Endian Rank-File Mapping
    // This was useful while writing these: https://tearth.dev/bitboard-viewer/
    Board {
      light_pieces:           0x55AA55AA55AA55AA,
      dark_pieces:            0xAA55AA55AA55AA55,
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
  
  fn get_attacking_squares() -> u64 {

  }

  fn check_if_move_obstructed(square_one: Square, square_two: Square) -> u64 {
    // Reading this: http://www.talkchess.com/forum3/viewtopic.php?f=7&t=12499&start=14
    // Initialize Constants
    let m1: u64       = -1;
    let a2_to_a7: u64 = 0x0001010101010100;
    let b2_to_g7: u64 = 0x0040201008040200;
    let h1_to_b7: u64 = 0x0002040810204080;

    // Use constants find any obstructing pieces. 
    let btwn: u64  = (m1 << square_one) ^ (m1 << square_two);
    let file: u64  =   (square_two & 7) - (sq1   & 7);
    let rank: u64  =  ((square_two | 7) -  sq1) >> 3 ;
    let line: u64  =      ((file  &  7) - 1) & a2_to_a7; /* a2_to_a7 if same file */
    line += 2 * (((rank  &  7) - 1) >> 58); /* b1_to_g1 if same rank */
    line += (((rank - file) & 15) - 1) & b2g7; /* b2_to_g7 if same diagonal */
    line += (((rank + file) & 15) - 1) & h1b7; /* h1_to_b7 if same antidiag */
    line *= btwn & -btwn; /* mul acts like shift by smaller square */
    line & btwn   /* return the bits on that line in-between */
 }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let board = Board();
    let light_pieces = board.light_pieces.view_bits::<Lsb0>();
    // Print the board states for the eight different bit boards that I setup.
    println!("Light Pieces: {light_pieces}, Dark Pieces: {dark_pieces}", light_pieces, dark_pieces);
    
    // Define the different attack patterns for the pawns...
    println!("");

    // Define the different attack patterns for the knights...
    println!("");

    Ok(())
}
