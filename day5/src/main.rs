use std::collections::HashMap;

mod read_lines;

fn find_number_indices(lines: &Vec<&str>) -> Vec<usize> {
    if let Some(last_line) = lines.last() {
        let mut indices = vec![];
        for (i, c) in last_line.char_indices() {
            if c.is_digit(10) {
                indices.push(i);
            }
        }
        indices
    } else {
        vec![]
    }
}


fn crates_strings_to_map(mut crates: Vec<&str>) -> HashMap<String, Vec<char>> {
    let indices = find_number_indices(&crates);
    let mut crates_map: HashMap<String, Vec<char>> = HashMap::new();
    let index_line = crates.pop().unwrap();
    let mut keys: HashMap<String, usize> = HashMap::new();
    for i in indices {
        let key = index_line.char_indices().nth(i).map(|(_, c)| c).unwrap();
        keys.insert(key.to_string(), i);
        crates_map.insert(key.to_string(), vec![]);
    }
    crates.reverse();
    for crate_line in crates {
        for (i_char, i) in keys.iter() {
            let stack = crates_map.get_mut(i_char).unwrap();
            let crate_letter = crate_line.char_indices().nth(*i).map(|(_, c)| c).unwrap();
            if !crate_letter.is_whitespace() {
                stack.push(crate_letter);
            }
        }
    }
    crates_map
}

fn all_numeric(word: &&str) -> bool {
    for c in word.chars() {
        if !c.is_digit(10) {
            return false
        }
    }
    true
}

fn get_three_numbers(instruction: &str) -> (&str, &str, &str) {
    println!("get_three_numbers");
    let words = instruction.split_whitespace();
    println!("words: {:?}", words);
    let numbers = words.filter(all_numeric);
    let numbers_to_filter: Vec<&str> = numbers.collect();
    println!("numbers_to_filter: {:?}", numbers_to_filter);
    (numbers_to_filter.get(0).unwrap(), numbers_to_filter.get(1).unwrap(), numbers_to_filter.get(2).unwrap())
}

fn execute_instruction_9000(stacks: &mut HashMap<String, Vec<char>>, instruction: &str) {
    println!("execute_instruction");
    let (n0, n1, n2) = get_three_numbers(instruction);
    println!("(n0, n1, n2): ({},{},{})", n0, n1, n2);
    let n_crates = n0.parse().unwrap();
    for _ in 0..n_crates {
        let from_stack = stacks.get_mut(n1).unwrap();
        let crate_to_move = from_stack.pop().unwrap();
        let to_stack = stacks.get_mut(n2).unwrap();
        to_stack.push(crate_to_move);
    }
}

fn execute_instruction_9001(stacks: &mut HashMap<String, Vec<char>>, instruction: &str) {
    println!("execute_instruction");
    let (n0, n1, n2) = get_three_numbers(instruction);
    println!("(n0, n1, n2): ({},{},{})", n0, n1, n2);
    let n_crates = n0.parse().unwrap();
    let mut crate_stack = vec![];
    for _ in 0..n_crates {
        let from_stack = stacks.get_mut(n1).unwrap();
        crate_stack.push(from_stack.pop().unwrap());
    }
    crate_stack.reverse();
    stacks.get_mut(n2).unwrap().extend(crate_stack);
}

fn show_sorted(stacks: &mut HashMap<String, Vec<char>>) {
    let mut sorted_entries: Vec<_> = stacks.into_iter().collect();
    sorted_entries.sort_by_key(|(k, _)| *k);

    for (key, value) in sorted_entries {
        println!("{}: {}", key, value.last().unwrap());
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (crates, instructions) = read_lines::read_lines_up_to_empty_line_from_string(input); 
    let mut stacks = crates_strings_to_map(crates);
    println!("Start stacks{:?}", stacks);
    for instruction in instructions {
        println!("instruction: '{}'", instruction);
        execute_instruction_9001(&mut stacks, instruction);
        println!("result stacks{:?}", stacks);
    }
    show_sorted(&mut stacks);
}
