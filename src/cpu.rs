use std::ops::{Index, RangeInclusive};

use anyhow::{anyhow, Result};

use crate::{
    asm::*,
    io::ScreenUpdate,
    //code_writer::assembler::{Comp, Instruction},
    vm::{MemSegment as Seg, VmCommand},
};

const KBD: i16 = 0x6000;
const SCREEN_START: i16 = 0x4000;
const SCREEN_END: i16 = 0x5FFF;
const SCREEN: RangeInclusive<i16> = 0x4000..=0x5FFF;
const LCL: i16 = 1;

/// The address of the current frame's `argument` memory segment is stored at address 2.
const ARG: i16 = 2;

/// The address of the current frame's `this` memory segment is stored at address 3.
///
/// This is `pointer 0` in the VM abstraction.
const THIS: i16 = 3;

/// The address of the current frame's `that` memory segment is stored at address 4.
///
/// This is `pointer 1` in the VM abstraction.
const THAT: i16 = 4;

struct Ram([i16; 0xFFFF]);

impl Index<i16> for Ram {
    type Output = i16;

    fn index(&self, index: i16) -> &Self::Output {
        todo!()
    }
}

pub struct Cpu<'a> {
    pub ram: [i16; 0xFFFF],
    rom: &'a [Instruction],
    pub(crate) pc: usize,
    d: i16,
    a: i16,
}

#[allow(overflowing_literals)]
impl<'a> Cpu<'a> {
    pub fn new(asm: &'a [Instruction]) -> Self {
        Self {
            ram: [0; 0xFFFF],
            rom: asm,
            pc: 0,
            d: 0,
            a: 0,
        }
    }

    pub const fn get_pixel(&self, x: usize, y: usize) -> bool {
        let offset = x & 15;
        let addr = SCREEN_START as usize + (x / 16) + (y * 32);
        self.ram[addr] & (1 << (15 - offset)) != 0
    }

    const fn m(&self) -> i16 {
        self.ram[self.a as u16 as usize]
    }

    fn m_mut(&mut self) -> &mut i16 {
        &mut self.ram[self.a as u16 as usize]
    }

    const fn at(&self, addr: i16) -> i16 {
        self.ram[addr as usize]
    }

    fn at_mut(&mut self, addr: i16) -> &mut i16 {
        &mut self.ram[addr as u16 as usize]
    }

    const fn a_comp(&self, mode: Mode) -> i16 {
        match mode {
            Mode::A => self.a,
            Mode::M => self.m(),
        }
    }

    fn sp(&mut self) -> &mut i16 {
        self.at_mut(0)
    }

    fn stack_top(&mut self) -> &mut i16 {
        let sp = *self.sp();
        self.at_mut(sp)
    }

    pub fn set_kbd(&mut self, kbd: i16) {
        self.ram[KBD as usize] = kbd;
    }

