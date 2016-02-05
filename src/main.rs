#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cryptopals;
extern crate crypto;

fn main() {
    cryptopals::set1::c1::main();
    cryptopals::set1::c2::main();
    cryptopals::set1::c3::main();
    cryptopals::set1::c4::main();
    cryptopals::set1::c5::main();
    cryptopals::set1::c6::main();
    cryptopals::set1::c7::main();
}
