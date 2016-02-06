#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cryptopals;
extern crate crypto;

const SET_1: bool = false;
const SET_2: bool = true;

fn main() {
    do_set_1();
    do_set_2();
}

fn do_set_1() {
    if SET_1 {
        use cryptopals::set1 as s;
        s::c1::main();
        s::c2::main();
        s::c3::main();
        s::c4::main();
        s::c5::main();
        s::c6::main();
        s::c7::main();
        s::c8::main();
    }
}

fn do_set_2() {
    if SET_2 {
        use cryptopals::set2 as s;
        s::c1::main();
        s::c2::main();
        s::c3::main();
        s::c4::main();
        s::c5::main();
        s::c6::main();
        s::c7::main();
        s::c8::main();
    }
}
