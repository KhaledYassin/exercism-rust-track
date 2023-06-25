pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => calculate_number_score(dice, 1),
        Category::Twos => calculate_number_score(dice, 2),
        Category::Threes => calculate_number_score(dice, 3),
        Category::Fours => calculate_number_score(dice, 4),
        Category::Fives => calculate_number_score(dice, 5),
        Category::Sixes => calculate_number_score(dice, 6),
        Category::FullHouse => calculate_full_house_score(dice),
        Category::FourOfAKind => calculate_four_of_a_kind_score(dice),
        Category::LittleStraight => calculate_little_straight_score(dice),
        Category::BigStraight => calculate_big_straight_score(dice),
        Category::Choice => calculate_choice_score(dice),
        Category::Yacht => calculate_yacht_score(dice),
    }
}

fn calculate_number_score(dice: Dice, number: u8) -> u8 {
    dice.iter().filter(|&&d| d == number).sum()
}

fn calculate_full_house_score(dice: Dice) -> u8 {
    let mut counts = [0; 6];
    for &die in dice.iter() {
        counts[die as usize - 1] += 1;
    }
    let has_two_of_a_kind = counts.contains(&2);
    let has_three_of_a_kind = counts.contains(&3);
    if has_two_of_a_kind && has_three_of_a_kind {
        dice.iter().sum()
    } else {
        0
    }
}

fn calculate_four_of_a_kind_score(dice: Dice) -> u8 {
    let mut counts = [0; 6];
    for &die in dice.iter() {
        counts[die as usize - 1] += 1;
    }
    if let Some(die_value) = counts.iter().position(|&count| count >= 4) {
        (die_value as u8 + 1) * 4
    } else {
        0
    }
}

fn calculate_little_straight_score(dice: Dice) -> u8 {
    let mut sorted_dice = dice.to_vec();
    sorted_dice.sort();

    if sorted_dice == [1, 2, 3, 4, 5] {
        30
    } else {
        0
    }
}

fn calculate_big_straight_score(dice: Dice) -> u8 {
    let mut sorted_dice = dice.to_vec();
    sorted_dice.sort(); 
    
    if sorted_dice == [2, 3, 4, 5, 6] {
        30
    } else {
        0
    }
}

fn calculate_choice_score(dice: Dice) -> u8 {
    dice.iter().sum()
}

fn calculate_yacht_score(dice: Dice) -> u8 {
    let first_die = dice[0];
    if dice.iter().all(|&die| die == first_die) {
        50
    } else {
        0
    }
}
