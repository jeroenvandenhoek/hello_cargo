
fn main() {
    let s: String = String::from("Hello Earth");
    print_a_string(s.clone()); // shallow copy
    print_a_string(s); // move

    // ownership move and copy excersise
    move_string_ownership();
    copy_string_ownership();
    
    // return a tuple from a function
    let a: String = String::from("Hello Mars"); // create a string
    let (b, c) = return_a_tuple(a.clone()); // print a string and another string. then return a tuple
    println!("{}\n{}", b, c); // print values from tuple
    println!("{}", a); // print a (only possible because a shallow copy was made when passing it to the return_a_tuple function)

}

fn print_a_string(some_string: String) {
    println!("as printed from print_a_string function: {}",some_string);
}

fn move_string_ownership() {
    let s = String::from("hello");
    let x = s;

    println!("{}", x);
}

fn copy_string_ownership() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    
    s1 = String::from("aloha");
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn return_a_tuple(a: String) -> (String, String) {
    let string_two = String::from("Hello Venus");
    (a, string_two)
}
