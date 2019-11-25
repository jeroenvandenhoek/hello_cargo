
fn main() {
    // testing Arrays
    let test_array: [u8; 5] = [3;5];
    let test_array = [0, test_array[4], 8, 9];

    println!("{}",test_array[1]);

    println!("and printing the number {}",five());   

    looper();

}

fn five() -> u8 {
    5
}

fn looper() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 6
        }
    };

    println!("The result is {}", result);
}
