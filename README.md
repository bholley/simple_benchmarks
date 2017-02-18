Simple benchmarks of common platform operations. Written in Rust but designed to (mostly) measure the metal.

Run |cargo bench| from the top-level. You may need to |rustup override set nightly|.

Here are the numbers on a late-2016 (touchbar) MBP, 2.9GHz i7:
```
test arc_clone             ... bench:           5 ns/iter (+/- 1)
test arc_clone_and_release ... bench:          12 ns/iter (+/- 6)
test atomic_fetch_add      ... bench:           5 ns/iter (+/- 1)
test heap_alloc            ... bench:          18 ns/iter (+/- 4)
test large_if_else_block   ... bench:           1 ns/iter (+/- 0)
test large_switch          ... bench:           1 ns/iter (+/- 0)
test virtual_call          ... bench:           4 ns/iter (+/- 0)
```
