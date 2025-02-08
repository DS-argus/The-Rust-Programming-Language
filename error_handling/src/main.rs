use std::fs::File;
use std::io::{self, Read};

// username을 담은 Ok값 혹은 error값을 return하는 함수
fn read_username_from_file1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), //함수의 마지막 표현식이기 때문에 명시적으로 return이라고 적을 필요는 없음
    }
}

// 동일한 기능이지만 propagation에 용이한 shortcut ?을 사용
// ?는 발생하는 error을 알아서 함수 return type error에 맞게 변경
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// ?의 위치를 통해 더 간결하게 표현가능
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {}
