fn main() {
  let name = String::from("zhangzhenqiang");
  let first_name = &name[0..=4];
  let last_name = &name[5..];
  println!("first name: {}, last name: {}", first_name, last_name);
}
