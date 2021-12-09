mod bank;
mod currency;
use bank::Bank;
use currency::Currency;

fn main() {
    currency!(USD, 1.0, "$");
    currency!(INR, 0.013, "₹");
    let mut bank = Bank::new();

    account!(bank => + "John Doe", USD(100.0), 1010);
    account!(bank => + "Jane Doe", USD(200.0), 2020);
    transaction!(bank => > "Jane Doe", "John Doe", USD(50.0), 2020);
    transaction!(bank => + "John Doe", INR(50.0));
    transaction!(bank => - "John Doe", USD(50.0), 1010);
    println!("{}", account!(bank => ? "Jane Doe", 2020).balance());
    println!("{}", account!(bank => ? "John Doe", 1010).balance());
    account!(bank => - "Jane Doe", 2020);
}
