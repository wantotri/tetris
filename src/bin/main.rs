use tetris::tetris::*;

fn main() {
    let mut game = Game::new();
    // game.print();
    for _ in 0..140 {
        // game.current_tetromino.rotate();
        game.tick();
    }
    game.print();
    println!("Next Field");
    println!("{}", game.get_nextfield_str());

    // game.crush_row(23, 0);
    // game.crush_row(23, 0);
    // println!("{}", game);
}