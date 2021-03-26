use text_io::read;
pub fn run() {
    main_function();
}

fn main_function() {
    // Creating the inputs
    let l: usize = read!();
    let n: i32 = read!();
    let q: i32 = read!();
    let mut i = 0;

    // Create array for storing the differences and for storing flags
    let mut temp = vec![0; l + 2];
    let mut first_i = vec![0; l];
    let mut last_i = vec![0; l];

    // Create loop to determine the differences between a stack to the other
    while i < n {
        let a: usize = read!();
        let b: usize = read!();
        let k: i32 = read!();
        update_arr(temp.as_mut_slice(), a - 1, b - 1, k);
        i += 1;
    }

    // Preparing for flags generator
    first_i[0] = 0;
    last_i[l - 1] = l as i32 - 1;
    let mut first: i32 = 0;
    let mut last: i32 = l as i32 - 1;

    // Creating the answers for both beginning and ending flags
    for i in 1..l as usize {
        if temp[i] >= 0 {
            first_i[i] = first;
        } else {
            first = i as i32;
            first_i[i] = i as i32;
        }
    }
    for i in (0..=l - 2).rev() {
        if -temp[i + 1] >= 0 {
            last_i[i] = last;
        } else {
            last = i as i32;
            last_i[i] = i as i32;
        }
    }

    i = 0;
    while i < q {
        let first: usize = read!();
        let last: usize = first;
        println!("{} {}", first_i[first - 1] + 1, last_i[last - 1] + 1);
        i += 1;
    }
}

fn update_arr(arr: &mut [i32], a: usize, b: usize, k: i32) -> () {
    arr[a] += k;
    arr[b + 1] -= k;
}
