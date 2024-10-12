use std::collections::HashSet;

use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let groups = data.split("\n\n");
    let answer: i64 = groups.map(|group| count_group_any_questions(group)).sum();
    return answer;
}
fn exercise2(data: &str) -> i64 {
    let groups = data.split("\n\n");
    let answer: i64 = groups.map(|group| count_group_every_questions(group)).sum();
    return answer;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn count_group_any_questions(group: &str) -> i64 {
    let mut seen_answers: Vec<char> = vec![];
    for person in group.split_whitespace() {
        for answer in person.chars() {
            if !seen_answers.contains(&answer) {
                seen_answers.push(answer)
            }
        }
    }
    return seen_answers.len() as i64;
}

fn count_group_every_questions(group: &str) -> i64 {
    let mut persons: Vec<&str> = group.split_whitespace().collect();
    let mut all_answers: HashSet<char> = persons.pop().unwrap().chars().collect();
    for person in persons {
        let new_answer_set: HashSet<char> = person.chars().collect();
        all_answers = all_answers.intersection(&new_answer_set).copied().collect();
    }
    return all_answers.len() as i64;
}
