//@error-pattern: which is protected
struct Newtype<'a>(&'a mut i32, i32);

fn dealloc_while_running(_n: Newtype<'_>, dealloc: impl FnOnce()) {
    dealloc();
}

// Make sure that we protect references inside structs that are passed as ScalarPair.
fn main() {
    let ptr = Box::into_raw(Box::new(0i32));
    #[rustfmt::skip] // I like my newlines
    unsafe {
        dealloc_while_running(
            Newtype(&mut *ptr, 0),
            || drop(Box::from_raw(ptr)),
        )
    };
}
