use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    size: Cell<usize>,
    name: String,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            size: Cell::new(0),
            name: name.to_owned(),
        }
    }
}

pub fn build_directories(input: &str) -> Vec<Rc<Directory>> {
    let mut directories: Vec<Rc<Directory>> = vec![];
    let mut directory_stack: Vec<Rc<Directory>> = vec![];

    for line in input.trim().split('\n') {
        match line.as_bytes()[0] {
            b'$' => {
                if line.starts_with("$ cd") {
                    let dir_name = line.split(' ').last().unwrap();
                    if dir_name == ".." {
                        directory_stack.pop();
                        continue;
                    }

                    let directory = Rc::new(Directory::new(dir_name));
                    directories.push(Rc::clone(&directory));
                    directory_stack.push(directory);
                }
            }
            b'd' => continue,
            _ => {
                let size: usize = line
                    .split(' ')
                    .next()
                    .expect("invalid file listing")
                    .parse()
                    .expect("invalid file size");

                for directory in &directory_stack {
                    directory.size.set(directory.size.get() + size);
                }
            }
        }
    }

    directories
}

// Computes the sum of the sizes of the directories that contain at least 100,000 bytes.
pub fn part_1(directories: &[Rc<Directory>]) -> usize {
    directories.iter().fold(0, |sum, d| match d.size.get() {
        size if size <= 100_000 => sum + size,
        _ => sum,
    })
}

// Computes the size of the directory that if deleted would free up the needed space.
pub fn part_2(directories: &[Rc<Directory>]) -> usize {
    let needed = 30_000_000 - (70_000_000 - directories[0].size.get());
    directories
        .iter()
        .fold(usize::MAX, |min, d| match d.size.get() {
            size if size >= needed && size < min => size,
            _ => min,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    macro_rules! directory {
        ($name:expr, $size:expr) => {
            Rc::new(Directory {
                name: $name.to_owned(),
                size: Cell::new($size),
            })
        };
    }

    #[test]
    fn test_build_directories() {
        let input = fs::read_to_string("test-input.txt").expect("unable to read test input");
        let directories = build_directories(&input);

        let expected_directories = vec![
            directory!["/", 48381165],
            directory!["a", 94853],
            directory!["e", 584],
            directory!["d", 24933642],
        ];

        assert_eq!(directories, expected_directories);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").expect("unable to read test input");
        let directories = build_directories(&input);
        assert_eq!(part_1(&directories), 95437);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").expect("unable to read test input");
        let directories = build_directories(&input);
        assert_eq!(part_2(&directories), 24933642);
    }
}
