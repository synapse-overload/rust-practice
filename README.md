This repo contains practice programs for studying RUST.

Projects:
- [mandelbrot](./mandelbrot) - This is Listing 2.12 from "Rust in action" which had some problems which needed to be fixed
- [where_clause](./where_clause) - This is a program that was inspired by the example [here](https://doc.rust-lang.org/rust-by-example/generics/where.html?highlight=PrintInOption#where-clauses) which was
bothering me because the `vec` variable would be unusable after the call to `print_in_option`, and I really liked the ability to define a (sometimes dangerous and unintentionally broad) generic trait implementation that
would allow subsequent use of the variable after it worked, this pushed me into a fight with the borrow checker which needed to know the lifetime of the passed in reference
- a lot of other smaller projects which I'm not going to document, I just want to keep them for some reason that may be related to hoarding, at some point I will decide to delete them
                        

Notes:
A list of things that you should be able to answer after studying the language:

 1. What is a `Box`, `Rc`, and `Arc` ?
 2. What is the difference between `&`, `&mut`, `ref` and `ref mut`?
 3. What is the unit type?
 4. What is destructuring (not deconstruction!) and show to match variables work?
 5. What is a move block/closure/capture?
 6. What is a higher order function?
 7. What does `_` do in Rust?
 8. What is an attribute?
 9. What is a trait and how do you implement/use it?
10. What is a diverging function?
11. What is an option type, how do you create it with `Some` and `None`, and how to you unpack it with `?` ?
12. How do you `map` and `and_then` with options?
13. How do you convert `Option<Option<T>>` to `Option<T>`?
14. How do you create a list with `enum`s?
15. How many references of each type (`&mut` or `&`) can you hold and when do they prohibit each other?


Useful books:
- [Rust by example](https://doc.rust-lang.org/rust-by-example/) - A superfast book, that goes through Rust by showing you examples, this is 
  actually the second book I started WHILE reading Rust in action so that I can be able to get the Manning cerifiction for RUST until the 8th 
  of November 2023 when they cut the certifications, it's great to look into stuff quickly and get an example then develop your own more 
  complicated example to test how deep your understanding goes (i.e. prepare for bloody fights with the Borrow checker that you will lose)
- [The Book](https://doc.rust-lang.org/stable/book/) - A complete reference, sane and recommendable, takes a healthy approach to learning rust
- [Rust in action](https://www.manning.com/books/rust-in-action) - A nice starter book, but don't expect to get in too deep with it, excellent
  for after work rust learning, but if you want to get a job, you better be able to understand things deeper than what is presented here, this 
  is the first book on Rust I actually bought, the rest of the books seem to be online at docs.rust-lang.org
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/intro.html) - dark arts of unsafe rust, but also helpful with deep understanding of the language internals
- [The Rust Reference](https://doc.rust-lang.org/stable/reference/) - a book that seems to aim to completely document the base library and language (*has some details of inline assembly)
- [
