fn order_string<'a>(s1 : &'a str, s2 : &'a str) -> (&'a str, &'a str) {
    if s1.len() < s2.len() {
        return (s1, s2);
    }
    return (s2, s1);
}
pub fn main(){
    let str1 = String::from("long long long long string");
    let str2 = "short string";
    let (long_str, short_str) = order_string(str1.as_str(), str2);
    println!(" long={} nshort={} ", long_str, short_str);
}