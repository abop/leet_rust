pub fn main() {
    println!("{}", longest_palindrome(String::from("eeee")));
}

pub fn longest_palindrome(s: String) -> String {
    let chars = s.as_bytes();
    let len = chars.len();
    if len <= 1 {
        return s;
    }
    let mut min = 0;
    let mut max = 0;
    let mut core = 0;
    let mut left;
    let mut right;
    while core < len - 1 {
        left = core;
        right = core;

        while right + 1 < len && chars[core] == chars[right + 1] {
            right += 1;
        }
        core = (left + right) / 2 + 1;

        while left >= 1 && right + 1 < len {
            if chars[left - 1] == chars[right + 1] {
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }
        if right - left > max - min {
            max = right;
            min = left;
        }
    }
    return String::from(&s[min..max + 1]);
}
