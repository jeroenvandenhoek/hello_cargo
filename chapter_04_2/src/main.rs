fn main() {
    let s1 = String::from("Hello Jupiter");
    let length_of_string = return_length_of_string(&s1);

    println!("{}", length_of_string)

}

fn return_length_of_string(s: &String) -> usize {
    println!("{}", s.len());
    s.len()
}
