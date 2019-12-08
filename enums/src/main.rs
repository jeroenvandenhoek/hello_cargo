#[derive(Debug)]
enum Message {
    Write(String),
    Number(u8),
}
impl Message {
    fn print_message(&self){
        println!("the value of this variable is {:?}", &self);

    }
}
fn main() {
    // let message1 = Message::Write(String::from("hello"));
    let message1 = Message::Write(String::from("hello"));
    let message2 = Message::Number(8);
    message1.print_message();
    message2.print_message();
}
