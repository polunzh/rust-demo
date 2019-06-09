fn main() {
  let tup: (i32, f32, i8) = (32, 12.3, 8);
  let (foo, bar, zoo) = tup;

  println!("foo: {}, bar: {}, zoo: {}", foo, bar, zoo);
  println!("foo: {}, bar: {}, zoo: {}", tup.0, tup.1, tup.2);

  let arr: [i32; 3] = [1, 2, 3];
  //   println!("{}", arr[3]); // 报错
  println!("{:?}", arr);
}
