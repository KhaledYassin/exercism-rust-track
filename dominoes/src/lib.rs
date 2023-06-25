pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let mut dominoes = input.to_vec();
    let domino = dominoes[0];

    find_chain(&mut dominoes[1..], domino.1, domino.0).then_some(dominoes.to_vec())
}

fn find_chain(dominoes: &mut [(u8, u8)], next_head: u8, tail: u8) -> bool {
    if dominoes.is_empty() && next_head == tail {
        // When there are no more dominos, we have reached the end of the recursion.
        return true;
    }

    // This short circuits on the first domino that matches the next head.
    (0..dominoes.len()).any(|i| {
        if dominoes[i].1 == next_head {
            // If the tail of the domino matches the next head, we rotate it.
            dominoes[i] = (dominoes[i].1, dominoes[i].0);
        }

        if dominoes[i].0 != next_head {
            // If this domino does not match at all, we keep going or end the recursion.
            false
        } else {
            // The matching domino gets brought to the front (connected) just after the next head.
            dominoes.swap(0, i);
            let domino = dominoes[0];
            find_chain(&mut dominoes[1..], domino.1, tail)
        }
    })
}
