fn main() {
    // create a mutable string
    let mut s1 = String::from("Hello Jupiter"); 
    
    // plug the mutable string in this function as a mutable reference argument
    // this function will add text to the mutable reference argument
    // this function will also return the length of the new string
    let length_of_string = return_length_of_string(&mut s1); 

    // print the new string
    println!("{}", length_of_string); // this wil print the length of the string

    // create a mutable reference to s1 (this is not actually usefull in this case)
    let s3 = &mut s1;

    // add tekst to the string through the reference variable 
    s3.push_str(" and Saturn");

    // print the new line
    println!("{}", s3); // this will print: "Hello Jupiter and Venus and Saturn"

}

fn return_length_of_string(s: &mut String) -> usize {
    // add tekst to the mutable reference argument
    s.push_str(" and Venus");

    // print the line
    println!("{}",s); // this will print: "Hello Jupiter and Venus"

    // return the length of the string
    s.len()
}
