use std::mem;
use text_io::read;

pub fn run() {
    main_function();
}

fn main_function() {
    let x = String::from("Hello");
    let mut y = string_to_node(x);
    println!("{:#?}", y.unwrap().data);
}
#[derive(Debug)]
struct Node {
    data: char,
    next: Option<Box<Node>>,
}

enum Link {
    None,
    More(Box<Node>),
}

impl Node {
    fn new(data: char) -> Node {
        Node { data, next: None }
    }
    fn remove_curr(&mut self) {}
}

fn string_to_node(data: String) -> Option<Box<Node>> {
    let mut cur = None;
    for i in data.chars().rev() {
        let mut new_node = Node::new(i);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }
    cur
}
