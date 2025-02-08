use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // match 표현식을 이용한 방법
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file! {:?}", other_error);
            }
        },
    };

    // unwrap_or_else 와 closure를 사용한 방법
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // panic 발생 shorcut
    // 1. unwrap : 알아서 Ok, Err 중 하나 발생시킴
    let greeting_file = File::open("hello.txt").unwrap();
    // 2. expect : error message까지 입력가능 => 더 많은 정보를 제공해서 실제로 자주 활용
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
