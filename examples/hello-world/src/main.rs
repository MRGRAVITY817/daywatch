use daywatch_newtypes::{
    self,
    money::{iso, Money},
    user_info::Email,
};
use std::error::Error;

struct User<'a> {
    money: Money<'a, iso::Currency>,
    email: Email,
}

fn main() -> Result<(), Box<dyn Error>> {
    let user = User {
        money: Money::from_major(1000, iso::KRW),
        email: Email::new("hoonwee817@email.com")?,
    };

    println!("Hello, world!");

    Ok(())
}
