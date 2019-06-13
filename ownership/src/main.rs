fn takes_ownership(some_str: String) {
  println!("{}", some_str);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn get_length(s: &String) -> usize {
  s.len()
}

fn main() {
  //   let mut s = String::from("zhang");
  //   s.push_str(" zhen");
  //   s.push_str(" qiang");

  //   let y = s;

  //   //   println!("{}", x); // 报错，这个时候，s变量不再存在
  //   let z = y.clone();
  //   println!("s: {}, z: {}", y, z);

  //   let a = 8;
  //   let b = a;
  //   // 整型是实现了`Copy`的类型
  //   println!("a: {}, b: {}", a, b);

  let name = String::from("polunzh");
  takes_ownership(name);
  //   println!("name: {}", name); // 这里name不再有效
  let age = 32;
  makes_copy(age);
  println!("age: {}", age); // 这里age可以正常访问

  let hobby = String::from("badminton");
  let length = get_length(&hobby);
  println!("str length: {}", length);
}
