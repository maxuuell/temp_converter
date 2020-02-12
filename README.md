## Rust Temp Converter

This is my first rust project, which is just the first challenge in the [Rust Programming Language Book](https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary).

The code works, and can be cloned and run with `cargo run`, etc.

I've left some notes in the code, to document my questions and findings. Some specifics that I wanted to call out are:

1. Coming from Node, Rust is hard. Not necessarily for any unique things about Rust itself, but mostly for all the type concerns I am introduced to. Specifically:
   - Converting `to_uppercase()` is not as straight forward as I'd like, though I can appreciate why.
   - Multiplying and dividing different types. The actual conversion calc was a trip, and I had to turn everything to an `f32`, from the beginning. That seems combursome to have to think about your entire forward computations right when you introduce the number. I am sure there are different, and better, ways to handle this.
   - Turbo fish. Great name. Totally confused why it's a thing.
   - Vectors, chars, str, String, collections, iterators, and that Result enum... trait... type?? The function signitures are just new, and I will have to get used to how and why Rust does things a certain ways.
2. Error handling seems strange, so far. The only error I came accross was `ParseFloatError` and had to treat explicitly at the top of `main`. It seems brittle to have to use a specific dependency just to handle a parse error, especially when you consider errors for many cases. Why can't parsing a string just return a more general parse error? Also, I had to do a sort of "catch" at the bottom with `OK()`, like it was some global. How do I treat many different types of errors in one function? I am sure once I read more on the specifics of error handling, some of this will become more clear.

This is my first project, so all of this may seem rude or naive, and it may be. I can appreciate all the decisions and tradeoffs the Rust team made to design all these items I'v brought up. Only time studying will help answer these questions.
