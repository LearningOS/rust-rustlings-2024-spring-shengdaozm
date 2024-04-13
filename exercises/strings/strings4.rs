// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}
//考察字符串的类型转换
fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    //引用用类型的字符串（如 &str）转换为一个拥有所有权的 String 类
    string("rust is fun!".to_owned());
    // 仍然是类型转换
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    //trim() 方法返回一个新的字符串切片，该切片是原字符串切片去除首尾空白字符后的结果。
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
