use std::{cell::RefCell, rc::Rc};

use num_format::ToFormattedString;

type Wrapper<T> = Rc<RefCell<T>>;

fn wrap_with_wrapper<T>(data: T) -> Wrapper<T> {
    Rc::new(RefCell::new(data))
}

#[derive(Default)]
struct Directory<'a> {
    name: &'a str,
    parent: Option<Wrapper<Self>>,
    size: usize,
    files: Vec<(usize, &'a str)>,
    children: Vec<Wrapper<Self>>,
}

impl<'a> Directory<'a> {
    fn get_size(&mut self) -> usize {
        let mut size = self.size;
        for child in &self.children {
            let child_size = child.borrow_mut().get_size();
            size += child_size;
        }
        self.size = size;
        size
    }

    fn get_solution2(&self, needed_space: usize, mut smallest: usize) -> usize {
        if self.size >= needed_space && self.size < smallest {
            smallest = self.size;
        }

        for children in &self.children {
            let child_small = children.borrow().get_solution2(needed_space, smallest);
            if child_small < smallest {
                smallest = child_small;
            }
        }

        smallest
    }

    fn get_solution1(&self, mut acc: usize) -> usize {
        let initial_acc = acc;
        if self.size <= 100_000 {
            acc += self.size;
        }

        for child in &self.children {
            acc += child.borrow().get_solution1(acc);
        }

        acc - initial_acc
    }

    fn draw(&self, level: u8) {
        println!(
            "{}({})",
            self.name,
            self.size.to_formatted_string(&num_format::Locale::en)
        );
        for file in &self.files {
            for _ in 0..level + 1 {
                print!("\t");
            }
            println!(
                "{}({})",
                file.1,
                file.0.to_formatted_string(&num_format::Locale::en)
            );
        }
        for child in &self.children {
            for _ in 0..level + 1 {
                print!("\t");
            }
            child.borrow().draw(level + 1);
        }
    }

    fn build_tree(input: &'a str) -> Wrapper<Self> {
        let mut current_directory = wrap_with_wrapper(Directory::default());
        let mut root_node: Wrapper<Directory> = current_directory.clone();
        for line in input.split_terminator("\n") {
            let mut iter = line.split_terminator(" ");
            match iter.next().unwrap() {
                "$" => match iter.next().unwrap() {
                    "cd" => match iter.next().unwrap() {
                        ".." => {
                            let directory = current_directory.clone();
                            let borrow = directory.borrow();
                            if let Some(parent) = borrow.parent.clone() {
                                current_directory = parent;
                            }
                        }
                        directory => {
                            let mut new_directory = Directory::default();
                            if !current_directory.borrow().name.is_empty() {
                                new_directory.parent = Some(current_directory.clone());
                            }
                            new_directory.name = directory;
                            let wrapped_directory = wrap_with_wrapper(new_directory);
                            if current_directory.borrow().name.is_empty() {
                                root_node = wrapped_directory.clone();
                            }
                            let children_clone = current_directory.clone();
                            children_clone
                                .borrow_mut()
                                .children
                                .push(wrapped_directory.clone());
                            current_directory = wrapped_directory;
                        }
                    },
                    _ => {}
                },
                "dir" => {}
                size => {
                    let size = size.parse::<usize>().unwrap();
                    current_directory.borrow_mut().size += size;
                    current_directory
                        .borrow_mut()
                        .files
                        .push((size, iter.next().unwrap()))
                }
            }
        }
        root_node
    }
}

pub fn another_solution(input: &str) {
    let root_node = Directory::build_tree(&input);
    let root_size = root_node.borrow_mut().get_size();
    println!("Solution 1 is {}", root_node.borrow().get_solution1(0));
    println!(
        "Solution 2 is {}",
        root_node
            .borrow()
            .get_solution2(30_000_000 - (70_000_000 - root_size), usize::MAX)
    );
}
