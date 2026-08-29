/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
28.08.26, 22:28
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree. 
*/

use el_std::autowrap::{AutoDeref, AutoDerefMut};

#[derive(AutoDeref)]
struct Wrapper(String);

#[derive(AutoDeref, AutoDerefMut)]
struct MutableWrapper(String);


#[test]
fn deref_works() {
    let wrapper = Wrapper("hello".to_owned());

    // Deref coercion to &str
    assert_eq!(wrapper.len(), 5);
    assert_eq!(&*wrapper, "hello");

    // Explicit Deref
    let value: &String = std::ops::Deref::deref(&wrapper);
    assert_eq!(value, "hello");
}

#[test]
fn deref_mut_works() {
    let mut wrapper = MutableWrapper("hello".to_owned());

    // Deref
    assert_eq!(wrapper.len(), 5);

    // DerefMut
    wrapper.push_str(" world");
    

    assert_eq!(&*wrapper, "hello world");
}


#[derive(AutoDeref, AutoDerefMut)]
struct GenericWrapper<T>(T);

#[test]
fn generic_wrapper_works() {
    let mut wrapper = GenericWrapper(String::from("hello"));

    assert_eq!(wrapper.len(), 5);

    wrapper.push_str(" world");

    assert_eq!(&*wrapper, "hello world");
}


#[derive(AutoDeref, AutoDerefMut)]
struct RefWrapper<'a, T>(&'a T);

#[test]
fn lifetime_wrapper_works() {
    let value = String::from("hello");
    let wrapper = RefWrapper(&value);

    assert_eq!(wrapper.len(), 5);
}
