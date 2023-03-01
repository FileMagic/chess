mod game;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // fucking make the board
    let board = game::board::Board::default();
    
    // Print the board state.
    println!("{}", board);
    
    // Move b2 to b4.
    let created_move = board.create_move(game::board::Square::B2, game::board::Square::B5);

    println!("{:?}", created_move);
    Ok(())
}