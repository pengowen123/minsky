#![recursion_limit = "128"]
#![allow(dead_code)]

#[macro_use]
extern crate minsky;

use minsky::{Cons, RunMachine};
use minsky::instruction::*;
use minsky::numbers::*;

mod subtract {
    use super::*;

    type Registers = list![P100, P2];
    type Instructions = list![
        Decrement<P1, P3, P1>,
        Decrement<P0, P2, P0>,
        Increment<P1, P3>,
        Halt
    ];

    pub type Out = RunMachine<Instructions, Registers>;
}

mod infinite_loop {
    use super::*;

    type Registers = list![P0, P0];

    type Instructions = list![
        Increment<P0, P0>
    ];

    pub type Out = RunMachine<Instructions, Registers>;
}

fn main() {
    let output: subtract::Out = Default::default();

    println!("{:?}", output);
}
