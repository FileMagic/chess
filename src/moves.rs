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