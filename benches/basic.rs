#![feature(test)]

extern crate fnv;
extern crate simple_benchmarks;
extern crate test;

use simple_benchmarks::chunkvec::ChunkVec;
use simple_benchmarks::stylearc::Arc as StyleArc;
use fnv::FnvHashMap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::mem;
use test::Bencher;
use test::black_box;

#[derive(Default)]
struct Bar(pub u64);

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
fn arc_new_and_drop(b: &mut Bencher) {
    b.iter(|| Arc::new(2));
}

#[bench]
fn style_arc_clone(b: &mut Bencher) {
    let a = StyleArc::new(2);
    b.iter(|| mem::forget(a.clone()));
}

#[bench]
fn style_arc_clone_and_release(b: &mut Bencher) {
    let a = StyleArc::new(2);
    b.iter(|| a.clone());
}

#[bench]
fn style_arc_new_and_drop(b: &mut Bencher) {
    b.iter(|| StyleArc::new(2));
}
