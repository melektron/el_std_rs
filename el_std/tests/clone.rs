/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
09.09.26, 18:31
All rights reserved.

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

use el_std::clone;

#[tokio::test]
async fn test_clone_block() {
    // plain block
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res: String = clone!(s1, s2 => {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    });
    assert_eq!(res, format!("{s1} {s2}"));

    // async move block
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res: String = clone!(s1, s2 => async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    })
    .await;
    assert_eq!(res, format!("{s1} {s2}"));
}

#[tokio::test]
async fn test_clone_regular_closure() {
    // regular closure (without parameter type annotation)
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => |data| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

    // regular closure (with parameter type annotation)
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => |data: &str| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

    // regular closure with explicit move (without parameter type annotation)
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));

    // regular closure with explicit move (with parameter type annotation)
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data: &str| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));
}

#[tokio::test]
async fn test_clone_async_closure() {
    // async move closure (without parameter type annotation)
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 =>  |data| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!").await, String::from("Hello World!"));

    // async move closure (with parameter type annotation)
    // note: using String here because explicit reference
    // without lifetime may not outlive the async block
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => |data: String| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!".to_owned()).await, format!("{} {}{}", s1, s2, "!"));

    // async move closure with explicit move (without parameter type annotation)
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!").await, format!("{} {}{}", s1, s2, "!"));

    // async move closure with explicit move (with parameter type annotation)
    // note: using String here because explicit reference
    // without lifetime may not outlive the async block
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data: String| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!".to_owned()).await, format!("{} {}{}", s1, s2, "!"));
}


#[tokio::test]
async fn test_clone_mut() {
    // plain block
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(mut s1, s2 => {
        s1.push_str(", mutated!");
        assert_eq!(s2, String::from("World"));
        s1.clone()
    });
    assert_eq!(res, "Hello, mutated!");
    assert_eq!(s1, "Hello"); // original untouched, only the clone was mutable
    
    // async move block
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res: String = clone!(mut s1, s2 => async move {
        s1.push_str(", mutated!");
        assert_eq!(s2, String::from("World"));
        s1.clone()
    })
    .await;
    assert_eq!(res, "Hello, mutated!");
    assert_eq!(s1, "Hello");
    
    // regular closure
    // note: also proves the clone is re-taken fresh on 
    // every call, not accumulated across calls
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(mut s1, s2 => |data: &str| {
        s1.push_str(data);
        assert_eq!(s2, String::from("World"));
        s1.clone()
    });
    assert_eq!(res("!"), "Hello!");
    assert_eq!(res("?"), "Hello?"); // NOT "Hello!?" — fresh clone each call
    assert_eq!(s1, "Hello");
    
    // regular closure with explicit move
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(mut s1, s2 => move |data: &str| {
        s1.push_str(data);
        assert_eq!(s2, String::from("World"));
        s1.clone()
    });
    assert_eq!(res("!"), "Hello!");
    assert_eq!(res("?"), "Hello?");
    assert_eq!(s1, "Hello");
    
    // async move closure
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(mut s1, s2 => |data: String| async move {
        s1.push_str(&data);
        assert_eq!(s2, String::from("World"));
        s1.clone()
    });
    assert_eq!(res("!".to_owned()).await, "Hello!");
    assert_eq!(res("?".to_owned()).await, "Hello?");
    assert_eq!(s1, "Hello");
    
    // async move closure with explicit move
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(mut s1, s2 => move |data: String| async move {
        s1.push_str(&data);
        assert_eq!(s2, String::from("World"));
        s1.clone()
    });
    assert_eq!(res("!".to_owned()).await, "Hello!");
    assert_eq!(res("?".to_owned()).await, "Hello?");
    assert_eq!(s1, "Hello");
}


#[tokio::test]
async fn test_clone_zero_param_closure() {
    //! || is parsed as or operator token, thus needs special handling
    
    // regular closure
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => || {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2} a")
    });
    assert_eq!(res(), format!("{s1} {s2} a"));

    // regular closure with explicit move
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move || {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    });
    assert_eq!(res(), format!("{s1} {s2}"));

    // async move closure
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => || async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    });
    assert_eq!(res().await, format!("{s1} {s2}"));

    // async move closure with explicit move
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move || async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    });
    assert_eq!(res().await, format!("{s1} {s2}"));

}

