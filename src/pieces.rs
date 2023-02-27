#[derive(PartialEq, Debug)]
enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

impl PieceType {
  fn get_pawn_moves(from: Square, promotion: bool, en_passant: bool) -> Vec<Move>{
  unimplemented!();
  }
  fn get_knight_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_bishop_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_rook_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_queen_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_king_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
}