#[derive(Debug)]
enum Landen {
    Nederland,
    Belgie,
}
#[derive(Debug)]
enum Coin {
    Rijksdaalder,
    Gulden,
    Kwartje,
    Dubbeltje(Landen),
    Stuiver,
}
impl Coin {
    fn value_in_cents (coin: Coin) -> u8{
        match coin {
            Coin::Rijksdaalder => 250,
            Coin::Gulden => 100,
            Coin::Kwartje => 25,
            Coin::Dubbeltje(Landen::Nederland) => {
                println!("{:?}",Landen::Nederland);
                8
            },
            Coin::Dubbeltje(Landen::Belgie) => {
                println!("{:?}",Landen::Belgie);
                8
            },
            Coin::Stuiver => 5,
        }
    }
    fn print_cents (coin: Coin){
       let num_cents = Coin::value_in_cents(coin);
        println!("number of cents in coin = {}", num_cents);
    
    }
}

fn main() {
    let stuiver: Coin = Coin::Stuiver;
    println!("{:#?}",stuiver);
    Coin::print_cents(stuiver);
    Coin::print_cents(Coin::Dubbeltje(Landen::Nederland));
    plus_one(Some(4));
}

fn plus_one(x: Option<i8>) {
    match x {
        Some(x) => println!("{}", x+1),
        None => println!("no num"),
    }

}
