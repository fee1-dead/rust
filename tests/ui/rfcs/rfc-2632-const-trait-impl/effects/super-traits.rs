// check-pass

#![feature(const_trait_impl, effects)]

#[const_trait]
pub trait X<Rhs: ?Sized = Self> {}

#[const_trait]
pub trait Y: X {}

impl X for () {}

impl Y for () {}

fn main() {}
