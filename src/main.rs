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
    cryptopals::set1::p1::main();
    //set1_1::main();
    //set1_2::main();
    //set1_3::main();
    //set1_4::main();
    //set1_5::main();
    //set1_6::main();
}
