fn main() {
    let s1 = String::from("Hello Jupiter");
    let length_of_string = return_length_of_string(&mut s1);

    println!("{}", length_of_string)

}

fn return_length_of_string(s: &mut String) -> usize {
    println!("{}", s.len());
    s.len()
}
