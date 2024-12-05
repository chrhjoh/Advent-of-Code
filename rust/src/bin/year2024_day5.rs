use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let mut parts = data.split("\n\n");
    let rules = parse_rules(parts.next().unwrap());
    let updates = parts.next().unwrap();
    updates
        .lines()
        .map(|raw| raw.split(",").map(|x| x.parse().unwrap()).collect())
        .filter(|update| check_order(&update, &rules))
        .map(|stack| *stack.get(stack.len().div_ceil(2) - 1).unwrap())
        .sum()
}
fn exercise2(data: &str) -> i64 {
    let mut parts = data.split("\n\n");
    let rules = parse_rules(parts.next().unwrap());
    let updates = parts.next().unwrap();
    updates
        .lines()
        .map(|raw| raw.split(",").map(|x| x.parse().unwrap()).collect())
        .filter(|update| !check_order(&update, &rules))
        .map(|update| process_update(&update, &rules))
        .map(|stack| *stack.get(stack.len().div_ceil(2) - 1).unwrap())
        .sum()
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_rules(rules: &str) -> Vec<OrderingRule> {
    let mut result = Vec::new();
    for rule in rules.lines() {
        let mut parts = rule.split("|");
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        result.push(OrderingRule::new(start, end));
    }
    return result;
}

struct OrderingRule {
    first: i64,
    after: i64,
}

impl OrderingRule {
    fn new(first: i64, after: i64) -> OrderingRule {
        OrderingRule { first, after }
    }

    fn rule_applies(&self, first: i64, after: i64) -> bool {
        return self.first == first && self.after == after;
    }

    fn rule_applies_reverse(&self, first: i64, after: i64) -> bool {
        return self.first == after && self.after == first;
    }
}
fn process_update(update: &Vec<i64>, rules: &Vec<OrderingRule>) -> Vec<i64> {
    let mut stack = Vec::new();
    update.iter().for_each(|item| {
        let position = find_position_in_stack(&stack, *item, rules);
        stack.insert(position, *item);
    });
    return stack;
}

fn find_position_in_stack(stack: &Vec<i64>, value: i64, rules: &Vec<OrderingRule>) -> usize {
    let mut index = 0;
    for &item in stack.iter() {
        for rule in rules {
            if rule.rule_applies_reverse(item, value) {
                return index;
            }
        }
        index += 1;
    }
    return index;
}

fn check_order(stack: &Vec<i64>, rules: &Vec<OrderingRule>) -> bool {
    for rule in rules {
        for i in 0..stack.len() - 1 {
            for j in i + 1..stack.len() {
                if rule.rule_applies_reverse(stack[i], stack[j]) {
                    return false;
                }
            }
        }
    }
    return true;
}
