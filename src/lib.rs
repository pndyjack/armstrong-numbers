pub fn is_armstrong_number(num: u32) -> bool {
  let mut mut_num = num;
  let mut digits: Vec<u32> = Vec::new();
  while mut_num != mut_num % 10 {
    digits.push(mut_num % 10);
    mut_num = mut_num / 10;
  }
  digits.push(mut_num);
  let mut sum = 0;
  for &digit in digits.iter() {
    sum += u32::pow(digit, digits.len() as u32);
  }
  if sum == num {
    return true;
  }
  false
}
