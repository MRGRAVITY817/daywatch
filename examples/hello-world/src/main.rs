use daywatch_newtypes::{
    money::{iso, Money},
    time::NaiveDate,
    user_info::{Email, PhoneNumber},
    uuid::Uuid,
};
use std::error::Error;

struct User<'a> {
    user_id: Uuid,
    money: Money<'a, iso::Currency>,
    email: Email,
    phone_number: PhoneNumber,
    birthdate: NaiveDate,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _user = User {
        user_id: Uuid::new_v4(),
        money: Money::from_major(1000, iso::KRW),
        email: Email::new("hoonwee817@email.com")?,
        phone_number: PhoneNumber::new("010-9121-6750")?,
        birthdate: NaiveDate::from_ymd_opt(1994, 8, 17).unwrap(),
    };

    Ok(())
}
