fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// <f32, f32> 타입인 Point 객체에서만 정의되는 메소드
// 따라서 impl뒤에 type이 따로 필요 없음. 구체적인 실제 type이기 때문에
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {result}");

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    let p4 = Point { x: 3.0, y: 4.0 };

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p4 distance from origin : {}", p4.distance_from_origin());
}
