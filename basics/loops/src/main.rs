fn main() {
  let mut count = 0;

  loop {
    println!("count: {}", count);
    count += 1;
    if count == 5 {
      break;
    }
  }

  let mut num = 0;
  while num < 5 {
    println!("number: {}", num);
    num += 1;
  }

  for i in 0..5 {  //  from 0 to 4
    println!("i: {}", i);
  }

  for i in 0..=5 {      // from 0 to 5(inclusive)
    println!("i: {}", i);
  }

  let numbers = [10,20,30,40,50];
  for num in numbers.iter() {
    println!("number: {}", num);
  }

  let mut x = vec![1,2,3,4,5,6];
  while let Some(num) = x.pop() {
    println!("popped: {}", num);
  }

  'outer: for i in 1..=3 {
    for j in 1..=3 {
      if i == 2 && j == 2 {
        break 'outer;
      }
      println!("i: {}, j: {}", i, j);
    }
  }

  for i in 1..=5 {
    if i == 3 {
      continue; // skip 3
    }
    println!("i: {}", i);
  }

  let res = loop {
    let x = 10;
    break x * 2;
  };
  println!("res: {}", res);
}
