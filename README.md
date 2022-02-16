# cellvec
This library contains various Rust collection types that use interior mutability (mainly [std::cell::Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)).
This makes it easy to create complex graph relationships between types and since normal aliasable references are used the performance is 
excellent and the usage almost as ergonomic as normal pointer types (but of course totally safe).
