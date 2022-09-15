/// This struct has a member that refers to another member of the
/// same struct. Referring to itself in this way means that we have
/// pointer magic that may result in a footgun we may not anticipate.
///
/// Your job is to introduce two things -
///     1. Pin
///     2. PhantomPinned
///
/// Using the above two, you need to fix the output so it looks like this
/// ```
/// "Test 1" "Test 1"
/// "Test 2" "Test 2"
/// "Test 2" "Test 2"
/// "Test 1" "Test 1"
/// ```
#[derive(Debug)]
struct Test {
    a: String,        // a heap stored value to force non-stack stuff
    b: *const String, // refers to `a`
}

impl Test {
    fn new(id: i32) -> Test {
        let mut t = Test {
            a: format!("Test {}", id),
            b: std::ptr::null_mut(),
        };
        let self_ref: *const String = &t.a;
        t.b = self_ref;
        t
    }

    /// helper getter for `a`
    fn a(&self) -> &String {
        &self.a
    }

    /// helper getter for `b`
    fn b(&self) -> &String {
        unsafe { &*self.b }
    }
}

fn main() {
    let mut t1 = Test::new(1);
    let mut t2 = Test::new(2);

    // Printing something like -
    // "Test 1" "Test 1"
    // "Test 2" "Test 2"
    println!("{:?} {:?}", t1.a(), t1.b());
    println!("{:?} {:?}", t2.a(), t2.b());

    // here we swap the locations and since `b` refers to `a` within
    // the struct `Test`, we may run into an issue where the members
    // still point to the older locations.
    std::mem::swap(&mut t1, &mut t2);

    // Printing something like -
    // "Test 2" "Test 1"
    // "Test 1" "Test 2"
    println!("{:?} {:?}", t1.a(), t1.b());
    println!("{:?} {:?}", t2.a(), t2.b());
}
