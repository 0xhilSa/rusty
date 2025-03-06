fn sqr(a:i32) -> i32 {
  a * a
}

fn swap(a:i32, b:i32) -> (i32, i32) {
  (b, a)
}

fn double(n:&i32) -> i32 {
  *n * 2
}

fn incr(n: &mut i32) {
  *n += 1; 
}

fn scale(f: fn(i32) -> i32, x:i32) -> i32 {
  f(f(x))
}

fn add(n:i32) -> i32 {
  n + 1
}

fn main() {
  let mut value = 1;

  let res = sqr(2);
  let (x, y) = swap(3,7);
  let a = double(&res);
  
  let multiply = |a:i32, b:i32| a * b;  // anonymous function

  println!("before incr: {}", value);
  incr(&mut value);
  println!("{}", res);
  println!("{} {}", x, y);
  println!("{}", a);
  println!("after incr: {}", value);
  println!("2 * 3: {}", multiply(2,3));
  println!("{}", scale(add, 3));
}
