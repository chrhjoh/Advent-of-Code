use aocsuite::utils;
use std::collections::HashMap;
fn exercise1(data: &str) -> i64 {
    let rules = parse_rules(data.split("\n\n").next().unwrap());
    let messages = data.split("\n\n").nth(1).unwrap();
    let filtered = messages
        .lines()
        .filter(|message| {
            let rule = rules.get(&0).unwrap();
            rule.prefix_match(message, &rules) == vec![""]
        })
        .collect::<Vec<&str>>();
    filtered.len() as i64
}
fn exercise2(data: &str) -> i64 {
    let mut rules = parse_rules(data.split("\n\n").next().unwrap());
    rules.insert(8, Rule::SubRules(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]));
    let messages = data.split("\n\n").nth(1).unwrap();

    let filtered = messages
        .lines()
        .filter(|message| {
            let rule = rules.get(&0).unwrap();
            rule.prefix_match(message, &rules).contains(&"")
        })
        .collect::<Vec<&str>>();

    filtered.len() as i64
}

fn main() {
    utils::run(exercise1, exercise2);
}

enum Rule {
    Char(char),
    SubRules(Vec<Vec<i64>>),
}

impl Rule {
    fn from_str(s: &str) -> Rule {
        if s.contains("\"") {
            return Rule::Char(s.chars().nth(1).unwrap());
        }
        let mut sub_rules = Vec::new();
        for sub_rule in s.split("|") {
            let mut sub_rule_vec = Vec::new();
            for rule in sub_rule.trim().split(" ") {
                sub_rule_vec.push(rule.parse::<i64>().unwrap());
            }
            sub_rules.push(sub_rule_vec);
        }
        return Rule::SubRules(sub_rules);
    }
    fn prefix_match<'a>(&self, src: &'a str, rules: &HashMap<i64, Rule>) -> Vec<&'a str> {
        match self {
            Rule::Char(c) => src.strip_prefix(|d| *c == d).into_iter().collect(),
            Rule::SubRules(rs) => rs
                .iter()
                .flat_map(|alt| {
                    alt.iter().fold(vec![src], |v, id| {
                        let r = rules.get(&id).unwrap();
                        v.into_iter()
                            .flat_map(|s| r.prefix_match(s, rules).into_iter())
                            .collect()
                    })
                })
                .collect(),
        }
    }
}

fn parse_rules(rules: &str) -> HashMap<i64, Rule> {
    let mut rule_map = HashMap::new();
    for rule in rules.lines() {
        let mut parts = rule.split(": ");
        let rule_num = parts.next().unwrap().parse::<i64>().unwrap();
        let rule_str = parts.next().unwrap();
        rule_map.insert(rule_num, Rule::from_str(rule_str));
    }
    return rule_map;
}
