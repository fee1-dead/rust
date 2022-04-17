// pp-exact
#![feature(yeet_expr)]

fn yeet_no_expr() -> Option<String> { k#yeet }

fn yeet_no_expr_with_semicolon() -> Option<String> { k#yeet; }

fn yeet_with_expr() -> Result<String, i32> { k#yeet 1 + 2 }

fn yeet_with_expr_with_semicolon() -> Result<String, i32> { k#yeet 1 + 2; }

fn main() {}
