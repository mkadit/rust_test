#[derive(Debug)]
struct LinkedList {
    head: Link,
}
impl LinkedList {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}
pub fn run() {
    main_function();
}

fn main_function() {
    print!("Hello World");
}
