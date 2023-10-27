use std::ops::RangeInclusive;

use anyhow::{bail, Result};
use sdl2::render::{Texture, Canvas, RenderTarget};

use crate::{
    asm::*,
    io::Screen,
    //code_writer::assembler::{Comp, Instruction},
    vm::{MemSegment as Seg, VmCommand},
};

const KBD: i16 = 0x6000;
const SCREEN_START: i16 = 0x4000;
const SCREEN_END: i16 = 0x5FFF;
const SCREEN: RangeInclusive<i16> = 0x4000..=0x5FFF;
const LCL: i16 = 1;

/// The address of the current frame's `argument` memory segment is stored at address 2.
pub const ARG: i16 = 2;

/// The address of the current frame's `this` memory segment is stored at address 3.
///
/// This is `pointer 0` in the VM abstraction.
pub const THIS: i16 = 3;

/// The address of the current frame's `that` memory segment is stored at address 4.
///
/// This is `pointer 1` in the VM abstraction.
pub const THAT: i16 = 4;

pub struct Cpu {
    pub ram: [i16; 0xFFFF],
    rom: [Instruction; 0xFFFF],
    screen: Screen,
    pub(crate) pc: usize,
    d: i16,
    a: i16,
    pub screen_changed: bool,
}

#[allow(overflowing_literals)]
impl Cpu {
    pub fn new() -> Self {
        Self {
            ram: [0; 0xFFFF],
            rom: [Instruction::from(0); 0xFFFF],
            screen: Screen::new(),
            pc: 0,
            d: 0,
            a: 0,
            screen_changed: false,
        }
    }

    pub const fn screen_changed(&self) -> bool {
        self.screen.changed()
    }

    pub fn refresh(&mut self, texture: &mut Texture) {
        for (i, word) in self.ram[0x4000..0x6000].iter().enumerate() {
            self.screen.update(i, *word);
        }
        self.screen.refresh(texture)
    }

    // pub fn refresh<R: RenderTarget>(&mut self, canvas: &mut Canvas<R>) {
    //     for i in 0x4000..0x6000 {

    //     }
    // }

    pub fn render<R: RenderTarget>(&self, canvas: &mut Canvas<R>) {
        Screen::render(&self.ram[0x4000..0x6000], canvas);
    }

    pub const fn screen(&self) -> &[u8] {
        &self.screen.data
    }

    #[inline]
    const fn m(&self) -> i16 {
        self.ram[self.a as u16 as usize]
    }

    #[inline]
    fn m_mut(&mut self) -> &mut i16 {
        &mut self.ram[self.a as u16 as usize]
    }

    #[inline]
    const fn at(&self, addr: i16) -> i16 {
        self.ram[addr as usize]
    }

    #[inline]
    fn at_mut(&mut self, addr: i16) -> &mut i16 {
        &mut self.ram[addr as u16 as usize]
    }

    #[inline]
    const fn a_comp(&self, mode: Mode) -> i16 {
        match mode {
            Mode::A => self.a,
            Mode::M => self.m(),
        }
    }

    #[inline]
    fn sp(&mut self) -> &mut i16 {
        self.at_mut(0)
    }

    #[inline]
    fn stack_top(&mut self) -> &mut i16 {
        let sp = *self.sp();
        self.at_mut(sp)
    }

    pub fn set_kbd(&mut self, kbd: i16) {
        self.ram[KBD as usize] = kbd;
    }

    pub fn tick(&mut self, asm: &[Instruction]) -> Result<()> {
        use InstructionType as Inst;
        //println!("{}", self.ram[KBD as usize]);
        let inst = asm[self.pc];
        //println!("Start: A: {} D: {} M: {} PC: {}", self.a, self.d, self.m(), self.pc);
        //println!("{inst}");
        self.pc += 1;
        match inst.get()? {
            // an address will always be an unsigned 15 bit integer, so can never overflow an i16.
            Inst::A(addr) => {
                self.a = addr.value() as i16;
            }
            Inst::C(c) => {
                // get a reference to the A or M register
                let a_comp = self.a_comp(c.comp().mode());
                let comp = match c.comp().c_bits() {
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
                    _ => bail!("Invalid C Bits"),
                };

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
                if c.dest().m()
                && self.a != KBD
                && self.m() != comp {
                    // Check to see if the A register is pointing to an address in the screen
                    if SCREEN.contains(&self.a) {
                        self.screen_changed = true;
                        //if let _a @ SCREEN_START..=SCREEN_END = self.a {
                        // if self.at(self.a) != comp {
                        //     self.screen.update(self.a, comp);
                        // }
                        //println!("{}={}", self.a, comp);
                        //.expect("failed to update screen");
                    }
                    *self.m_mut() = comp;
                }

                // The other destination bits are more permissive
                if c.dest().a()
                && self.a != comp {
                    self.a = comp;
                }
                if c.dest().d()
                && self.d != comp {
                    self.d = comp;
                }
            }
        }
        //println!("End: A: {} D: {} M: {} PC: {}", self.a, self.d, self.m(), self.pc);
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
                    Seg::Argument => self.at(ARG) + i,
                    Seg::Local => self.at(LCL) + i,
                    Seg::Static => 16 + i,
                    Seg::This => self.at(THIS) + i,
                    Seg::That => self.at(THAT) + i,
                    Seg::Pointer => THIS + i,
                    Seg::Temp => 5 + i,
                    // Cannot pop to constants
                    Seg::Constant => unreachable!(),
                };
                *self.at_mut(addr) = self.d;
            }
            VmCommand::Label(_) => todo!(),
            VmCommand::Goto(_) => todo!(),
            VmCommand::IfGoto(_) => todo!(),
            VmCommand::Function(_, _) => todo!(),
            VmCommand::Call(_, _) => todo!(),
            VmCommand::Return => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {}
