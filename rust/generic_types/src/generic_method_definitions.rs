#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

// Including T and U after the implement keyword hell's the rust compiler that will be implementing methods and functions for a rectangle with two generic types.
impl<T, U> Rectangle<T, U> {
    // We have to return a reference here because we don't know what T is, so if it's a string that lives on the heap then it needs to be a reference
    fn get_width(&self) -> &T {
        &self.width
    }
}

// We can also define methods that only apply to a specific type of rectangle, and for that we won't add the <T, U> after the impl keyword, and this tells the
// compiler that this implementation is meant for specific types, which in this case are u8's
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    // We can also use generics when implementing methods for a struct

    let rect = Rectangle {
        width: 1u8,
        height: 3u8,
    };

    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
}
