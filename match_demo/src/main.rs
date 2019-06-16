#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dim,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("Luck penny");
      1
    }
    Coin::Nickel => 5,
    Coin::Dim => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
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

fn main() {
  let coin = value_in_cents(Coin::Quarter(UsState::Alabama));
  println!("coin of quarter: {}", coin);

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("six: {:?}", six);
  println!("none: {:?}", none);

  let value = 10;
  match value {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => (),
  };

  let coin = Coin::Quarter(UsState::Alaska);
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}", state);
  } else {
    count += 1;
  }
}
