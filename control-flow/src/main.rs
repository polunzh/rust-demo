fn main() {
  let num = 10;
  if num > 10 {
    println!("big");
  } else if num < 10 {
    println!("small");
  } else if num == 10 {
    println!("equal");
  }

  let foo = if 3 > 2 { 3 } else { 2 };
  // let bar = if 3 > 2 { 3 } else { 'c' }; // 报错，必须是相同类型的表达式
  println!("foo: {}", foo);

  let mut x = 0;
  loop {
    if x == 10 {
      break;
    }

    x += 1;
  }

  println!("x: {}", x);

  let mut y = 0;
  while y != 10 {
    y += 1;
  }

  println!("y: {}", y);

  let arr: [i8; 5] = [1, 8, 10, 32, 10];
  for ele in arr.iter() {
    println!("ele: {}", ele);
  }

  // 很"帅气"的倒计时
  for ele in (1..4).rev() {
    println!("rev ele: {}", ele);
  }
}
