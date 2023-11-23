fn main() {
  let b = true;
  let mut a = 12;
  println!("{}", a);
  a = 13;

  println!("Hello, world!");
  println!("{}", b);
  println!("{}", a);
  println!("add(1, 2) = {}", add(1,2));
}

fn add(a:u32, b:u32) ->u32{
  return a+b;
}
