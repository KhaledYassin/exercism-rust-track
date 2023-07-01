use poker::winning_hands;
fn main() {
    let hand = &["3H 3D 3C 2S AH", "4H 4D 4S KH 9D"];

    let result = winning_hands(hand);

    print!("{:?}", result)
}
