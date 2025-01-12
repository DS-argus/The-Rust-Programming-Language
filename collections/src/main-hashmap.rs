use std::collections::HashMap;

fn main() {
    // 새로운 HashMap 생성
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 값 접근
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // 해당하는 값 없으면 None
    println!("The score of team {team_name} is {score}");

    // for loop으로 값 출력
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // map.insert(field_name, field_value); // 소유권 이동
    // map.insert(field_name.clone(), field_value.clone()); // 깊은 복사
    map.insert(&field_name, &field_value); // reference
    println!("{field_name} and {field_value}");

    // 값 업데이트
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // 마지막 값으로 수정됨

    // key가 없는 경우에만 추가. 있으면 수정 안함
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert는 해당 value에 대한 &mut V 반환
        *count += 1;
    }
    println!("{:?}", map);
}
