use pract_traits2::*;

fn main() {
    let my_rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    let my_triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };

    let my_circle = Circle {
        radius: 3.0,
    };

    print_info(&my_rectangle);
    print_info(&my_triangle);
    print_info(&my_circle);
}