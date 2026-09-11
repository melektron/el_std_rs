/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
09.09.26, 18:02

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree. 

This is a modified version of the code from the
https://github.com/crates-dev/clonelicious repository,
licensed under the following terms: 

MIT License

Copyright (c) 2024 crates-dev organization

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


/// A helper macro to clone variables into closures or async blocks easily.
///
/// This macro supports several usage patterns:
///
/// - Clone variables into a regular block.
/// - Clone variables into an `async move` block.
/// - Clone variables into a regular closure with or without explicit `move`.
/// - Clone variables into an `async move` closure with or without explicit `move`.
#[macro_export]
macro_rules! clone {
    ( $($rest:tt)* ) => {
        $crate::_clone_impl!(@parse [] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _clone_impl {
    (@parse [$($lets:tt)*] mut $var:ident , $($rest:tt)*) => {
        $crate::_clone_impl!(@parse [$($lets)* let mut $var = $var.clone();] $($rest)*)
    };
    (@parse [$($lets:tt)*] $var:ident , $($rest:tt)*) => {
        $crate::_clone_impl!(@parse [$($lets)* let $var = $var.clone();] $($rest)*)
    };
    (@parse [$($lets:tt)*] mut $var:ident => $($rest:tt)*) => {
        $crate::_clone_impl!(@body [$($lets)* let mut $var = $var.clone();] $($rest)*)
    };
    (@parse [$($lets:tt)*] $var:ident => $($rest:tt)*) => {
        $crate::_clone_impl!(@body [$($lets)* let $var = $var.clone();] $($rest)*)
    };
    (@parse [] => $($rest:tt)*) => {
        $crate::_clone_impl!(@body [] $($rest)*)
    };

    (@body [$($lets:tt)*] async move $body:block) => {{
        $($lets)*
        async move $body
    }};

    (@body [$($lets:tt)*] move || async move $body:block) => {{
        $($lets)*
        move || {
            $($lets)*
            async move $body
        }
    }};
    (@body [$($lets:tt)*] || async move $body:block) => {{
        $($lets)*
        move || {
            $($lets)*
            async move $body
        }
    }};
    (@body [$($lets:tt)*] move |$( $arg:ident $(: $ty:ty)? ),*| async move $body:block) => {{
        $($lets)*
        move |$( $arg $(: $ty)? ),*| {
            $($lets)*
            async move $body
        }
    }};
    (@body [$($lets:tt)*] |$( $arg:ident $(: $ty:ty)? ),*| async move $body:block) => {{
        $($lets)*
        move |$( $arg $(: $ty)? ),*| {
            $($lets)*
            async move $body
        }
    }};

    (@body [$($lets:tt)*] move || $body:block) => {{
        $($lets)*
        move || {
            $($lets)*
            $body
        }
    }};
    (@body [$($lets:tt)*] || $body:block) => {{
        $($lets)*
        move || {
            $($lets)*
            $body
        }
    }};
    (@body [$($lets:tt)*] move |$( $arg:ident $(: $ty:ty)? ),*| $body:block) => {{
        $($lets)*
        move |$( $arg $(: $ty)? ),*| {
            $($lets)*
            $body
        }
    }};
    (@body [$($lets:tt)*] |$( $arg:ident $(: $ty:ty)? ),*| $body:block) => {{
        $($lets)*
        move |$( $arg $(: $ty)? ),*| {
            $($lets)*
            $body
        }
    }};

    (@body [$($lets:tt)*] $body:block) => {{
        $($lets)*
        $body
    }};
}