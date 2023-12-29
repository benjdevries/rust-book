#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other)
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    rect.set_width(0);

    let rect_ref = &mut rect;

    rect_ref.set_width(5);

    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(other_rect);

    println!("{}", rect.area())
}
