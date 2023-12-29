use daywatch_newtypes::{
    money::{iso, Money},
    user_info::{Email, PhoneNumber},
    uuid::Uuid,
};
use std::error::Error;

#[derive(Debug)]
struct User<'a> {
    user_id: Uuid,
    money: Money<'a, iso::Currency>,
    email: Email,
    phone_number: PhoneNumber,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _user = User {
        user_id: Uuid::new_v4(),
        money: Money::from_major(1000, iso::KRW),
        email: Email::new("hoonwee817@email.com")?,
        phone_number: PhoneNumber::new("010-9121-6750")?,
    };

    Ok(())
}
