use std::collections::HashSet;

fn columns(input: &str) -> Vec<Vec<u8>> {
    let mut cols: Vec<Vec<u8>> = vec![];
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    for (i, c) in first_line.bytes().enumerate() {
        cols.push(vec![c]);
    }
    for line in lines {
        for (i, c) in line.bytes().enumerate() {
            cols.get_mut(i).unwrap().push(c);
        }
    }
    cols
}

fn seen(input: &str) -> usize {
    let lines = input.lines();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in lines.enumerate() {
        let seen_from_left = seen_from_left(line);
        for l in seen_from_left {
            seen.insert((i, l));
        }
        let seen_from_right = seen_from_right(line);
        for r in seen_from_right {
            seen.insert((i, r));
        }
    }
    let cols = columns(input);    
    for (i, col) in cols.iter().enumerate() {
        let seen_from_top = seen_from_top(col);
        for t in seen_from_top {
            seen.insert((t, i));
        }
        let seen_from_bottom = seen_from_bottom(col);
        for b in seen_from_bottom {
            seen.insert((b, i));
        }
    }
    println!("seen coordinates: {:?}", seen);
    seen.len()
}

fn seen_from_left(line: &str) -> Vec<usize> {
    let mut seen = vec![];
    let mut max = b'0' - 1;
    for (i, c) in line.bytes().enumerate() {
        if max < c {
            max = c;
            seen.push(i);
        }
    }
    seen
}

fn seen_from_right(line: &str, ) -> Vec<usize> {
    let mut seen = vec![];
    let mut max = b'0' - 1;
    for (i, c) in line.bytes().enumerate().rev() {
        if max < c {
            max = c;
            seen.push(i);
        }
    }
    seen
}

fn seen_from_top(line: &Vec<u8>) -> Vec<usize> {
    let mut seen = vec![];
    let mut max = b'0' - 1;
    for (i, c) in line.iter().enumerate() {
        if max < *c {
            max = *c;
            seen.push(i);
        }
    }
    seen
}

fn seen_from_bottom(line: &Vec<u8>) -> Vec<usize> {
    let mut seen = vec![];
    let mut max = b'0' - 1;
    for (i, c) in line.iter().enumerate().rev() {
        if max < *c {
            max = *c;
            seen.push(i);
        }
    }
    seen
}


fn main() {
    let input = include_str!("input.txt");
    println!("input:\n{}", input);
    let seen = seen(input);
    println!("seen: {}", seen);
}
