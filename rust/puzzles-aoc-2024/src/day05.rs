use crate::get_input_as_string;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = get_input_as_string("day05", "input");

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: String) {
    let updates_middle_pages_sum = find_update_middle_page_number_sum(input);
    println!("Part one: {}", updates_middle_pages_sum);
}

fn part_two(input: String) {
    let incorrectly_ordered_sum = order_incorrectly_ordered_updates_and_sum(input);
    println!("Part two: {}", incorrectly_ordered_sum);
}

fn find_update_middle_page_number_sum(input: String) -> u32 {
    let (rules, updates) = get_ordering_rules_and_updates(input);
    updates
        .iter()
        .filter(|update| update.is_ordered_according_to_rules(&rules))
        .map(|update| update.get_middle_page_number())
        .sum()
}

fn order_incorrectly_ordered_updates_and_sum(input: String) -> u32 {
    let (rules, updates) = get_ordering_rules_and_updates(input);
    updates
        .iter()
        .filter(|update| !update.is_ordered_according_to_rules(&rules))
        .map(|update| update.order_according_to_rules(&rules))
        .map(|update| update.get_middle_page_number())
        .sum()
}

fn get_ordering_rules_and_updates(input: String) -> (ParsedOrderingRules, Vec<Update>) {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let rules: ParsedOrderingRules =
        ordering_rules
            .lines()
            .map(OrderingRule::new)
            .fold(HashMap::new(), |mut acc, rule| {
                // get entry for the key rule.before, or inserts a new HashSet if the key is not present
                acc.entry(rule.before).or_default().insert(rule.after);
                acc
            });
    let updates: Vec<Update> = updates.lines().map(Update::new).collect();

    (rules, updates)
}

type ParsedOrderingRules = HashMap<u32, HashSet<u32>>;

#[derive(Debug, Clone, PartialEq)]
struct OrderingRule {
    before: u32,
    after: u32,
}

impl OrderingRule {
    fn new(input: &str) -> Self {
        let (before, after) = input.split_once("|").unwrap();
        Self {
            before: before.parse::<u32>().unwrap(),
            after: after.parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Update {
    page_numbers: Vec<u32>,
}

impl Update {
    fn new(input: &str) -> Self {
        Self {
            page_numbers: input
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn is_ordered_according_to_rules(&self, rules: &ParsedOrderingRules) -> bool {
        for (i, page_number) in self.page_numbers.iter().enumerate() {
            for j in 0..i {
                if let Some(matching_rules) = rules.get(page_number) {
                    if matching_rules.contains(&self.page_numbers[j]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_middle_page_number(&self) -> u32 {
        self.page_numbers[self.page_numbers.len() / 2]
    }

    fn order_according_to_rules(&self, rules: &ParsedOrderingRules) -> Update {
        let mut ordered_page_numbers = self.page_numbers.clone();
        for i in 0..ordered_page_numbers.len() {
            let earlier = rules
                .get(&self.page_numbers[i]) // get rules associated with the current page number
                .iter()
                .flat_map(|x| x.iter()) // iterate over the HashSet
                .flat_map(|x| ordered_page_numbers.iter().position(|y| y == x)) // map rule to position
                .min() // find minimum position
                .filter(|x| x < &i); // found position must be earlier than the current index i / page number

            if let Some(earlier_position) = earlier {
                // if an earlier position was found
                // remove element at index i
                ordered_page_numbers.remove(i);
                // insert element at earlier position
                ordered_page_numbers.insert(earlier_position, self.page_numbers[i]);
            }
        }
        Update {
            page_numbers: ordered_page_numbers,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day05::Update;

    fn get_example_input() -> String {
        String::from(
            "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        )
    }

    #[test]
    fn test_update_parsing() {
        let update = Update::new("75,47,61,53,29");
        assert_eq!(update.page_numbers, [75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_ordering_rule_parsing() {
        let ordering_rule = super::OrderingRule::new("47|53");
        assert_eq!(ordering_rule.before, 47);
        assert_eq!(ordering_rule.after, 53);
    }

    #[test]
    fn test_get_ordering_rules_and_updates() {
        let input = get_example_input();
        let (ordering_rules, updates) = super::get_ordering_rules_and_updates(input);
        assert_eq!(ordering_rules.len(), 6);
        let expected_keys = [47, 97, 75, 61, 29, 53];
        assert_eq!(
            ordering_rules.keys().all(|key| expected_keys.contains(key)),
            true
        );
        assert_eq!(updates.len(), 6);
    }

    #[test]
    fn test_check_correctly_ordered_updates() {
        let input = get_example_input();
        let (ordering_rules, _updates) = super::get_ordering_rules_and_updates(input);
        assert_eq!(
            Update::new("75,47,61,53,29").is_ordered_according_to_rules(&ordering_rules),
            true
        );
        assert_eq!(
            Update::new("97,61,53,29,13").is_ordered_according_to_rules(&ordering_rules),
            true
        );
        assert_eq!(
            Update::new("75,29,13").is_ordered_according_to_rules(&ordering_rules),
            true
        );
        assert_eq!(
            Update::new("75,97,47,61,53").is_ordered_according_to_rules(&ordering_rules),
            false
        );
        assert_eq!(
            Update::new("61,13,29").is_ordered_according_to_rules(&ordering_rules),
            false
        );
        assert_eq!(
            Update::new("97,13,75,29,47").is_ordered_according_to_rules(&ordering_rules),
            false
        );
    }

    #[test]
    fn test_order_incorrectly_ordered_update() {
        let input = get_example_input();
        let (ordering_rules, _updates) = super::get_ordering_rules_and_updates(input);
        assert_eq!(
            Update::new("75,97,47,61,53").order_according_to_rules(&ordering_rules),
            Update::new("97,75,47,61,53")
        );
        assert_eq!(
            Update::new("61,13,29").order_according_to_rules(&ordering_rules),
            Update::new("61,29,13")
        );
        assert_eq!(
            Update::new("97,13,75,29,47").order_according_to_rules(&ordering_rules),
            Update::new("97,75,47,29,13")
        );
    }

    #[test]
    fn test_find_update_middle_page_number_sum_in_example_should_be_143() {
        let input = get_example_input();
        assert_eq!(super::find_update_middle_page_number_sum(input), 143);
    }

    #[test]
    fn test_order_incorrectly_ordered_updates_and_sum_in_example_should_be_123() {
        let input = get_example_input();
        assert_eq!(super::order_incorrectly_ordered_updates_and_sum(input), 123);
    }
}
