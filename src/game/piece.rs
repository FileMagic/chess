use super::board::Square;

#[derive(PartialEq, Debug)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

impl PieceType {
  fn get_bishop_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_king_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_knight_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_pawn_moves(from: Square, promotion: bool, en_passant: bool) -> Vec<Move>{
  unimplemented!();
  }
  fn get_queen_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
  fn get_rook_moves(from: Square) -> Vec<Move>{
  unimplemented!();
  }
}

#[derive(PartialEq, Debug)]
pub struct Move {
  from: Square,
  to: Square,
  piece_type: PieceType,
  capture: bool,
  promotion: Option<PieceType>,
  en_passant: bool,
}

impl std::fmt::Display for Move{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "from: {:#?}, to: {:#?}, piece_type: {:#?}, capture: {:#?}, promotion: {:#?},", self.from, self.to, self.piece_type, self.capture, self.promotion)
  }
}