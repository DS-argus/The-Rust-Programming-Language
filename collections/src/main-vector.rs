#[allow(unused_variables)]
fn main() {
    // vector 생성하는 2가지 방법: 일반 / macro 이용
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // vector 생성하고 값 삽입
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // vector 요소 읽기
    let v = vec![1, 2, 3, 4, 5];

    // 없는 index에 접근하면 panic 발생해서 종료
    let third2 = v[2]; // i32인 v[2]를 Copy해서 stack에 메모리 할당
    let third1: &i32 = &v[2]; // heap momory에 대한 불변참조자 만드는 것
    println!("The third element is {third1}, {third2}");

    let third: Option<&i32> = v.get(2); // 없는 index에 접근하면 None return
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // 아이템의 참조자를 갖고 있는 상황에서 새로운 요소 추가 시도
    // => 같은 scope에서 mutable 2개 이상, mutable & immutable 동시에 가질 수 없음
    // (immutable 여러개만 가능)
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow 발생
    println! {"The first element is: {first}"}; // 여기에서 immutable borrow 제거하면 compile 됨
    let first = &mut v[0]; // first mutable borrow 발생
    println! {"The first element is: {first}"}; // 여기에서 mutable borrow 제거하면 compile 됨
    v.push(6); // second mutable borrow 발생 -> vector는 모든 요소가 붙어서 메모리에 저장. 새로운 요소
               // 추가할 때 공간 없으면 새로운 곳에 메모리 할당하고 복사. 이때 기존 요소 참조자가
               // 해제된 메모리 가리킬 수 있어 막아둔 것
    println! {"The first element is: {}", v[0]};

    // 벡터 값에 대해 반복
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // dereferencing
        println!("{i}");
    }

    // enum을 이용해 여러 type 저장
    // 컴파일 타임에 각 요소 저장을 위한 heap memory를 알아야해서 enum으로 처리
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
