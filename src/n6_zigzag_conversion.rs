pub fn main() {
    // println!("{}", longest_palindrome(String::from("eeee")));
    convert_and_print(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 6);
    println!("{}", convert(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), 6));
}

pub fn convert(s: String, num_rows: i32) -> String {
    let chars = s.as_bytes();
    let len: i32 = chars.len() as i32;
    let mut result = Vec::with_capacity(len as usize);
    let chars_in_group = if num_rows == 1 { 1 } else { num_rows * 2 - 2 };
    let groups = (len - 1) / chars_in_group + 1;

    // let mut i = 0;
    for row in 0..num_rows {
        // let mut group = 0;
        for group in 0..groups {
            let first_char_index = chars_in_group * group + row;
            if first_char_index < len {
                result.push(chars[first_char_index as usize]);
                // result[i] = chars[first_char_index as usize];
                // i+=1;
            }
            if row != 0 && row != num_rows - 1 {
                let second_char_index = chars_in_group * group + num_rows * 2 - row - 2;
                if second_char_index < len {
                    result.push(chars[second_char_index as usize]);
                    // result[i] = chars[second_char_index as usize];
                    // i+=1;
                }
            }
        }
    }

    String::from_utf8(result).expect("Invalid utf-8 in [u8].")
}
pub fn convert_and_print(s: String, num_rows: i32) {
    let chars = s.as_bytes();
    let chars_in_group = if num_rows == 1 { 1 } else { num_rows * 2 - 2 };
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
            let spaces_to_second_char = columns_in_group_full - row;
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
