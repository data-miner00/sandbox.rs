use common::add;
use time::PrimitiveDateTime as DateTime;
use time::Duration;
use regex::Regex;
use time::macros::datetime;

fn main() {
    println!("Hello, from console!");
    println!("Total: {}", add(6, 8));

    let today = datetime!(2019-01-01 0:00);
    let after = after(today);

    println!("{}", after);
}

fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
