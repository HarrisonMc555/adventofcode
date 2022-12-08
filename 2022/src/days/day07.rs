use crate::days::{Day, Debug, Example, Part};

pub struct Day07;

impl Day for Day07 {
    fn number(&self) -> u32 {
        7
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day07 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let commands = parse_commands(self.read_file(example).trim());
        let filetree = assemble_filetree(&commands);
        filetree
            .directories()
            .map(Directory::calc_size)
            .filter(|size| *size <= MAX_SIZE)
            .sum()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let commands = parse_commands(self.read_file(example).trim());
        let filetree = assemble_filetree(&commands);
        let cur_used = filetree.calc_size();
        let cur_unused = TOTAL_SPACE - cur_used;
        let required_deleted_size = REQUIRED_UNUSED_SPACE - cur_unused;
        filetree
            .directories()
            .map(Directory::calc_size)
            .filter(|size| *size >= required_deleted_size)
            .min()
            .unwrap()
    }
}

const MAX_SIZE: usize = 100000;
const TOTAL_SPACE: usize = 70000000;
const REQUIRED_UNUSED_SPACE: usize = 30000000;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Item {
    Directory(Directory),
    File(File),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct File {
    size: usize,
    name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Command {
    Cd(String),
    Ls(Vec<Item>),
}

fn parse_commands(text: &str) -> Vec<Command> {
    let lines = text.split("\n");
    let mut commands = Vec::new();
    let mut cur_command = None;
    for line in lines {
        match line.strip_prefix("$ ") {
            Some(command) => {
                commands.extend(cur_command);
                cur_command = Some(Command::parse(command));
            }
            None => match cur_command {
                None | Some(Command::Cd(_)) => panic!(),
                Some(Command::Ls(ref mut items)) => items.push(Item::parse(line)),
            },
        }
    }
    commands.extend(cur_command);
    commands
}

fn assemble_filetree(commands: &[Command]) -> Directory {
    if !matches!(&commands[0], Command::Cd(dir) if dir == "/") {
        panic!();
    };
    let root = Directory::new("/".to_string());
    helper(root, &mut commands[1..].iter())
}

fn helper<'a, T>(mut cur_directory: Directory, commands: &mut T) -> Directory
where
    T: Iterator<Item = &'a Command>,
{
    while let Some(command) = commands.next() {
        match command {
            Command::Cd(dir) if dir == ".." => return cur_directory,
            Command::Cd(dir) => {
                let subdirectory = helper(Directory::new(dir.to_string()), commands);
                cur_directory.subdirectories.push(subdirectory);
            }
            Command::Ls(files) => {
                for file in files {
                    match file {
                        Item::File(file) => cur_directory.files.push(file.clone()),
                        _ => {}
                    }
                }
            }
        }
    }
    cur_directory
}

impl Command {
    fn parse(line: &str) -> Self {
        if line == "ls" {
            return Command::Ls(Vec::new());
        }
        let cd_dest = line.strip_prefix("cd ").unwrap();
        Command::Cd(cd_dest.to_string())
    }
}

impl Item {
    fn parse(line: &str) -> Self {
        if let Some(name) = line.strip_prefix("dir ") {
            return Item::Directory(Directory::new(name.to_string()));
        }
        let parts = line.split(" ").collect::<Vec<_>>();
        let (size, name) = match parts.as_slice() {
            [size, name] => (size, name),
            _ => panic!(),
        };
        let size = size.parse().ok().unwrap();
        Item::File(File::new(size, name.to_string()))
    }
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: Vec::new(),
        }
    }

    fn directories(&self) -> DirectoriesIter {
        DirectoriesIter::new(self)
    }

    fn files(&self) -> FilesIter {
        FilesIter::new(self)
    }

    fn calc_size(&self) -> usize {
        self.files().map(|f| f.size).sum()
    }
}

impl File {
    fn new(size: usize, name: String) -> Self {
        File { name, size }
    }
}

struct DirectoriesIter<'a> {
    directory: &'a Directory,
    listed_self: bool,
    subdirectory_file_iter: Box<Option<DirectoriesIter<'a>>>,
    subdirectories_iter: std::slice::Iter<'a, Directory>,
}

struct FilesIter<'a> {
    directory: &'a Directory,
    files_iter: std::slice::Iter<'a, File>,
    subdirectory_file_iter: Box<Option<FilesIter<'a>>>,
    subdirectories_iter: std::slice::Iter<'a, Directory>,
}

