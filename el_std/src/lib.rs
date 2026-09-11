//! An assortment of useful rust utilities complementing some builtin and third party rust crates.
//! 
//! ## Disclaimer
//! 
//! 1. I mainly develop this library for my own use use, and while I do think that it might be useful to others, there is no guarantee that features work as expected and I may introduce breaking changes to APIs at any time.
//! 2. While the `el_std_rs` is namely related to my [`el_std_cpp`](https://github.com/melektron/el_std_cpp) and [`el_std_py`](https://github.com/melektron/el_std_py) libraries, they are not related in any way content-wise and there is no promise of feature parity whatsoever. All these libraries are simply utility libraries for the respective languages with features I commonly use in them. They are otherwise independent.
//! 
//! ## Versions
//! 
//! I am currently not targeting any specific rust version, but in general I will not refrain from using the latest rust features when I need them, so compatibility with older rust versions is not guaranteed.
//! 

#[cfg(feature = "poisonless_lock")]
pub mod poisonless_lock;
#[cfg(feature = "or_log")]
pub mod or_log;
#[cfg(feature = "terminal")]
pub mod terminal;
#[cfg(feature = "autowrap")]
pub mod autowrap;
#[cfg(feature = "clone")]
mod clone;