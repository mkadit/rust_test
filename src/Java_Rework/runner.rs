use text_io::read;

mod lab1;
mod lab2;
mod tp2;

pub fn runner() {
    println!("Choose what to do (Type the number):\n1. Lab1: Waterfall");
    let choice: String = read!();
    if choice.eq_ignore_ascii_case("1") {
        lab1::run();
    } else if choice.eq_ignore_ascii_case("2") {
        lab2::run();
    } else if choice.eq_ignore_ascii_case("3") {
        tp2::run();
    }
}
