fn order_string<'a>(s1: &'a str, s2: &'a str) -> (&'a str, &'a str) {
    if s1.len() < s2.len() {
        return (s1, s2);
    }
    return (s2, s1);
}
// fn order_string(s1 : &str, s2 : &str) -> (&str, &str) {
//     if s1.len() < s2.len() {
//         return (s1, s2);
//     }
//     return (s2, s1);
// }
pub fn main() {
    let str1 = String::from("long long long long string");
    let str2 = "short string";
    let (long_str, short_str) = order_string(str1.as_str(), str2);
    println!(" long={} nshort={} ", long_str, short_str);
}

struct Person {
    name: String,
    age: u8,
}
fn lifetime_in_closure() {
    let p = Person {
        name: "Hao Chen".to_string(),
        age: 44,
    };
    //可以运行，因为 <code data-enlighter-language="raw" class="EnlighterJSRAW">u8</code> 有 Copy Trait
    // let age = |p : Person| p.age;
    // String 没有Copy Trait，所以，这里所有权就 Move 走了
    // let name = |p : Person | p.name;
    // println! ("name={}, age={}" , name(p), age(p));
    let age = |p: &Person| p.age;
    // let name = |p: &'a Person| &p.name;
    //下面的声明可以正确译
    let name: for<'a> fn(&'a Person) -> &'a String = |p: &Person| &p.name;
    println!("name={}, age={}", name(&p), age(&p));
}
