#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn area(&self)->u16{
        self.width * self.height
    }

    fn hello_world(){
        println!("Hello world")
    
    }
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("rect1 = {:#?}", rect1);

    Rectangle::hello_world()
}

