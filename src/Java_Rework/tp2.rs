use std::collections::HashMap;
use text_io::read;

pub fn run() {
    main_function();
}

pub fn main_function() {
    let mut stores: HashMap<String, Store> = HashMap::new();
    let mut list_road: Vec<Road> = Vec::new();
    let mut s = Store::new("Halo".to_string());
    let x = Road::new(10, 30, 6);
    s.exclusive_route
        .insert("World".to_string(), Road::new(10, 10, 0));
    println!("{:#?}\n{:#?}", s, x);
    println!("{:#?}", s.exclusive_route);
    println!("{},", s.exclusive_route.contains_key("World"));
    list_road.push(x);
    stores.insert(s.name.to_string(), s);
    println!("{:#?}", list_road);
    println!(
        "{:?}",
        stores.get("Halo").unwrap().exclusive_route.get("World")
    );
    println!(
        "{}",
        ask_connected(&"Halo".to_string(), &"World".to_string(), &mut stores)
    );
}

#[derive(Debug)]
struct Store {
    name: String,
    distance: i32,
    routes: HashMap<String, Road>,
    exclusive_route: HashMap<String, Road>,
    shortest_path: Vec<String>,
}
impl Store {
    pub fn new(name: String) -> Self {
        Self {
            name,
            distance: i32::MAX,
            routes: HashMap::new(),
            exclusive_route: HashMap::new(),
            shortest_path: Vec::new(),
        }
    }
}
fn ask_connected(from: &String, target: &String, stores: &HashMap<String, Store>) -> bool {
    stores
        .get(from)
        .unwrap()
        .exclusive_route
        .contains_key(target)
}

#[derive(Debug)]
struct Road {
    road: i32,
    coupon: i32,
    time: i32,
}
impl Road {
    pub fn new(coupon: i32, road: i32, time: i32) -> Self {
        Self { coupon, road, time }
    }
}

#[derive(Debug)]
struct Town {}

impl Town {}
