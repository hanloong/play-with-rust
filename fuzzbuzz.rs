fn main() {
  for num in range(1i, 100) {
    println!("{:d}", num);
  }
}

fn div_by_three(num: int) -> bool {
  num % 3 == 0
}

fn div_by_five(num: int) -> bool {
  num % 5 == 0
}

#[test]
fn test_number() {
  if get_word(1) != "1" {
    fail!("Should return 1");
  }
}

#[test]
fn test_div_by_three() {
  if div_by_three(1) {
    fail!("One is not divisble by three");
  }
}

#[test]
fn test_div_by_three_with_one() {
  if !div_by_three(3) {
    fail!("Three is not divisble by three");
  }
}

#[test]
fn test_div_by_five() {
  if div_by_five(1) {
    fail!("One is not divisible by five")
  }
}

#[test]
fn test_div_by_five_with_five() {
  if !div_by_five(5) {
    fail!("Five is divisible by five")
  }
}
