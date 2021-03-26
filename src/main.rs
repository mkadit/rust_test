use text_io::read;

// Import modules
mod fizzbuzz;
mod guessing_game;
mod linked_list;
#[path = "Java_Rework/runner.rs"]
mod runner;

fn main() {
    // Loop endlessl until break
    loop {
        println!(
            "Choose what to do (Type the number):\n1. Guessing game\n2. Fizzbuzz\n3. Koleksi SDA"
        );

        let choice: String = read!();
        if choice.eq_ignore_ascii_case("1") {
            guessing_game::run();
        } else if choice.eq_ignore_ascii_case("2") {
            fizzbuzz::run();
        } else if choice.eq_ignore_ascii_case("3") {
            linked_list::run();
        } else if choice.eq_ignore_ascii_case("4") {
            runner::runner();
        } else {
            for i in (0..10).rev() {
                print!("{}", i);
            }

            break;
        }
    }
}
