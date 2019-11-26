// instead of this program being fahrenheit to celsius, it's trying different things

fn main() {
    if_statement();
    println!("{}", for_loop_test());
}

fn if_statement() {
    let x: u8 = 4;

    // if statement with 3 arms
    if x == 5 {
        println!("x = {}",x);
    } else if x == 4 {
        println!("x = {}",x);
    } else {
        println!("x = {}",x);
    }
}

fn for_loop_test() -> u8 {
    let temp_array: [u8;5] = [4;5];
    let mut x: u8 = 0;
    for i in temp_array.iter() {
        x = x + i;
    }
    
    // return x
    x
}
