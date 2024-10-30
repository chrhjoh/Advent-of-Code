use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let initial = data.split(",").map(|x| x.parse().unwrap()).collect();
    return spoken_game(initial, 2020);
}
fn exercise2(data: &str) -> i64 {
    let initial = data.split(",").map(|x| x.parse().unwrap()).collect();
    return spoken_game(initial, 30000000);
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn spoken_game(initial: Vec<i64>, total_turns: usize) -> i64 {
    let mut spoken_turns: Vec<i64> = vec![0; total_turns];
    for (i, num) in initial.iter().enumerate() {
        spoken_turns[*num as usize] = i as i64 + 1;
    }
    let mut spoken_word = *initial.last().unwrap();
    for turn in initial.len()..total_turns {
        let last_spoken_turn = spoken_turns[spoken_word as usize];

        spoken_turns[spoken_word as usize] = turn as i64;

        spoken_word = if last_spoken_turn == 0 {
            0
        } else {
            turn as i64 - last_spoken_turn
        };
    }
    return spoken_word as i64;
}
