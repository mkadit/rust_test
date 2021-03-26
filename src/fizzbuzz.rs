pub fn run() {
    // Declare start
    let mut n = 1;

    while n < 101 {
        if n % 3 == 0 {
            print!("Fizz");
        }
        if n % 5 == 0 {
            print!("Buzz");
        }
        if n % 3 != 0 && n % 5 != 0 {
            print!("{}", n);
        }
        println!();
        n += 1;
    }
}
