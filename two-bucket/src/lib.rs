use std::cmp::min;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let gcd = gcd(capacity_1, capacity_2);

    if goal % gcd != 0 {
        return None;
    }

    let (bucket_a, bucket_b, mut liters_a, mut liters_b, mut goal_bucket) = match start_bucket {
        Bucket::One => (capacity_1, capacity_2, capacity_1, 0_u8, Bucket::One),
        Bucket::Two => (capacity_2, capacity_1, capacity_2, 0_u8, Bucket::Two),
    };

    let mut moves = 1_u8;

    while liters_a != goal {
        moves += 1;
        if liters_a == 0 {
            liters_a = bucket_a;
        } else if liters_b == bucket_b {
            liters_b = 0;
        } else if liters_b == 0 && bucket_b == goal {
            liters_b += bucket_b;
            goal_bucket = if goal_bucket == Bucket::One {
                Bucket::Two
            } else {
                Bucket::One
            };
            break;
        } else {
            let transferred_liters = min(liters_a, bucket_b - liters_b);
            liters_a -= transferred_liters;
            liters_b += transferred_liters;

            if liters_b == goal {
                goal_bucket = if goal_bucket == Bucket::One {
                    Bucket::Two
                } else {
                    Bucket::One
                };
                break;
            }
        }
    }

    Some(BucketStats {
        moves: moves,
        goal_bucket: goal_bucket,
        other_bucket: if start_bucket == &goal_bucket {
            liters_b
        } else {
            liters_a
        },
    })
}

fn gcd(a: u8, b: u8) -> u8 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}
