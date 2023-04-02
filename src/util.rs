use crate::types::Word;
use std::{borrow::Cow, time::Instant};

#[allow(unused)]
pub fn st(w: &Word) -> Cow<'_, str> {
    String::from_utf8_lossy(w)
}

pub fn bench<F: Fn() -> ()>(title: &str, f: F) {
    let start = Instant::now();
    f();
    let elapsed = Instant::elapsed(&start);
    println!("[{title}] runtime: {:?}", elapsed);
}
