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
