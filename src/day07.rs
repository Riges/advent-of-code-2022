use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let size = parts.next().unwrap().parse::<u64>()?;
        let name = parts.next().unwrap().to_string();
        Ok(Self::new(name, size))
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    path: String,
    files: Vec<File>,
    directories: Vec<String>,
}

impl Directory {
    fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn push(&mut self, file: File) {
        self.files.push(file);
    }

    fn push_dir(&mut self, dir: String) {
        self.directories.push(dir);
    }

    fn du(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }
}

#[derive(Debug)]
struct FileSystem {
    working_directory: String,
    directories: HashMap<String, Directory>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            working_directory: String::new(),
            directories: HashMap::new(),
        }
    }

    fn move_forward(&mut self, name: String) {
        self.working_directory = format!("{}/{}", self.working_directory, name);
    }

    fn move_back(&mut self) {
        let mut path = self.working_directory.split('/').collect::<Vec<&str>>();
        path.pop();
        self.working_directory = path.join("/");
    }

    fn move_to_root(&mut self) {
        self.working_directory = String::new();
    }

    fn pwd(&self) -> String {
        if self.working_directory.is_empty() {
            return "/".to_string();
        }
        self.working_directory.clone()
    }

    fn mkdir(&mut self) -> &mut Directory {
        let path = self.pwd();
        let name = self.pwd().split('/').last().unwrap().to_string();
        self.directories
            .entry(self.pwd())
            .or_insert(Directory::new(name, path))
    }

    fn create_file_to_current_directory(&mut self, file: File) {
        let dir = self.mkdir();
        dir.push(file);
    }

    fn create_directory_in_current_directory(&mut self, name: String) {
        let dir = self.mkdir();
        dir.push_dir(name);
    }
}

fn load_file_system(path: &str) -> anyhow::Result<FileSystem> {
    let mut fs = FileSystem::new();
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap()?;
        // if line.is_empty() {
        //     break;
        // }
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some("$"), Some("cd"), Some("/"), None) => fs.move_to_root(),
            (Some("$"), Some("cd"), Some(".."), None) => fs.move_back(),
            (Some("$"), Some("cd"), Some(name), None) => fs.move_forward(name.to_string()),
            (Some("$"), Some("ls"), None, None) => {}
            (Some("dir"), Some(name), None, None) => {
                fs.create_directory_in_current_directory(name.to_string())
            }
            (Some(size), Some(name), None, None) => fs.create_file_to_current_directory(File::new(
                name.to_string(),
                size.parse::<u64>()?,
            )),
            _ => {}
        }
    }

    Ok(fs)
}

fn compute_directory_size(fs: &FileSystem, dir: &Directory) -> u64 {
    let mut size = dir.du();

    if dir.directories.is_empty() {
        return size;
    }

    for directory_name in dir.directories.iter() {
        let mut sub_dir_path = format!("{}/{}", dir.path, directory_name).to_owned();
        if dir.name == "" {
            sub_dir_path = format!("/{}", directory_name).to_owned();
        } else {
        }
        match fs.directories.get(&sub_dir_path) {
            Some(dir) => size += compute_directory_size(fs, dir),
            None => {}
        }
    }

    size
}

pub fn day07() -> anyhow::Result<()> {
    let fs = load_file_system("data/day07.txt")?;

    println!(
        "Day 07 part 1: {:?}",
        fs.directories.iter().fold(0, |acc: u64, (_, dir)| {
            let size = compute_directory_size(&fs, dir);
            if size <= 100000 {
                return acc + size;
            }

            acc
        })
    );

    let disk_space = 70000000;
    let update_size = 30000000;
    let total_size = fs.directories.iter().map(|(_, dir)| dir.du()).sum::<u64>();
    let used_space = disk_space - total_size;
    let needed_space = update_size - used_space;

    println!(
        "total_size: {}, used_space: {}, neededspace {}",
        total_size, used_space, needed_space
    );
    println!(
        "Day 07 part 2 {:?}",
        fs.directories
            .iter()
            .fold(disk_space, |acc: u64, (_, dir)| {
                let size = compute_directory_size(&fs, dir);
                if size >= needed_space && size < acc {
                    return size;
                }

                acc
            })
    );

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn working_directory_should_be_expected() {
        let mut fs = FileSystem::new();
        fs.move_forward("a".to_string());
        assert_eq!(fs.pwd(), "/a");
    }

    #[test]
    fn working_directory_should_be_root() {
        let fs = FileSystem::new();
        assert_eq!(fs.pwd(), "/");
    }

    #[test]
    fn working_directory_should_be_root_with_move_back_from_root() {
        let mut fs = FileSystem::new();
        fs.move_back();
        assert_eq!(fs.pwd(), "/");
    }

    #[test]
    fn working_directory_should_be_root_with_move_to_root() {
        let mut fs = FileSystem::new();
        fs.move_forward("a".to_string());
        fs.move_to_root();
        assert_eq!(fs.pwd(), "/");
    }

    #[test]
    fn mkdir_should_create_directory() {
        let mut fs = FileSystem::new();
        let dir = fs.mkdir();
        assert_eq!(dir.name, "");
        assert_eq!(dir.path, "/");
        assert_eq!(fs.directories.len(), 1);
    }

    #[test]
    fn mkdir_should_not_create_directory_if_already_exist() {
        let mut fs = FileSystem::new();
        fs.mkdir();
        let dir = fs.mkdir();
        assert_eq!(dir.name, "");
        assert_eq!(dir.path, "/");
        assert_eq!(fs.directories.len(), 1);
    }

    #[test]
    fn should_parse_file() {
        assert_eq!(
            "14848514 b.txt".parse::<File>().unwrap(),
            File::new("b.txt".to_string(), 14848514)
        );
        assert_eq!(
            "8504156 c.dat".parse::<File>().unwrap(),
            File::new("c.dat".to_string(), 8504156)
        );
        assert_eq!(
            "29116 f".parse::<File>().unwrap(),
            File::new("f".to_string(), 29116)
        );
    }
}
