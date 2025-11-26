# Structures

# Rust Lock-Free Stack Roadmap

## Project Goals

### 1. Basic Treiber Stack (Push/Pop)

- [ ] Implement a generic `TreiberStack<T>` using `AtomicPtr` / `Atomic` types.
- [ ] Support lock-free `push` and `pop` operations using CAS loops.
- [ ] Add unit tests covering:
  - [ ] Single-threaded correctness (LIFO behavior).
  - [ ] Basic multi-threaded stress tests (multiple producers/consumers).
- [ ] Add simple benchmarks to measure throughput under contention.

---

### 2. Treiber Stack with `peek` and Native Rust SMR

- [ ] Support a `peek` operation without violating memory safety.
- [ ] Use `Arc` for Safe Memory Reclamation (SMR) 
  - ensure `peek` does not access reclaimed nodes 
    - Protect nodes while they are being read
    - Defer reclamation until no references remain
- [ ] Extend tests:
  - [ ] Verify `peek` returns the current top element when present.
  - [ ] Stress-test interleavings of `push`, `pop`, and `peek`.

---

### 3. Treiber Stack Using Crossbeam’s EBR

- [ ] Replace custom SMR with Crossbeam’s Epoch-Based Reclamation (EBR).
- [ ] Refactor node allocation and reclamation to use `crossbeam_epoch` primitives.
- [ ] Benchmark comparison:
  - [ ] Crossbeam EBR (throughput, latency).
  - [ ] Document trade-offs in performance and implementation ease `Arc` vs `Crossbeam`.

## License

All code in this repository is dual-licensed under either:

- Apache License, Version 2.0
- MIT license

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both as noted in
this [issue](https://github.com/bevyengine/bevy/issues/2373) on [Bevy](https://bevyengine.org)'s repo.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
