struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let a = Point {x: 100, y: -1_f32};
    println!("x = {} y = {}", a.x, a.y);

    let b = Point {x: 10.1, y: -2.3_f32};
    println!("x = {} y = {}", b.x, b.y);
}