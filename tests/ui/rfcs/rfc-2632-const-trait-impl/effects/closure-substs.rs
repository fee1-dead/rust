// check-pass

#![feature(effects, const_trait_impl)]
#![feature(
    no_core,
    lang_items,
    unboxed_closures,
)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "tuple_trait"]
pub trait Tuple {}

#[lang = "receiver"]
pub trait Receiver {}

impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}

#[lang = "copy"]
pub unsafe trait Copy {}

#[lang = "fn_once"]
#[rustc_paren_sugar]
#[const_trait]
pub trait FnOnce<Args: Tuple> {
    #[lang = "fn_once_output"]
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
#[rustc_paren_sugar]
#[const_trait]
pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub(crate) struct NeverShortCircuit<T>(pub T);

impl<T> NeverShortCircuit<T> {
    #[inline]
    pub fn wrap_mut_1<A>(mut f: impl FnMut(A) -> T) -> impl FnMut(A) -> NeverShortCircuit<T> {
        move |a| NeverShortCircuit(f(a))
    }
}

fn main() {}
