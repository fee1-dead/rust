// run-pass

#![feature(yeet_expr)]

fn always_yeet() -> Option<String> {
    k#yeet;
}

fn main() {
    assert_eq!(always_yeet(), None);
}
