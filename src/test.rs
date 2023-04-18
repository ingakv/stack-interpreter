use std::collections::hash_map::Keys;
use std::env::{var, Vars};

fn apply2(f: i32) -> Vars {
    let i = (2 * f);
    i
}

// A function which takes a closure and returns an `i32`.
fn apply<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

pub(crate) fn main() {
    use std::mem;

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply(double));

    let new_var = apply2(5);
    println!("3 doubled: {}", new_var);

}