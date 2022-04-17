// compile-flags: --edition 2018

pub fn demo() -> Option<i32> {
    k#yeet //~ ERROR `k#yeet` expression is experimental
}

pub fn main() -> Result<(), String> {
    k#yeet "hello"; //~ ERROR `k#yeet` expression is experimental
}
