use std::fmt::Debug;
use std::collections::HashMap;

#[macro_export]
macro_rules! svec {
    ($($x:expr),*) => ({
        let mut v = vec![$($x),*];
        v.sort();
        v
    });
    ($($x:expr,)*) => (svec![$($x),*])
}

#[macro_export]
macro_rules! strvec {
    ($($x:expr),*) => ({
        vec![$(String::from($x)),*]
    });
    ($($x:expr,)*) => (strvec![$($x),*])
}

#[macro_export]
macro_rules! vec2 {
    ($([$($x:expr),*]),*) => ({
        vec![$(vec![$($x),*]),*]
    });
}

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),*) => {{
        let mut map = HashMap::new();
        $(map.insert($k, $v);)*
        map
    }}
}

pub fn vec_eq<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for a in v1 {
        if v2.iter().find(|&b| a == b).is_none() {
            return false;
        }
    }
    true
}

pub fn vec2_eq<T: PartialEq>(v1: &Vec<Vec<T>>, v2: &Vec<Vec<T>>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for a in v1 {
        if v2.iter().find(|&b| vec_eq(a, b)).is_none() {
            return false;
        }
    }
    true
}

pub fn assert_vec2_eq<T: PartialEq + Debug>(v1: &Vec<Vec<T>>, v2: &Vec<Vec<T>>) {
    if !vec2_eq(v1, v2) {
        assert_eq!(v1, v2);
    }
}

pub fn assert_vec_eq<T: PartialEq + Debug>(v1: &Vec<T>, v2: &Vec<T>) {
    if !vec_eq(v1, v2) {
        assert_eq!(v1, v2);
    }
}
