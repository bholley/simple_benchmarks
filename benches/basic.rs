#![feature(test)]

extern crate test;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::mem;
use test::Bencher;
use test::black_box;

#[derive(Default)]
struct Bar(pub u64);

#[bench]
fn atomic_fetch_add(b: &mut Bencher) {
    let a = AtomicUsize::new(5);
    b.iter(|| a.fetch_add(1, Ordering::Acquire));
    black_box(a);
}

#[bench]
fn arc_clone(b: &mut Bencher) {
    let a = Arc::new(2);
    b.iter(|| mem::forget(a.clone()));
}

#[bench]
fn arc_clone_and_release(b: &mut Bencher) {
    let a = Arc::new(2);
    b.iter(|| a.clone());
}

#[bench]
fn heap_alloc(b: &mut Bencher) {
    b.iter(|| mem::forget(black_box(Box::new(2))));
}

pub trait Foo {
    fn fun(&mut self) -> u64;
}

impl Foo for Bar {
    fn fun(&mut self) -> u64 {
        self.0 += black_box(5);
        self.0
    }
}

#[bench]
fn virtual_call(b: &mut Bencher) {
    let mut bar = Bar(100);
    b.iter(|| black_box(black_box::<&mut Foo>(&mut bar).fun()));
}

pub enum Task {
    A(u64),
    B(u64),
    C(u64),
    D(u64),
    E(u64),
    F(u64),
    G(u64),
    H(u64),
    I(u64),
    J(u64),
    K(u64),
    L(u64),
    M(u64),
    N(u64),
    O(u64),
    P(u64),
}

impl Task {
    fn exec(&mut self) -> u64 {
        use Task::*;
        match *self {
            A(ref mut x) => { *x += 1; black_box(*x) },
            B(ref mut x) => { *x += 2; *x },
            C(ref mut x) => { *x += 3; *x },
            D(ref mut x) => { *x += 4; *x },
            E(ref mut x) => { *x += 5; *x },
            F(ref mut x) => { *x += 6; *x },
            G(ref mut x) => { *x += 7; *x },
            H(ref mut x) => { *x += 8; *x },
            I(ref mut x) => { *x += 1; *x },
            J(ref mut x) => { *x += 2; *x },
            K(ref mut x) => { *x += 3; *x },
            L(ref mut x) => { *x += 4; *x },
            M(ref mut x) => { *x += 5; *x },
            N(ref mut x) => { *x += 6; *x },
            O(ref mut x) => { *x += 7; *x },
            P(ref mut x) => { *x += 8; *x },
        }
    }
}

#[bench]
fn large_switch(b: &mut Bencher) {
    let mut task = Task::P(5);
    b.iter(|| black_box(black_box::<&mut Task>(&mut task).exec()));
}

// Note: I stepped through this, and we do appear to iteratively check the all
// the cases (that is to say, there didn't appear to be any funny optimizations
// at work).
fn if_else(x: u64) -> u64 {
    if x == 111 {
        black_box(1)
    } else if x == 222 {
        black_box(2)
    } else if x == 333 {
        black_box(3)
    } else if x == 444 {
        black_box(4)
    } else if x == 555 {
        black_box(5)
    } else if x == 666 {
        black_box(6)
    } else if x == 777 {
        black_box(7)
    } else if x == 888 {
        black_box(8)
    } else if x == 999 {
        black_box(9)
    } else if x == 1111 {
        black_box(1)
    } else if x == 2222 {
        black_box(2)
    } else if x == 3333 {
        black_box(3)
    } else if x == 4444 {
        black_box(4)
    } else if x == 5555 {
        black_box(5)
    } else if x == 6666 {
        black_box(6)
    } else if x == 7777 {
        black_box(7)
    } else if x == 8888 {
        black_box(8)
    } else if x == 9999 {
        black_box(9)
    } else if x == 11111 {
        black_box(1)
    } else if x == 22222 {
        black_box(2)
    } else if x == 33333 {
        black_box(3)
    } else if x == 44444 {
        black_box(4)
    } else if x == 55555 {
        black_box(5)
    } else if x == 66666 {
        black_box(6)
    } else if x == 77777 {
        black_box(7)
    } else if x == 88888 {
        black_box(8)
    } else if x == 99999 {
        black_box(9)
    } else if x == 111111 {
        black_box(1)
    } else if x == 222222 {
        black_box(2)
    } else if x == 333333 {
        black_box(3)
    } else if x == 444444 {
        black_box(4)
    } else if x == 555555 {
        black_box(5)
    } else if x == 666666 {
        black_box(6)
    } else if x == 777777 {
        black_box(7)
    } else if x == 888888 {
        black_box(8)
    } else if x == 999999 {
        black_box(9)
    } else {
        black_box(2)
    }
}

#[bench]
fn large_if_else_block(b: &mut Bencher) {
    b.iter(|| black_box(if_else(black_box(888888))));
}
