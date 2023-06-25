pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let n = items.len();
    let mut dp = vec![vec![0; (max_weight + 1) as usize]; n + 1];

    for i in 1..=n {
        for w in 1..=max_weight {
            if items[i - 1].weight <= w {
                dp[i][w as usize] = dp[i - 1][w as usize]
                    .max(items[i - 1].value + dp[i - 1][(w - items[i - 1].weight) as usize]);
            } else {
                dp[i][w as usize] = dp[i - 1][w as usize];
            }
        }
    }

    dp[n][max_weight as usize]
}
