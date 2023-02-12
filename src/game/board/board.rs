use bitvec::prelude::*;

enum enumSquare {
    a1, a2, a3, a4, a5, a6, a7, a8,
    b1, b2, b3, b4, b5, b6, b7, b8,
    c1, c2, c3, c4, c5, c6, c7, c8,
    d1, d2, d3, d4, d5, d6, d7, d8,
    e1, e2, e3, e4, e5, e6, e7, e8,
    f1, f2, f3, f4, f5, f6, f7, f8,
    g1, g2, g3, g4, g5, g6, g7, g8,
    h1, h2, h3, h4, h5, h6, h7, h8
  }

  let a_file: u64             = 0x0101010101010101;
  let h_file: u64             = 0x8080808080808080;
  let 1st_rank: u64           = 0x00000000000000FF;
  let 8th_rank: u64           = 0xFF00000000000000;
  let a1_h8_diagonal: u64     = 0x8040201008040201;
  let h1_a8_antidiagonal: u64 = 0x0102040810204080;
  let light_squares: u64      = 0x55AA55AA55AA55AA;
  let dark_squares: u64       = 0xAA55AA55AA55AA55;