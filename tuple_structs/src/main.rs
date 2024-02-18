// A tuple struct is the same as a normal struct, but instead of it having
// field names, the fields are indexed and may, as usual have varying types
// instead of 
//   struct Color {
//      r: i32,
//      g: i32,
//      b:i32
//  } we can have
#[derive(Debug)]
struct Color(i32, i32, i32);

// Also similar to tuple structs we can have "unit-like" structs
#[derive(Debug)]
struct AlwaysTrue;

fn main() {
    let red = Color(255, 0, 0);
    println!("Red is {:?}", red);
    let always_true_var = AlwaysTrue;
    println!("What would True look like as a struct? {:?}", always_true_var);
}
