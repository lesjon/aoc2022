
enum State {
    Crates,
    Instructions
}

pub fn read_lines_up_to_empty_line_from_string(text: &str) -> (Vec<&str>, Vec<&str>) {
    let mut crate_lines:Vec<&str> = vec![];
    let mut instruction_lines:Vec<&str> = vec![];
    let mut state = State::Crates;

    for line in text.lines() {
        if line.is_empty() {
            state = State::Instructions;
            continue;
        }
        match state {
            State::Crates => crate_lines.push(line),
            State::Instructions => instruction_lines.push(line),
        }
    }

    (crate_lines, instruction_lines)
}
