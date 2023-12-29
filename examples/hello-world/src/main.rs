use daywatch_newtypes::{
    self,
    money::{iso, Money},
};

struct User<'a> {
    money: Money<'a, iso::Currency>,
}

fn main() {
    let user = User {
        money: Money::from_major(1000, iso::KRW),
    };

    println!("Hello, world!");
}
