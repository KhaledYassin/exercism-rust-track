const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

pub fn actions(n: u8) -> Vec<&'static str> {
    let binary = format!("{:05b}", n % 32);

    let mut handshake: Vec<&str> = Vec::new();

    for (i, c) in binary.chars().rev().enumerate() {
        if c == '1' {
            if i == 4 {
                handshake.reverse();
            } else {
                handshake.push(ACTIONS[i]);
            }
        }
    }

    handshake
}
