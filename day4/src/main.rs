fn main() {
  /*
  However, they do remember a few key facts about the password:

    It is a six-digit number.
    The value is within the range given in your puzzle input.
    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

  How many different passwords within the range given in your puzzle input meet these criteria?
  */
  let input = include_str!("input.txt");
  let parts: Vec<Password> = input.split("-").filter_map(|x| x.parse().ok()).collect();

  let num_passwords = (parts[0]..=parts[1]).filter(is_viable_pw).count();
  println!("Part 1: {}", num_passwords);

  /*
  An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
  */
  let num_passwords2 = (parts[0]..=parts[1]).filter(is_viable_pw2).count();
  println!("Part 2: {}", num_passwords2);
  // let pw = 123444;
  // println!("{} => {}", pw, is_viable_pw2(&pw));
}

type Password = u32;

fn is_viable_pw(pw: &Password) -> bool {
  let mut ds = digits(pw).into_iter();
  let mut prev = ds.next().unwrap();
  let mut has_double = false;
  for d in ds {
    if prev > d {
      // must never decrease
      return false;
    }
    if prev == d {
      has_double = true;
    }
    prev = d;
  }
  return has_double;
}

fn is_viable_pw2(pw: &Password) -> bool {
  let mut ds = digits(pw).into_iter();
  let mut prev = ds.next().unwrap();
  let mut n = 0;
  let mut has_double = false;
  for curr in ds {
    if prev > curr {
      return false;
    }

    if prev == curr {
      n += 1;
    } else if n == 2 {
      has_double = true;
    }

    prev = curr;
  }
  return has_double;
}

fn digits(pw: &Password) -> Vec<u8> {
  pw.to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect()
}
