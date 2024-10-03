use crate::asm::{Instruction, Asm};

pub(crate) trait Optimize {
    type Item;

    fn optimize<I: Iterator<Item = Self::Item>>(data: &mut impl Iterator<Item = Self::Item>) -> T;
}

// struct AsmOptimizer {
//     buf: Vec<Asm>,
// }

// impl AsmOptimizer {

// }