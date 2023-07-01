use bowling::BowlingGame;
fn main() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4); // 13
    let _ = game.roll(3); // 3

    for _ in 0..17 {
        let _ = game.roll(0);
    }

    let score = game.score();
    println!("Score: {:?}", score)
}