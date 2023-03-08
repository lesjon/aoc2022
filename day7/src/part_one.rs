use std::collections::HashMap;

const MAX_SIZE: u32 = 100000;

pub enum LineType {
    CdUp,
    CdRoot,
    Cd,
    Ls,
    File,
    Directory
}

pub fn parse_line(line: &str) -> (LineType, Option<&str>) {
    if line.starts_with("$ cd ..")     { return (LineType::CdUp, None); }
    else if line.starts_with("$ cd /") { return (LineType::CdRoot, None); }
    else if line.starts_with("$ cd ")  { return (LineType::Cd, Some(&line[5..])); }
    else if line.starts_with("$ ls")   { return (LineType::Ls, None); }
    else if line.starts_with("dir")    { return (LineType::Directory, Some(&line[4..])); }
    else {return (LineType::File, Some(&line[5..]))};
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct File {
    pub name: String,
    pub size: u32
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Directory {
    pub files: Vec<File>,
    pub directories: Vec<String>
}

#[derive(Debug)]
pub struct FileSystem {
    pub directories: HashMap<String, Directory>,
    pub cwd: String
}

impl Directory {
    pub fn new() -> Self {
        Self {
            files: vec![],
            directories: vec![]
        }
    }

    pub fn size(&self, fs: &FileSystem) -> u32 {
        let mut size = 0;
        for file in self.files.iter() {
            size += file.size;
        }
        for dir in self.directories.iter() {
            if let Some(dir) = fs.get_dir(dir) {
                size += dir.size(fs);
            }
        }
        size
    }
}

impl File {
    pub fn from(line: &str) -> Self {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() != 2 {
            panic!("file line did not have two parts");
        }
        File { name: String::from(words[1]), size: words[0].parse().unwrap() }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        let mut directories = HashMap::new();
        directories.insert(String::from("/"), Directory::new());
        FileSystem { 
            directories,
            cwd: String::new()
        }
    }
    pub fn new_dir_from_cli(&mut self, line: &str) {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() != 2 {
            panic!("file line did not have two parts");
        }
        let mut dir_path = self.cwd.clone();
        dir_path.push_str("/");
        dir_path.push_str(&words[1]);
        if let Some(cwd) = self.get_cwd_dir_mut() {
            cwd.directories.push(String::from(dir_path.clone()));
        }
        self.directories.insert(dir_path, Directory::new());
    }

    pub fn get_cwd_dir_mut(&mut self) -> Option<&mut Directory> {
        self.directories.get_mut(&self.cwd)
    }

    pub fn get_dir(&self, path: &str) -> Option<&Directory> {
        self.directories.get(path)
    }
    pub fn get_root_dir(&self) -> Option<&Directory> {
        self.get_dir("/")
    }
}

pub fn part1() {
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

    let mut sum_of_small_dirs = 0;
    for (dir_name, dir) in file_system.directories.iter() {
        let size = dir.size(&file_system);
        println!("{}: size: {}", dir_name, size);
        if MAX_SIZE > size {
            sum_of_small_dirs += size;
        }
    }
    println!("result: {}", sum_of_small_dirs);
    
}
