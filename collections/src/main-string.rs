#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    // 비어있는 새로운 문자열 생성
    let mut s = String::new();

    // 문자열 literal에서 string으로
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // UTF-8 encoding으로 적합하게 encoding된 모든 데이터 삽입 가능
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // 문자열 업데이트
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str은 문자열 슬라이스를 매개변수로. 소유권 필요 없어서

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}"); // 소유권이 넘어간게 아니라서 s2 출력가능

    let mut s = String::from("lo");
    s.push('l'); // push는 글자 한 개를 받아서 추가
    println!("s is {s}");

    // + 연산자, format!을 이용한 concat
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 소유권 이동. fn add(self, s: &str) -> String 이라서 이런 형태.
                       // 여기서 &s2는 &String이지만 defer coercion이 발생해서 강제로 add함수
                       // 내부에서 &s2를 &s2[..]으로 수정
                       // 결과적으로 s1 메모리에 s2 내용을 추가하는 것

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;  // 불편
    let s = format!("{s1}-{s2}-{s3}"); // 모두 참조자 이용해서 소유권 변경 없음

    // indexing
    let s1 = String::from("hello");
    // let h = s1[0]; // rust String은 indexing 지원 X

    let hello = "Здравствуйте";
    let s = &hello[0..10]; //구체적으로 4 byte 지정 : 이 문자열의 경우 각 글자가 2 byte
    println!("{}", s);

    // char type 2개의 값 출력
    for c in "Зд".chars() {
        println!("{c}");
    }

    // byte 값 출력
    for c in "Зд".bytes() {
        println!("{c}");
    }
}
