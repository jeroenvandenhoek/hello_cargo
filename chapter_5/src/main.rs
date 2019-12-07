#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("rect1 = {:?}", rect1);
}

fn area(rect: &Rectangle)->u16{
    rect.width * rect.height
}
