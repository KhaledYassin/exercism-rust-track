use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord)]
struct TeamTally {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl std::fmt::Display for TeamTally {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name, self.matches_played, self.wins, self.draws, self.losses, self.points
        );
        write!(f, "{}", string)
    }
}

impl PartialOrd for TeamTally {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.points == other.points {
            return Some(self.name.cmp(&other.name));
        }

        Some(self.points.cmp(&other.points).reverse())
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = HashMap::new();
    let matches = match_results.split_terminator('\n');

    for m in matches {
        let line: Vec<&str> = m.split(';').collect();

        let home_team = line[0];
        let away_team = line[1];
        let result = line[2];
        let opposite_result = opposite_result(result);

        let iteration = [(home_team, result), (away_team, opposite_result)];

        for (team_name, result) in iteration {
            let team = table.entry(team_name).or_insert(TeamTally {
                name: String::from(team_name),
                matches_played: 0,
                wins: 0,
                draws: 0,
                losses: 0,
                points: 0,
            });

            track_score(team, result)
        }
    }

    let mut sorted_table = table.iter().map(|(_, v)| v).collect::<Vec<_>>();
    sorted_table.sort_unstable();

    (String::from("Team                           | MP |  W |  D |  L |  P\n")
        + &sorted_table
            .iter()
            .map(|team| team.to_string())
            .collect::<Vec<_>>()
            .join("\n"))
        .trim()
        .to_string()
}

fn track_score(team: &mut TeamTally, result: &str) {
    team.matches_played += 1;

    match result {
        "win" => {
            team.wins += 1;
            team.points += 3;
        }
        "loss" => {
            team.losses += 1;
        }
        "draw" => {
            team.draws += 1;
            team.points += 1;
        }
        _ => panic!("Invalid Result")
    }
}

fn opposite_result(result: &str) -> &str {
    match result {
        "win" => "loss",
        "loss" => "win",
        "draw" => "draw",
        _ => panic!("Invalid Result"),
    }
}
