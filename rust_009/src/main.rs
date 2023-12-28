//ANCHOR - 枚举
#[derive(Debug)]
enum IpKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpKind {
    fn call(&self) {
        println!("self is {:?}", self);
    }
}

fn print_ip_kind(k: &IpKind) {
    println!("this ip kind is {:#?}", k);
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum ChState {
    HuBei,
    BeiJing,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(ChState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("coin penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(sheng) => {
            println!("chState is {:?}", sheng);
            25
        }
    }
}

fn main() {
    println!("Hello, rust009!");

    let v4 = IpKind::V4(127, 0, 0, 1);
    let v6 = IpKind::V6(String::from(":1"));

    print_ip_kind(&v4);
    print_ip_kind(&v6);
    v4.call();

    let mut t = Some(1);
    if t.is_some() {
        println!("t has value");
    }
    t = None;
    if t.is_none() {
        println!("t dont have value");
    }
    //NOTE - 使用match
    match t {
        Some(v) => {
            println!("t has value {v}");
        }
        None => println!("t dont have value"),
    }

    value_in_cents(Coin::Quarter(ChState::HuBei));
    value_in_cents(Coin::Nickel);

    let dice: u8 = 9;

    // let dice2 = match dice {
    //     0 => 3,
    //     1 => 4,
    //     _ => 10,
    // };

    let dice2 = match dice {
        0 => 3,
        1 => 4,
        other => 10 * other,
    };

    let tt = Some(2);
    if let Some(v) = tt {
        println!("tt = {v}");
    } else {
        println!("tt = None");
    }

    // NOTE match的other和_是有区别的，other会使用匹配的值，_不会。

    println!("dice2 = {dice2}");
}
