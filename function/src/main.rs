fn say_something(something: &str) -> () {
  // 不返回值
  println!("{}!", something);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}

fn main() {
  // let x = (let y = 6); // 报错
  let x = {
    let y = 10;
    y + 5; // 语句
    y + 5 // 表达式
  };

  println!("x: {}", x);
  say_something("done");
  println!("10 + 25 = {}", add(10, 25));
}
