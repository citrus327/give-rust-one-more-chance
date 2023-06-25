#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print(&self) {
        println!("The area of the rectangle {:#?} is {}", self, self.area());
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 40,
        height: 40
    };

    // println!("The area of the rectangle {:#?} is {}", rectangle, calc_area(&rectangle));
    // println!("The area of the rectangle {:#?} is {}", rectangle, rectangle.area());
    rectangle.print();
    Rectangle::print(&rectangle);

    let square = Rectangle::square(12);

    square.print();

}

fn calc_area (rectangle: &Rectangle) -> u32{
    return rectangle.width * rectangle.height;
}
