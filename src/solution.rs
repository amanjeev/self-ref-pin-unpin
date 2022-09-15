use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    /// You would think that a negative impl here would work but doing something
    /// like `!Unpin` is unsable.
    /// See https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html
    ///
    /// This PhantomPinned is critical here because by default most types in
    /// Rust impl `Unpin`. This means that most types, as long as the types
    /// they contain also impl `Unpin`, will be `Unpin`. This means at least one
    /// of those internal members should be `Pin` to make the struct immovable in
    /// the memory.
    ///
    /// Like `PhantomData`, you have `PhantomPinned` to your rescue. It is a
    /// zero-sized marker type that can be added to a field we do not use, like
    /// this. This makes the struct Not Unpin, and is marked as immovable in the memory.
    _pin: PhantomPinned,
}

impl Test {
    /// The constructor here needs to return a Pinned version of your result
    fn new(id: i32) -> Pin<Box<Self>> {
        let t = Test {
            a: format!("Test {}", id),
            b: std::ptr::null_mut(),
            _pin: PhantomPinned,
        };
        let mut t = Box::pin(t); // construction of a `Pin<Box<T>>`

        // Now after pinning, we take the reference
        let self_ref: *const String = &t.a;

        // To mutate any data underneath Pinned stuff we need to use `unsafe`
        unsafe {
            t.as_mut().get_unchecked_mut().b = self_ref;
        }
        t
    }

    fn a(&self) -> &String {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*self.b }
    }
}

fn main() {
    let mut t1 = Test::new(1);
    let mut t2 = Test::new(2);

    println!("{:?} {:?}", t1.a(), t1.b());
    println!("{:?} {:?}", t2.a(), t2.b());

    std::mem::swap(&mut t1, &mut t2);

    println!("{:?} {:?}", t1.a(), t1.b());
    println!("{:?} {:?}", t2.a(), t2.b());
}
