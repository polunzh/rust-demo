enum IpAddrKind1 {
  V4,
  V6,
}

enum IpAddrKind2 {
  V4(u8, u8, u8, u8),
  V6(String),
}

impl IpAddrKind2 {
  fn call(&self) {
    println!("this is the method of a enum");
  }
}

fn main() {
  let home = IpAddrKind2::V4(192, 168, 1, 1);
  home.call();
  let age = Some(8);

  println!("{:?}", age);
}
