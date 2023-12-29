# daywatch_newtypes

Tired of implementing commonly used value objects (or newtypes, in rust) in every projects?

Do not worry - `daywatch_newtypes` has done the hard work for you.

## List of newtypes

- Money: ISO currency, Cryptocurrency (re-exported from `rusty-money`)
- User Info: Email, Username, PhoneNumber
- Common: UUID (re-exported from `uuid`),
- Time: DateTime, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc (re-exported from `chrono`)

## Example: Creating User

```rust
use daywatch_newtypes::prelude::*;
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
```
