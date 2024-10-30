use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let mut players = data.split("\n\n").map(|player| {
        let mut lines = player.lines();
        let name = lines.next().unwrap().trim_end_matches(':');
        let cards = lines.map(|line| line.parse().unwrap()).collect();
        Player::new(name, cards)
    });
    let mut player1 = players.next().unwrap();
    let mut player2 = players.next().unwrap();
    while !player1.cards.is_empty() && !player2.cards.is_empty() {
        play_round(&mut player1, &mut player2);
    }
    let winner = determine_winner(&player1, &player2);
    return winner.score();
}
fn exercise2(data: &str) -> i64 {
    let mut players = data.split("\n\n").map(|player| {
        let mut lines = player.lines();
        let name = lines.next().unwrap().trim_end_matches(':');
        let cards = lines.map(|line| line.parse().unwrap()).collect();
        Player::new(name, cards)
    });
    let mut player1 = players.next().unwrap();
    let mut player2 = players.next().unwrap();
    let mut previous_rounds = Vec::new();
    while !player1.cards.is_empty() && !player2.cards.is_empty() {
        play_recursive_round(&mut player1, &mut player2, &mut previous_rounds);
    }
    let winner = determine_winner(&player1, &player2);
    return winner.score();
}

fn main() {
    utils::run(exercise1, exercise2);
}

struct Player {
    name: String,
    cards: Vec<i64>,
}

impl Player {
    fn new(name: &str, cards: Vec<i64>) -> Player {
        Player {
            name: name.to_string(),
            cards: cards,
        }
    }
    fn draw(&mut self) -> i64 {
        return self.cards.remove(0);
    }
    fn add(&mut self, card: i64) {
        self.cards.push(card);
    }
    fn score(&self) -> i64 {
        let mut score = 0;
        for i in 0..self.cards.len() {
            score += self.cards[i] * (self.cards.len() as i64 - i as i64);
        }
        return score;
    }
}

fn play_round(player1: &mut Player, player2: &mut Player) {
    let card1 = player1.draw();
    let card2 = player2.draw();
    if card1 > card2 {
        player1.add(card1);
        player1.add(card2);
    } else {
        player2.add(card2);
        player2.add(card1);
    }
}

fn determine_winner<'a>(player1: &'a Player, player2: &'a Player) -> &'a Player {
    if player1.cards.is_empty() {
        return player2;
    } else {
        return player1;
    }
}

fn play_recursive_round(
    player1: &mut Player,
    player2: &mut Player,
    previous_rounds: &mut Vec<(Vec<i64>, Vec<i64>)>,
) {
    if previous_rounds.contains(&(player1.cards.clone(), player2.cards.clone())) {
        player2.cards.clear();
        return;
    }
    previous_rounds.push((player1.cards.clone(), player2.cards.clone()));
    let card1 = player1.draw();
    let card2 = player2.draw();

    if player1.cards.len() as i64 >= card1 && player2.cards.len() as i64 >= card2 {
        let mut sub_player1 = Player::new(&player1.name, player1.cards[0..card1 as usize].to_vec());
        let mut sub_player2 = Player::new(&player2.name, player2.cards[0..card2 as usize].to_vec());
        let previous_rounds = &mut Vec::new();
        while !sub_player1.cards.is_empty() && !sub_player2.cards.is_empty() {
            play_recursive_round(&mut sub_player1, &mut sub_player2, previous_rounds);
        }
        let winner = determine_winner(&sub_player1, &sub_player2);
        if winner.name == player1.name {
            player1.add(card1);
            player1.add(card2);
        } else {
            player2.add(card2);
            player2.add(card1);
        }
    } else {
        if card1 > card2 {
            player1.add(card1);
            player1.add(card2);
        } else {
            player2.add(card2);
            player2.add(card1);
        }
    }
}
