use std::io;
use std::io::Write;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for _i in 0..10_000_000_000u64 {
        stdout.write("a".as_bytes()).unwrap();
    }
}
