//! A Minsky machine implemented at the type-level.

use std::marker::PhantomData;
use std::fmt;

pub mod peano;
#[macro_use]
pub mod cons;
pub mod append;
pub mod rot_left;
pub mod rot_left_by;
pub mod get_first;
pub mod index;
pub mod remove_last;
pub mod get_last;
pub mod rot_right;
pub mod rot_right_by;
pub mod inc_head;
pub mod index_inc;
pub mod dec_head;
pub mod index_dec;
pub mod instruction;
pub mod dec_ins;
pub mod execute;
pub mod machine;
pub mod output;

pub use peano::*;
pub use cons::*;
pub use append::*;
pub use rot_left::*;
pub use rot_left_by::*;
pub use get_first::*;
pub use index::*;
pub use remove_last::*;
pub use get_last::*;
pub use rot_right::*;
pub use rot_right_by::*;
pub use inc_head::*;
pub use index_inc::*;
pub use dec_head::*;
pub use index_dec::*;
pub use instruction::*;
pub use dec_ins::*;
pub use execute::*;
pub use machine::*;
pub use output::*;

pub type RunMachine<I, R> =
    <Machine<I, R, Zero, Nil> as Cycle>::Next;

pub mod numbers {
    pub use peano::*;

    pub type P0 = Zero;
    pub type P1 = Succ<P0>;
    pub type P2 = Succ<P1>;
    pub type P3 = Succ<P2>;
    pub type P4 = Succ<P3>;
    pub type P5 = Succ<P4>;
    pub type P6 = Succ<P5>;
    pub type P7 = Succ<P6>;
    pub type P8 = Succ<P7>;
    pub type P9 = Succ<P8>;
    pub type P10 = Succ<P9>;
    pub type P11 = Succ<P10>;
    pub type P12 = Succ<P11>;
    pub type P13 = Succ<P12>;
    pub type P14 = Succ<P13>;
    pub type P15 = Succ<P14>;
    pub type P16 = Succ<P15>;
    pub type P17 = Succ<P16>;
    pub type P18 = Succ<P17>;
    pub type P19 = Succ<P18>;
    pub type P20 = Succ<P19>;
    pub type P21 = Succ<P20>;
    pub type P22 = Succ<P21>;
    pub type P23 = Succ<P22>;
    pub type P24 = Succ<P23>;
    pub type P25 = Succ<P24>;
    pub type P26 = Succ<P25>;
    pub type P27 = Succ<P26>;
    pub type P28 = Succ<P27>;
    pub type P29 = Succ<P28>;
    pub type P30 = Succ<P29>;
    pub type P31 = Succ<P30>;
    pub type P32 = Succ<P31>;
    pub type P33 = Succ<P32>;
    pub type P34 = Succ<P33>;
    pub type P35 = Succ<P34>;
    pub type P36 = Succ<P35>;
    pub type P37 = Succ<P36>;
    pub type P38 = Succ<P37>;
    pub type P39 = Succ<P38>;
    pub type P40 = Succ<P39>;
    pub type P41 = Succ<P40>;
    pub type P42 = Succ<P41>;
    pub type P43 = Succ<P42>;
    pub type P44 = Succ<P43>;
    pub type P45 = Succ<P44>;
    pub type P46 = Succ<P45>;
    pub type P47 = Succ<P46>;
    pub type P48 = Succ<P47>;
    pub type P49 = Succ<P48>;
    pub type P50 = Succ<P49>;
    pub type P51 = Succ<P50>;
    pub type P52 = Succ<P51>;
    pub type P53 = Succ<P52>;
    pub type P54 = Succ<P53>;
    pub type P55 = Succ<P54>;
    pub type P56 = Succ<P55>;
    pub type P57 = Succ<P56>;
    pub type P58 = Succ<P57>;
    pub type P59 = Succ<P58>;
    pub type P60 = Succ<P59>;
    pub type P61 = Succ<P60>;
    pub type P62 = Succ<P61>;
    pub type P63 = Succ<P62>;
    pub type P64 = Succ<P63>;
    pub type P65 = Succ<P64>;
    pub type P66 = Succ<P65>;
    pub type P67 = Succ<P66>;
    pub type P68 = Succ<P67>;
    pub type P69 = Succ<P68>;
    pub type P70 = Succ<P69>;
    pub type P71 = Succ<P70>;
    pub type P72 = Succ<P71>;
    pub type P73 = Succ<P72>;
    pub type P74 = Succ<P73>;
    pub type P75 = Succ<P74>;
    pub type P76 = Succ<P75>;
    pub type P77 = Succ<P76>;
    pub type P78 = Succ<P77>;
    pub type P79 = Succ<P78>;
    pub type P80 = Succ<P79>;
    pub type P81 = Succ<P80>;
    pub type P82 = Succ<P81>;
    pub type P83 = Succ<P82>;
    pub type P84 = Succ<P83>;
    pub type P85 = Succ<P84>;
    pub type P86 = Succ<P85>;
    pub type P87 = Succ<P86>;
    pub type P88 = Succ<P87>;
    pub type P89 = Succ<P88>;
    pub type P90 = Succ<P89>;
    pub type P91 = Succ<P90>;
    pub type P92 = Succ<P91>;
    pub type P93 = Succ<P92>;
    pub type P94 = Succ<P93>;
    pub type P95 = Succ<P94>;
    pub type P96 = Succ<P95>;
    pub type P97 = Succ<P96>;
    pub type P98 = Succ<P97>;
    pub type P99 = Succ<P98>;
    pub type P100 = Succ<P99>;
}