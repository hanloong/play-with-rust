fn main() {
  for num in range(1i, 101) {
    println!("{:s}",
      if div_by_fifteen(num) { "fuzzbuzz".to_str() }
      else if div_by_five(num) { "buzz".to_str() }
      else if div_by_three(num) { "fuzz".to_str() }
      else { num.to_str() }
    );
  }
}

fn div_by_three(num: int) -> bool {
  div_by(3, num)
}

fn div_by_five(num: int) -> bool {
  div_by(5, num)
}

fn div_by_fifteen(num: int) -> bool {
  div_by(15, num)
}

fn div_by(by: int, num: int) -> bool {
  num % by == 0
}

#[test]
fn test_div_by_fifteen() {
  assert!(!div_by_fifteen(1))
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
  assert!(div_by_fifteen(15))
}
#[test]
fn test_div_by_three() {
  assert!(!div_by_three(1))
}

#[test]
fn test_div_by_three_with_three() {
  assert!(div_by_three(3))
}

#[test]
fn test_div_by_five() {
  assert!(!div_by_five(1))
}

#[test]
fn test_div_by_five_with_five() {
  assert!(div_by_five(5))
}
