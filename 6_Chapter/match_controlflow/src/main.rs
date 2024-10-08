#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Hawaii);
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        Some(i) => println!("The value is: {}", i),
        None => println!("The value is: None"),
    }

    let dice_roll = 5;
    match dice_roll {
        6 => get_started(),
        1 => one_more_chance(),
        1..=6 => println!("Good luck"),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn one_more_chance() {
    println!("Roll the dice again");
}
fn get_started() {
    println!("Piece go to start position");
}
