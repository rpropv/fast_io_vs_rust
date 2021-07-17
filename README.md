# fast_io_vs_rust

**NOTE:** This is just a fun little benchmark to poke fun at [/u/bisixyf](https://old.reddit.com/user/bisixyf) (the author of the C++ [fast_io](https://github.com/tearosccebe/fast_io) library) because I know how much they hate Rust. :)

It's a bit of a contrived example, but whatever. Try it out for yourself and let me know how it goes!

## Background

> fast_io is a new C++20 library for extremely fast input/output

- [fast_io](https://github.com/tearosccebe/fast_io)

Well, let's test the input/output speeds then, shall we?

We don't want to test the formatting speed, just the I/O. Therefore, we'll be writing 10 billion "a" characters to standard out. However, so that we don't include the speed of the terminal itself in the benchmark, we'll just throw all of those characters to `/dev/null`.

We'll just use the `time` command in linux/unix/etc because I can't be bothered to set up a proper benchmark in C++.

The resulting benchmark will therefore look like:

`time $BINARY > /dev/null`

## Compiling the programs under test

#### C++

`main.cpp`

```cpp
#include <fast_io.h>

int main() {
    for (unsigned long i=0; i<10'000'000'000; i++) {
        print("a");
    }
}
```

`fast_io` suggests two different compiling commands:

 1. `g++ -o example example.cc -Ofast -std=c++20 -s` ([source](https://github.com/tearosccebe/fast_io#getting-started), although you do need the `-I` option below too)
 2. `g++ -o compile compile.cc -Ofast -std=c++20 -s -flto -march=native -I../../include` ([source](https://github.com/tearosccebe/fast_io/blob/master/examples/0000.compile/compile.cc#L6))

I'll just benchmark both, I guess. The second one is probably faster if I had to guess.

**NOTE:** you'll need to get the [`fast_io`](https://github.com/tearosccebe/fast_io) library to link against, and point the `-I` option to the `fast_io` "`include`" directory.

#### Rust

`main.rs`

```rust
use std::io;
use std::io::Write;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for _i in 0..10_000_000_000u64 {
        stdout.write("a".as_bytes()).unwrap();
    }
}
```

`Cargo.toml`

```
[package]
name = "rust_print"
version = "0.1.0"
edition = "2018"

[profile.release]
panic = "abort"
lto = "fat"

[dependencies]
```

Compiling:

`RUSTFLAGS="-C target-cpu=native" cargo build --release`

## Running the benchmarks:

**NOTE:** I ran each benchmark three times in a row, and took the best result for each. Like I said, I couldn't be bothered to set up a "proper" benchmark for C++. Rust has some nice ones like `criterion` that are super easy to use, though :)

#### `fast_io`, compile command [1]

```
$ cd cpp/
$ g++ -o cpp_print main.cpp -Ofast -std=c++20 -s -I ../../fast_io/include/
$ time ./cpp_print > /dev/null

real    0m47.939s
user    0m47.643s
sys     0m0.267s
```

#### `fast_io`, compile command [2]

```
$ cd cpp/
$ g++ -o cpp_print main.cpp -Ofast -std=c++20 -s -flto -march=native -I ../../fast_io/include/
$ time ./cpp_print > /dev/null

real    0m48.605s
user    0m48.226s
sys     0m0.327s
```

Huh, looks like the extra flags didn't help much after all. Benchmarking FTW. :)

#### Rust

```
$ cd rust/
$ RUSTFLAGS="-C target-cpu=native" cargo build --release
$ time ./target/release/rust_print > /dev/null

real    0m43.948s
user    0m42.747s
sys     0m1.169s
```

## Results

| benchmark             | fast_io [1] | fast_io [2] | rust    |
| --------------------- | ----------- | ----------- | ------- |
| print 10 billion "a"s | 47.939s     | 48.605s     | 43.948s |

Rust looks like it runs in **~91.67%** the time fast_io does. Not bad for [such a bad language](https://www.youtube.com/watch?v=ksTyCQwHGro). :)

Oh, and I didn't need an external dependency either!
