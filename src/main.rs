use board::Board;

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