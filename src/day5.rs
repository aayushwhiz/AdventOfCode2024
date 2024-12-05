use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io;

const PROBLEM_NUMBER: i32 = 2;

pub fn run() -> io::Result<()> {
    match PROBLEM_NUMBER {
        1 => problem_1(),
        2 => problem_2(),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Invalid problem number",
        )),
    }
}

fn problem_1() -> io::Result<()> {
    let content = fs::read_to_string("input/day5.txt").expect("Failed to read input file");

    let sections: Vec<&str> = content.split("\n\n").collect();
    if sections.len() != 2 {
        panic!("Invalid input format");
    }

    let rules = parse_rules(sections[0]);

    let updates = parse_pages(sections[1]);

    let mut total = 0;
    for update in updates {
        if is_update_valid(&update, &rules) {
            let middle_number = find_middle_number(&update);
            total += middle_number;
        }
    }

    println!("{}", total);
    Ok(())
}

fn problem_2() -> io::Result<()> {
    let content = fs::read_to_string("input/day5.txt").expect("Failed to read the file");

    let sections: Vec<&str> = content.split("\n\n").collect();
    if sections.len() != 2 {
        panic!("Invalid input format");
    }

    let rules = parse_rules(sections[0]);

    let updates = parse_pages(sections[1]);

    let mut total = 0;
    for update in updates {
        if !is_update_valid(&update, &rules) {
            if let Some(reordered_update) = reorder_update(&update, &rules) {
                let middle_number = find_middle_number(&reordered_update);
                total += middle_number;
            } else {
                println!("Could not reorder update: {:?}", update);
            }
        }
    }

    println!("{}", total);
    Ok(())
}

fn parse_rules(rules_section: &str) -> Vec<(i32, i32)> {
    let mut rules = Vec::new();
    for line in rules_section.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.trim().split('|').collect();
        if parts.len() != 2 {
            panic!("Invalid rule format: {}", line);
        }
        let x = parts[0].parse::<i32>().expect("Invalid number in rule");
        let y = parts[1].parse::<i32>().expect("Invalid number in rule");
        rules.push((x, y));
    }
    rules
}

fn parse_pages(updates_section: &str) -> Vec<Vec<i32>> {
    let mut updates = Vec::new();
    for line in updates_section.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let numbers: Vec<i32> = line
            .trim()
            .split(',')
            .map(|num| num.trim().parse::<i32>().expect("Invalid number in update"))
            .collect();
        updates.push(numbers);
    }
    updates
}

fn is_update_valid(update: &[i32], rules: &Vec<(i32, i32)>) -> bool {
    let mut position_map = HashMap::new();
    for (index, &number) in update.iter().enumerate() {
        position_map.insert(number, index);
    }

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (position_map.get(&x), position_map.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }
    true
}

fn reorder_update(update: &[i32], rules: &Vec<(i32, i32)>) -> Option<Vec<i32>> {
    let numbers_in_update: HashSet<i32> = update.iter().cloned().collect();

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();

    for &number in &numbers_in_update {
        graph.entry(number).or_default();
        in_degree.entry(number).or_default();
    }

    for &(x, y) in rules {
        if numbers_in_update.contains(&x) && numbers_in_update.contains(&y) {
            graph.get_mut(&x).unwrap().push(y);
            *in_degree.get_mut(&y).unwrap() += 1;
        }
    }

    let mut queue: VecDeque<i32> = VecDeque::new();

    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut ordered_numbers = Vec::new();

    while let Some(node) = queue.pop_front() {
        ordered_numbers.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if ordered_numbers.len() != numbers_in_update.len() {
        return None;
    }

    let position_in_topo: HashMap<i32, usize> = ordered_numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i))
        .collect();

    let max_topo_pos = position_in_topo.values().max().unwrap_or(&0) + 1;

    let mut position_map: HashMap<i32, usize> = HashMap::new();

    for (&number, &pos) in &position_in_topo {
        position_map.insert(number, pos);
    }

    let mut next_pos = max_topo_pos;
    for &number in update.iter() {
        position_map.entry(number).or_insert_with(|| {
            let pos = next_pos;
            next_pos += 1;
            pos
        });
    }

    let mut numbers_with_positions: Vec<(usize, i32)> = update
        .iter()
        .map(|&n| {
            let pos = *position_map.get(&n).unwrap();
            (pos, n)
        })
        .collect();

    numbers_with_positions.sort_by_key(|&(pos, _)| pos);

    let reordered_update = numbers_with_positions.into_iter().map(|(_, n)| n).collect();

    Some(reordered_update)
}

fn find_middle_number(update: &[i32]) -> i32 {
    let len = update.len();
    let middle_index = len / 2;
    update[middle_index]
}
