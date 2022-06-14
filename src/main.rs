#![feature(yeet_expr)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_yeet)]

use core::ops::{Yeet,FromResidual};

#[derive(Debug)]
enum LolCatErr {
    Lmao
}

impl FromResidual<Yeet<LolCatErr>> for LolCatErr {
    fn from_residual(_: Yeet<LolCatErr>) -> Self {
        LolCatErr::Lmao
    }

}

/// # trying yeet in rust nightly
/// Seeing if it really works from:
///     [This is why Rust is better than C++](https://youtu.be/LpKm2GjIf5Y)
///
/// ```sh
/// rustup default nightly
/// cargo run
///    Compiling yeet-expr v0.1.0
///     Finished dev [unoptimized + debuginfo] target(s) in 0.40s
///      Running `target/debug/yeet-expr`
/// Error: Lmao
/// ```
///
/// ## links
///
/// 1. [rust-lang/rust #96374](https://github.com/rust-lang/rust/issues/96374)
/// 2. [rust unstable book](https://doc.rust-lang.org/beta/unstable-book/language-features/yeet-expr.html)
/// 3. [try trait v2](https://rust-lang.github.io/rfcs/3058-try-trait-v2.html)
/// 4. [tracking issue for `ops::Yeet`](https://github.com/rust-lang/rust/issues/96374)
///
/// -----
///

fn main() -> Result<(), LolCatErr> {

    fn foo() -> Result<String, i32> {
        do yeet 4;
    }

    fn bar() -> Option<String> {
        do yeet;
    }

    assert_eq!(foo(), Err(4));
    assert_eq!(bar(), None);

    do yeet LolCatErr::Lmao
}