#[tokio::test]
async fn test_clone_single_var() {
    //! single var, no trailing comma
    
    // plain block
    let s1: String = String::from("Hello");
    let res = clone!(s1 => { s1.clone() });
    assert_eq!(res, "Hello");

    // plain block with mut
    let s1: String = String::from("Hello");
    let res = clone!(mut s1 => { s1.push_str("!"); s1 });
    assert_eq!(res, "Hello!");
    assert_eq!(s1, "Hello");

    // async move block
    let s1: String = String::from("Hello");
    let res: String = clone!(s1 => async move { s1.clone() }).await;
    assert_eq!(res, "Hello");

    // async move block with mut
    let s1: String = String::from("Hello");
    let res: String = clone!(mut s1 => async move { s1.push_str("!"); s1 }).await;
    assert_eq!(res, "Hello!");
    assert_eq!(s1, "Hello");

    // regular closure (zero parameters)
    let s1: String = String::from("Hello");
    let res = clone!(mut s1 => || {
        s1.push_str("!");
        s1.clone()
    });
    assert_eq!(res(), "Hello!");
    assert_eq!(res(), "Hello!"); // fresh clone each call
    assert_eq!(s1, "Hello");

    // regular closure with explicit move (zero parameters)
    let s1: String = String::from("Hello");
    let res = clone!(mut s1 => move || {
        s1.push_str("!");
        s1.clone()
    });
    assert_eq!(res(), "Hello!");
    assert_eq!(s1, "Hello");

    // async move closure (zero parameters)
    let s1: String = String::from("Hello");
    let res = clone!(mut s1 => || async move {
        s1.push_str("!");
        s1.clone()
    });
    assert_eq!(res().await, "Hello!");
    assert_eq!(s1, "Hello");

    // async move closure with explicit move (zero parameters)
    let s1: String = String::from("Hello");
    let res = clone!(mut s1 => move || async move {
        s1.push_str("!");
        s1.clone()
    });
    assert_eq!(res().await, "Hello!");
    assert_eq!(s1, "Hello");
}

#[tokio::test]
async fn test_clone_empty_clone_list() {
    let res = clone!(=> { 42 });
    assert_eq!(res, 42);

    let res: i32 = clone!(=> async move { 42 }).await;
    assert_eq!(res, 42);

    // One cannot use `|| 42`, as that won't match.
    // A block is mandatory.
    let f = clone!(=> || { 42 });
    assert_eq!(f(), 42);

    // One cannot use `move || 42``, as that won't match.
    // A block is mandatory.
    let f = clone!(=> move || { 42 });
    assert_eq!(f(), 42);

    let f = clone!(=> || async move { 42 });
    assert_eq!(f().await, 42);

    let f = clone!(=> move || async move { 42 });
    assert_eq!(f().await, 42);
}

#[tokio::test]
async fn test_clone_three_vars_mixed_mut() {
        
    let s1: String = String::from("A");
    let s2: String = String::from("B");
    let s3: String = String::from("C");
    let res = clone!(s1, mut s2, s3 => {
        s2.push('!');
        format!("{s1}{s2}{s3}")
    });
    assert_eq!(res, "AB!C");
    assert_eq!(s2, "B"); // untouched original

    let s1: String = String::from("A");
    let s2: String = String::from("B");
    let s3: String = String::from("C");
    let res = clone!(mut s1, mut s2, mut s3 => {
        s1.push('1');
        s2.push('2');
        s3.push('3');
        format!("{s1}{s2}{s3}")
    });
    assert_eq!(res, "A1B2C3");
    assert_eq!((s1, s2, s3), (String::from("A"), String::from("B"), String::from("C")));

    let s1: String = String::from("A");
    let s2: String = String::from("B");
    let s3: String = String::from("C");
    let res = clone!(mut s1, s2, mut s3 => async move {
        s1.push('1');
        s3.push('3');
        format!("{s1}{s2}{s3}")
    })
    .await;
    assert_eq!(res, "A1BC3");
}