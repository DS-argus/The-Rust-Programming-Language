// 이 함수가 return할 문자열 슬라이스는 적어도
// input parameter의 lifecycle만큼은 살아있다는 뜻
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// x를 return하니 y에는 lifetime parameter없어도 됨
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
