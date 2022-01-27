pub fn main() {
    // println!("{}", longest_palindrome(String::from("eeee")));
    convert_and_print(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 27);
}

pub fn convert(s: String, num_rows: i32) -> String {
    s
}
pub fn convert_and_print(s: String, num_rows: i32) {
    let chars = s.as_bytes();
    let chars_in_group = if num_rows==1 {1} else {num_rows * 2 - 2};
    let columns_in_group_full = num_rows - 1;
    let len: i32 = chars.len() as i32;
    let groups = (len - 1) / chars_in_group + 1;
    
    for row in 0..num_rows {
        // let mut group = 0;
        for group in 0..groups {
            let first_char_index = chars_in_group * group + row;
            if first_char_index < len {
                print!("{}", chars[first_char_index as usize] as char);
            }
            let spaces_to_second_char = columns_in_group_full  - row;
            for _ in 1..spaces_to_second_char {
                print!("{}", ' ');
            }
            if row != 0 && row != num_rows - 1 {
                let second_char_index = chars_in_group * group + num_rows * 2 - row - 2;
                if second_char_index < len {
                    print!("{}", chars[second_char_index as usize] as char);
                }
            }
            for _ in 0..row - 1 {
                print!("{}", ' ');
            }
        }
        println!();
    }
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