impl<'a> DirectoriesIter<'a> {
    fn new(directory: &'a Directory) -> Self {
        DirectoriesIter {
            directory,
            listed_self: false,
            subdirectory_file_iter: Box::new(None),
            subdirectories_iter: directory.subdirectories.iter(),
        }
    }
}

impl<'a> FilesIter<'a> {
    fn new(directory: &'a Directory) -> Self {
        FilesIter {
            directory,
            files_iter: directory.files.iter(),
            subdirectory_file_iter: Box::new(None),
            subdirectories_iter: directory.subdirectories.iter(),
        }
    }
}

impl<'a> Iterator for DirectoriesIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.listed_self {
            self.listed_self = true;
            return Some(self.directory);
        }
        loop {
            if let Some(ref mut iter) = &mut *self.subdirectory_file_iter {
                if let directory @ Some(_) = iter.next() {
                    return directory;
                }
            }
            self.subdirectory_file_iter =
                Box::new(Some(DirectoriesIter::new(self.subdirectories_iter.next()?)));
        }
    }
}

impl<'a> Iterator for FilesIter<'a> {
    type Item = &'a File;

    fn next(&mut self) -> Option<Self::Item> {
        if let file @ Some(_) = self.files_iter.next() {
            return file;
        }
        loop {
            if let Some(ref mut iter) = &mut *self.subdirectory_file_iter {
                if let file @ Some(_) = iter.next() {
                    return file;
                }
            }
            self.subdirectory_file_iter =
                Box::new(Some(FilesIter::new(self.subdirectories_iter.next()?)));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = include_str!("../../static/example07.txt");
        let expected = vec![
            Command::Cd("/".to_string()),
            Command::Ls(vec![
                Item::Directory(Directory::new("a".to_string())),
                Item::File(File::new(14848514, "b.txt".to_string())),
                Item::File(File::new(8504156, "c.dat".to_string())),
                Item::Directory(Directory::new("d".to_string())),
            ]),
            Command::Cd("a".to_string()),
        ];
        let commands = parse_commands(text);
        assert_eq!(expected, commands[..expected.len()]);

        let filetree = assemble_filetree(&commands);
        assert_eq!(filetree.name, "/");
        assert_eq!(
            filetree.files,
            vec![
                File::new(14848514, "b.txt".to_string()),
                File::new(8504156, "c.dat".to_string())
            ]
        );
        assert_eq!(filetree.subdirectories.len(), 2);
        let a = &filetree.subdirectories[0];
        assert_eq!(
            a.files,
            vec![
                File::new(29116, "f".to_string()),
                File::new(2557, "g".to_string()),
                File::new(62596, "h.lst".to_string()),
            ]
        );
    }

    #[test]
    fn test_directories_iter() {
        let text = include_str!("../../static/example07.txt");
        let commands = parse_commands(text);
        let filetree = assemble_filetree(&commands);
        let mut iter = filetree.directories();
        assert_eq!(iter.next().unwrap().name, "/");
        assert_eq!(iter.next().unwrap().name, "a");
        assert_eq!(iter.next().unwrap().name, "e");
        assert_eq!(iter.next().unwrap().name, "d");
    }

    #[test]
    fn test_files_iter() {
        let text = include_str!("../../static/example07.txt");
        let commands = parse_commands(text);
        let filetree = assemble_filetree(&commands);
        let mut iter = filetree.files();
        assert_eq!(
            *iter.next().unwrap(),
            File::new(14848514, "b.txt".to_string())
        );
        assert_eq!(
            *iter.next().unwrap(),
            File::new(8504156, "c.dat".to_string())
        );
        assert_eq!(*iter.next().unwrap(), File::new(29116, "f".to_string()));
        assert_eq!(*iter.next().unwrap(), File::new(2557, "g".to_string()));
        assert_eq!(*iter.next().unwrap(), File::new(62596, "h.lst".to_string()));
        assert_eq!(*iter.next().unwrap(), File::new(584, "i".to_string()));
        assert_eq!(*iter.next().unwrap(), File::new(4060174, "j".to_string()));
        assert_eq!(
            *iter.next().unwrap(),
            File::new(8033020, "d.log".to_string())
        );
        assert_eq!(
            *iter.next().unwrap(),
            File::new(5626152, "d.ext".to_string())
        );
        assert_eq!(*iter.next().unwrap(), File::new(7214296, "k".to_string()));
    }

    #[test]
    fn test_examples_part1() {}

    #[test]
    fn test_examples_part2() {}
}