    const fn get_comp(&self, comp: CompBits) -> i16 {
        let a_comp = match comp.mode() {
            Mode::A => self.a,
            Mode::M => self.m(),
        };

        match comp.c_bits() {
            CBits::Zero => 0,
            CBits::One => 1,
            CBits::NegOne => -1,
            CBits::D => self.d,
            CBits::A => a_comp,
            CBits::NotD => !self.d,
            CBits::NotA => !a_comp,
            CBits::NegD => -self.d,
            CBits::NegA => -a_comp,
            CBits::DPlusOne => self.d.wrapping_add(1),
            CBits::APlusOne => a_comp.wrapping_add(1),
            CBits::DMinusOne => self.d.wrapping_sub(1),
            CBits::AMinusOne => a_comp.wrapping_sub(1),
            CBits::DPlusA => self.d.wrapping_add(a_comp),
            CBits::DMinusA => self.d.wrapping_sub(a_comp),
            CBits::AMinusD => a_comp.wrapping_sub(self.d),
            CBits::DAndA => self.d & a_comp,
            CBits::DOrA => self.d | a_comp,
            // Unofficial
            CBits::Zero0
            | CBits::Zero1
            | CBits::Zero2
            | CBits::Zero3
            | CBits::Zero4
            | CBits::Zero5
            | CBits::Zero6
            | CBits::Zero7
            | CBits::Zero8
            | CBits::Zero9 => 0,
            CBits::NegOne0
            | CBits::NegOne1
            | CBits::NegOne2
            | CBits::NegOne3
            | CBits::NegOne4
            | CBits::NegOne5
            | CBits::NegOne6
            | CBits::NegOne7
            | CBits::NegOne8
            | CBits::NegOne9 => -1,
            CBits::D0 | CBits::D1 | CBits::D2 => self.d,
            CBits::A0 | CBits::A1 | CBits::A2 => self.a,
            CBits::NotD0 | CBits::NotD1 | CBits::NotD2 => !self.d,
            CBits::NotA0 | CBits::NotA1 | CBits::NotA2 => !self.a,
            CBits::DNandA => !self.d | !self.a,
            CBits::NotOfDPlusA => !(self.d.wrapping_add(self.a)),
            CBits::DAndNotA => self.d & !self.a,
            CBits::NotDOrA => !self.d | self.a,
            CBits::DPlusNotA => self.d.wrapping_add(!self.a),
            CBits::NotDAndA => !self.d & self.a,
            CBits::DOrNotA => self.d | !self.a,
            CBits::NotDPlusA => (!self.d).wrapping_add(self.a),
            CBits::DNorA => !self.d & !self.a,
            CBits::NotDPlusNotA => (!self.d).wrapping_add(!self.a),
            CBits::NotDMinus1 => (!self.d).wrapping_sub(1),
            CBits::NotNotDPlusNotA => !((!self.d).wrapping_add(!self.a)),
            CBits::NotAMinus1 => (!self.a).wrapping_sub(1),
            CBits::NegTwo => -2,
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        use InstructionType as Inst;
        let inst = self.rom[self.pc];
        self.pc += 1;
        match inst.get().map_err(|i| anyhow!("{i} is not a valid instruction"))? {
            // an address will always be an unsigned 15 bit integer, so can never overflow an i16.
            Inst::A(addr) => {
                self.a = addr.value() as i16;
            }
            Inst::C(c) => {
                // get a reference to the A or M register
                let comp = self.get_comp(c.comp());

                // Calculate jump before updating registers from destination
                // Not doing this is the cause of the official CPU emulator bug
                if (c.jump() == Jump::JMP)
                    || (c.jeq() && comp == 0)
                    || (c.jgt() && comp > 0)
                    || (c.jlt() && comp < 0)
                {
                    self.pc = self.a as usize;
                }
                
                // Do not allow writing to the KBD register
                // Handle the M destination first to avoid writing to the wrong address.
                if c.dest().m() && self.a != KBD {
                    *self.m_mut() = comp;
                }

                // The other destination bits are more permissive
                if c.dest().a() {
                    self.a = comp;
                }
                if c.dest().d() {
                    self.d = comp;
                }
            }
        }
        Ok(())
    }

    /// Sets the D register to the current stack top, and decrements the stack pointer
    ///
    /// A way to directly simulate the VM Pop command at the CPU level
    /// without generating or executing every hack instruction individually
    fn pop(&mut self) {
        self.d = *self.stack_top();
        *self.sp() -= 1;
    }

    /// Emulates the execution a VM command on the CPU level
    ///
    /// Each instruction *should* leave the CPU registers (and hopefully memory) in the same state
    /// as compiling to a series of assembly instructions and executing each in turn.
    fn execute_vm(&mut self, vm: VmCommand) {
        match vm {
            VmCommand::Add => {
                self.pop();
                *self.stack_top() += self.d;
            }
            VmCommand::Sub => {
                self.pop();
                *self.stack_top() -= self.d;
            }
            VmCommand::Neg => *self.stack_top() = -*self.stack_top(),
            VmCommand::Compare(_) => todo!(),
            VmCommand::And => {
                self.pop();
                *self.stack_top() &= self.d;
            }
            VmCommand::Or => {
                self.pop();
                *self.stack_top() |= self.d;
            }
            VmCommand::Not => *self.stack_top() = !*self.stack_top(),
            VmCommand::Push(_, _) => todo!(),
            VmCommand::Pop(seg, i) => {
                self.pop();
                let addr = match seg {
                    Seg::Argument => self.at(ARG).wrapping_add(i),
                    Seg::Local => self.at(LCL).wrapping_add(i),
                    Seg::Static => 16i16.wrapping_add(i),
                    Seg::This => self.at(THIS).wrapping_add(i),
                    Seg::That => self.at(THAT).wrapping_add(i),
                    Seg::Pointer => THIS.wrapping_add(i),
                    Seg::Temp => 5i16.wrapping_add(i),
                    // Cannot pop to constants
                    Seg::Constant => unreachable!(),
                };
                *self.at_mut(addr) = self.d;
            }
            VmCommand::Label(_) => todo!(),
            VmCommand::Goto(_) => todo!(),
            VmCommand::IfGoto(_) => todo!(),
            VmCommand::Function(_, _) => todo!(),
            VmCommand::Call(fun, n) => {
                *self.sp() += 5;
                let sp = *self.sp() as usize;
                //&mut self.ram.copy_within();
            },
            VmCommand::Return => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {}
