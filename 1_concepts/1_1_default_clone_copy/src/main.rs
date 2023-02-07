use smart_default::SmartDefault;

#[derive(Clone, Copy, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, SmartDefault)]
enum Polyline {
    #[default]
    Point {
        #[default = 1]
        x: i32,
        y: i32,
    }
}

fn main() {}
