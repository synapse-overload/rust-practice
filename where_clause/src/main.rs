// ensure tabstop and shiftwidth are 4 so that the positional comments work
use std::fmt::Debug;

#[allow(unused)]
#[derive(Debug)]
struct Integer32 {
    val: i32,
}

trait PrintInOption {
    // Sample usage of self (like a "this" in C++) variable reference and Self
    // type
    // Note: we're trying to specify that the argument is a reference so that calling the
    // function does not invalidate the object it's being called on. If the argument was 'self'
    // then the calling code would not be allowed to use the bound variable anymore after calling
    // x.return_option()
    fn return_option(&self) -> Option<&Self>;
}

// if we remove lifetime param 'a the compiler will suggest that we add a new lifetime param
// to for, like for<'a> but that doesn't work the way you'd expect
// Note: in order to mention that the Option would have the same lifetime as the argument
// we need to add the lifetime requirement to the trait bounds
//                                                            v--- a new trait bound for Option<&'a T>
impl<'a, T> PrintInOption for T where Option<&'a T>: Debug + 'a {
//    ^-- new lifetime declaration                            ^-- initial problem here when there was no 'a
    fn return_option<'k>(&'k self) -> Option<&'k T> {
        Some(&self)
    }
}

fn main() {
    let x = Integer32 { val: 54 };

    let y = x.return_option();

    println!("the option y is now {:?}", y);
}

