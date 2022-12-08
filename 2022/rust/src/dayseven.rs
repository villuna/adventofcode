// This code is fucking awful and i hope no future employer sees it
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

const SPACE_NEEDED: u32 = 30000000;
const TOTAL_SPACE: u32 = 70000000;

enum Command<'a> {
    Cd(&'a str),
    Ls,
    File(&'a str, u32),
    Directory(&'a str),
}

#[derive(Debug)]
struct Directory<'a> {
    parent: Option<Weak<RefCell<Directory<'a>>>>,
    children: HashMap<&'a str, FileNode<'a>>,
}

#[derive(Debug)]
enum FileNode<'a> {
    Directory(Rc<RefCell<Directory<'a>>>),
    File(u32),
}

#[derive(Debug)]
struct FileSystem<'a> {
    root: FileNode<'a>,
}

fn parse_line(line: &str) -> Command {
    if line.starts_with("$") {
        // Command
        if line == "$ ls" {
            Command::Ls
        } else {
            Command::Cd(line.split(" ").nth(2).unwrap())
        }
    } else {
        // Listing
        if line.starts_with("dir") {
            Command::Directory(line.split(" ").nth(1).unwrap())
        } else {
            let mut split = line.split(" ");
            let size = split.next().unwrap().parse::<u32>().unwrap();
            let name = split.next().unwrap();

            Command::File(name, size)
        }
    }
}

impl<'a> FileSystem<'a> {
    fn parse(input: &'a str) -> FileSystem<'a> {
        let root = Rc::new(RefCell::new(Directory {
            parent: None,
            children: HashMap::new(),
        }));

        let mut current = Rc::clone(&root);

        for line in input.lines() {
            // Skip the first line
            if line == "$ cd /" {
                continue;
            }

            match parse_line(line) {
                Command::Ls => {}

                Command::Cd("..") => {
                    let new = {
                        let parent = &current.borrow().parent;

                        Rc::clone(&match parent {
                            Some(reference) => reference.upgrade().unwrap(),

                            None => unreachable!(),
                        })
                    };

                    current = new;
                }

                Command::Cd(dir) => {
                    let parent = Rc::downgrade(&current);
                    current =
                        {
                            let mut borrowed_current = current.borrow_mut();

                            let next = borrowed_current.children.entry(dir).or_insert(
                                FileNode::Directory(Rc::new(RefCell::new(Directory {
                                    parent: Some(parent),
                                    children: HashMap::new(),
                                }))),
                            );

                            match &*next {
                                FileNode::Directory(reference) => Rc::clone(&reference),
                                FileNode::File(_) => unreachable!(),
                            }
                        };
                }

                Command::File(name, size) => {
                    current
                        .borrow_mut()
                        .children
                        .insert(name, FileNode::File(size));
                }

                Command::Directory(name) => {
                    current.borrow_mut().children.insert(
                        name,
                        FileNode::Directory(Rc::new(RefCell::new(Directory {
                            parent: Some(Rc::downgrade(&current)),
                            children: HashMap::new(),
                        }))),
                    );
                }
            }
        }

        return FileSystem {
            root: FileNode::Directory(root),
        };
    }
}

impl<'a> FileNode<'a> {
    fn directory_sizes(&self) -> Vec<u32> {
        let mut res = Vec::new();
        self.directory_sizes_helper(&mut res);
        res
    }

    // Calculates the size of this node. If it is a directory, pushes the size
    // to `vec`
    fn directory_sizes_helper(&self, vec: &mut Vec<u32>) -> u32 {
        match &*self {
            &FileNode::File(size) => size,

            FileNode::Directory(dir) => {
                let dir = dir.borrow();
                let mut total = 0;

                for child in dir.children.values() {
                    total += child.directory_sizes_helper(vec);
                }

                vec.push(total);
                total
            }
        }
    }
}

pub fn day_seven(input: String) {
    let fs = FileSystem::parse(&input);
    println!("{}", part_one(&fs));
    println!("{}", part_two(&fs))
}

fn part_one(fs: &FileSystem) -> u32 {
    fs.root
        .directory_sizes()
        .into_iter()
        .filter(|&size| size < 100000)
        .sum()
}

fn part_two<'a>(fs: &FileSystem<'a>) -> u32 {
    let mut sizes = fs.root.directory_sizes();
    sizes.sort();

    let used = sizes.last().unwrap();
    let free_space = TOTAL_SPACE - used;
    let target = SPACE_NEEDED - free_space;

    sizes.into_iter().find(|&size| size >= target).unwrap()
}
