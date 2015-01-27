#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unstable)]

#![feature(slicing_syntax)]

// Rexport stuff so I can use it in other projects
pub use slices_iterators::splice;
pub use slices_iterators::split_while;
pub use slices_iterators::take_while2 as take_while;

mod slices_iterators;
