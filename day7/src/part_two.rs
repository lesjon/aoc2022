use super::part_one::{File, FileSystem, parse_line, LineType};

const TOTAL_SPACE: u32 = 70000000;
const REQUIRED_SPACE: u32 = 30000000;

pub fn main() {
    let input = include_str!("input.txt");
    println!("input: {}", input);
    let mut file_system = FileSystem::new();
    
    for line in input.lines() {
        println!("{}", line);
        match parse_line(line) {
            (LineType::CdRoot, _) => {
                println!("Found 'cd /'");
                file_system.cwd = String::from("/");
            },
            (LineType::CdUp, _) => {
                println!("Found 'cdup', cwd: {}", file_system.cwd);
                let mut last_slash:  usize = 0;
                for (i, c) in file_system.cwd.char_indices() {
                    if c == '/' {
                        last_slash = i;
                    }
                }
                file_system.cwd.drain(last_slash..);
                println!("Result cwd: {}", file_system.cwd);
            },
            (LineType::Directory, Some(name)) => {
                println!("Found 'dir {}'", name);
                file_system.new_dir_from_cli(line);
            },
            (LineType::File, Some(name)) => {
                println!("Found 'file {}'", name);
                if let Some(cwd) = file_system.get_cwd_dir_mut(){
                    let file = File::from(line);
                    cwd.files.push(file);
                }
            },
            (LineType::Cd, Some(name)) => {
                println!("Found 'cd {}'", name);
                file_system.cwd.push_str("/");
                file_system.cwd.push_str(name);
            },
            (LineType::Cd, None) => {
                println!("Found 'cd None'");
            },
            (LineType::Directory, None) => {
                println!("Found 'dir None'");
            },
            (LineType::File, None) => {
                println!("Found 'file None'");
            },
            (LineType::Ls, _) => {
                println!("Found 'ls'");
            }
        }
    }
    println!("{:?}", file_system);
    let root = file_system.get_root_dir().unwrap();
    let root_size = root.size(&file_system);
    let needed_dir_size = REQUIRED_SPACE - (TOTAL_SPACE - root_size);
    let mut smallest_big_enough = u32::MAX;
    for (name, dir) in file_system.directories.iter() {
        let dir_size = dir.size(&file_system);
        println!("{}: {}", name, dir_size);
        if needed_dir_size < dir_size {
            println!("needed_dir_size < dir_size");
            if dir_size < smallest_big_enough {
                println!("dir_size < smallest_big_enough");
                smallest_big_enough = dir_size;
            }
        }
    }
    println!("smallest_big_enough: {}", smallest_big_enough);
    
}
