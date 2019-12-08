enum Coin {
    Rijksdaalder,
    Gulden,
    Kwartje,
    Dubbeltje,
    Stuiver,
}
impl Coin {
    fn value_in_cents (coin: Coin) -> u8{
        match coin {
            Coin::Rijksdaalder => 250,
            Coin::Gulden => 100,
            Coin::Kwartje => 25,
            Coin::Dubbeltje => 10,
            Coin::Stuiver => 5,
        }
    }
    fn print_cents (coin: Coin){
        let num_cents = Coin::value_in_cents(coin);
        println!("number of cents in coin = {}", num_cents);
    
    }
}

fn main() {
    let Stuiver: Coin = Coin::Stuiver;
    Coin::print_cents(Stuiver)
}
