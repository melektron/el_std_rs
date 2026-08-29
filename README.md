# el_std_rs

An assortment of useful rust utilities complementing some builtin and third party rust crates.

Note: On [crates.io](https://crates.io) the package is called `el_std` without the `_rs` suffix, as per convention.


## Disclaimer

1. I mainly develop this library for my own use use, and while I do think that it might be useful to others, there is no guarantee that features work as expected and I may introduce breaking changes to APIs at any time.
2. While the `el_std_rs` is namely related to my [`el_std_cpp`](https://github.com/melektron/el_std_cpp) and [`el_std_py`](https://github.com/melektron/el_std_py) libraries, they are not related in any way content-wise and there is no promise of feature parity whatsoever. All these libraries are simply utility libraries for the respective languages with features I commonly use in them. They are otherwise independent.


## Versions

I am currently not targeting any specific rust version, but in general I will not refrain from using the latest rust features when I need them, so compatibility with older rust versions is not guaranteed.


## Modules and Documentation

While I might write more comprehensive documentation for some modules along the way, I will at least try to maintain an up-to-date list of all available functionality:

- `el_std::or_log`: Methods to log a message if a result is an error.
- `el_std::poisonless_lock`: Methods for std::sync::poison::Mutex and similar to intentfully unwrap lock results.
- `el_std::terminal`: Functionality to setup an async interactive terminal REPL with non-interfering logging and a good starting point for log formatting based on env_logger. This module serves a similar purpose to [`el_std_py`](https://github.com/melektron/el_std_py)'s `el.terminal`.
- `el_std::autowrap`: Derive macros to derive common types (Deref, ...) for simple wrapper structs (`struct A(B);`).

These modules are currently not configurable via Cargo feature flags.