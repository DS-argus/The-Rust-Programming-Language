// 사용되지 않는 변수, 함수로 발생하는 warning 제거
#[allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // mut으로 선언해서 변경 가능
    user1.email = String::from("anotheremail@example.com");

    // email만 다르고 나머지는 user1과 동일
    // user1의 String type인 username의 소유권이 이동되어 이 이후 user1 사용 불가능
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
