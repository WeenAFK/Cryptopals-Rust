#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cryptopals;

mod set1_1;
mod set1_2;
mod set1_3;
mod set1_4;
mod set1_5;
mod set1_6;

fn main() {
    cryptopals::set1::c1::main();
    cryptopals::set1::c2::main();
}
