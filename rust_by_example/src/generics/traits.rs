// Non-copyable types
#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Null;

// A trait generic over `T`
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

trait SingleDrop<T> {
    fn single_drop(&self, _: T);
}

impl<T, U> SingleDrop<T> for U {
    fn single_drop(&self, _: T) {}
}

pub fn run() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`
    // empty.double_drop(null);

    // following wont work as its being deallocated.
    // empty;
    // null;

    null.single_drop(&empty);
    empty.double_drop(null);
}
