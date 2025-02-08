use std::fmt::{Debug, Display};

pub trait Summary {
    // 구현이 없으니 struct에 구현 필수
    fn summarize_author(&self) -> String;

    // 구현이 되어 있으니, struct에 없으면 기본 구현 사용
    // struct에 있으면 overriding
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl Trait 문법
pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// traint bound 문법
// 더 복잡한 상황 간결하게 표현 가능
// ex) pub fn notify3<T: Summary>(item1: &T, item2: &T)
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound 여러 개 지정 가능
pub fn notify3(item: &(impl Summary + Display)) {}
pub fn notify3<T: Summary + Display>(item: &T) {}

// where 로 trait bound 정리
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// Triat를 return type으로 사용
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_Ebooks"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

// 근데 같은 trait라도 다양한 type return은 안됨
// impl Trait 문법이 컴파일러 내에 구현된 방식으로 인한 제약 때문에 발생
// 17장에서 가능하게 하는 법 배움
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Tweet {
            username: String::from("horse_Ebooks"),
            content: String::from("content"),
            reply: false,
            retweet: false,
        }
    } else {
        NewsArticle {
            headline: String::from("headline"),
            location: String::from("Seoul"),
            author: String::from("Iceburgh"),
            content: String::from("content"),
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 특정 trait가 구현된 타입에서만 사용가능한 메소드
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is x = {}", self.y);
        }
    }
}
