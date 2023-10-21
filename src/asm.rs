use anyhow::{bail, Result};
use arbitrary_int::{u15, u3, u7};
use bitbybit::{bitenum, bitfield};
use std::borrow::Cow;
use std::collections::HashMap;

#[bitenum(u3, exhaustive: true)]
#[derive(Debug, PartialEq)]
/// The destination bits of a Hack C-Instruction.
///
/// Each letter corresponds to a register that the result of the computation segment will be placed into.
pub enum Dest {
    /// The value computed by the ALU in the `comp` segment will not be stored in a register
    None = 0,
    /// The value computed by the ALU in the `comp` segment will be stored in the `M` register
    M = 1,
    /// The value computed by the ALU in the `comp` segment will be stored in the `D` register
    D = 2,
    /// The value computed by the ALU in the `comp` segment will be stored in `M` and `D` registers
    MD = 3,
    /// The value computed by the ALU in the `comp` segment will be stored in the `A` register
    A = 4,
    /// The value computed by the ALU in the `comp` segment will be stored in the `A` and `M` registers
    AM = 5,
    /// The value computed by the ALU in the `comp` segment will be stored in the `A` and `D` registers
    AD = 6,
    /// The value computed by the ALU in the `comp` segment will be stored in the `A`, `M`, and `D` registers
    AMD = 7,
}

#[bitfield(u3, default: 0)]
pub struct DestBits {
    #[bits(0..=2, rw)]
    get: Dest,

    /// Bit 3 of a C-Instruction
    ///
    /// If this flag is set, the value computed by `comp` will be stored in register `M`
    #[bit(0, rw)]
    m: bool,

    /// Bit 4 of a C-Instruction
    ///
    /// If this flag is set, the value computed by `comp` will be stored in register `D`
    #[bit(1, rw)]
    d: bool,

    /// Bit 5 of a C-Instruction
    ///
    /// If this flag is set, the value computed by `comp` will be stored in register `A`
    #[bit(2, rw)]
    a: bool,
}

impl Dest {
    pub fn from_flags(a: bool, m: bool, d: bool) -> Self {
        DestBits::DEFAULT.with_a(a).with_m(m).with_d(d).get()
    }
}

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::M => write!(f, "M"),
            Self::D => write!(f, "D"),
            Self::MD => write!(f, "MD"),
            Self::A => write!(f, "A"),
            Self::AM => write!(f, "AM"),
            Self::AD => write!(f, "AD"),
            Self::AMD => write!(f, "AMD"),
        }
    }
}

/// The comp segment (bits 6-11) represents the computations performed in a C-Instruction by the Hack ALU
/// 
/// They are named based on the A register in all cases where its value is used in the calculation
/// 
/// Bit 12 is used to determine whether A is read by value (`A`) or as a pointer (`M`/`memory[A]`)
#[rustfmt::skip]
#[bitenum(u7, exhaustive: false)]
pub enum ValidComp {
    /// The configuration of C bits that translates to the comp `0` in the Hack assembly specification.
    Zero        = 0b0101010,
    /// The configuration of C bits that translates to the comp `1` in the Hack assembly specification.
    One         = 0b0111111,
    /// The configuration of C bits that translates to the comp `-1` in the Hack assembly specification.
    NegOne      = 0b0111010,
    /// The configuration of C bits that translates to the comp `D` in the Hack assembly specification.
    D           = 0b0001100,
    /// The configuration of C bits that translates to the comp `A` in the Hack assembly specification.
    A           = 0b0110000,
    /// The configuration of C bits that translates to the comp `M` in the Hack assembly specification.
    M           = 0b1110000,
    /// The configuration of C bits that translates to the comp `!D` in the Hack assembly specification.
    NotD        = 0b0001101,
    /// The configuration of C bits that translates to the comp `!A` in the Hack assembly specification.
    NotA        = 0b0110001,
    /// The configuration of C bits that translates to the comp `!M` in the Hack assembly specification.    
    NotM        = 0b1110001,
    /// The configuration of C bits that translates to the comp `-D` in the Hack assembly specification.
    NegD        = 0b0001111,
    /// The configuration of C bits that translates to the comp `-A` in the Hack assembly specification.
    NegA        = 0b0110011,
    /// The configuration of C bits that translates to the comp `-M` in the Hack assembly specification.
    NegM        = 0b1110011,
    /// The configuration of C bits that translates to the comp `D+1` in the Hack assembly specification.
    DPlusOne    = 0b0011111,
    /// The configuration of C bits that translates to the comp `A+1` in the Hack assembly specification.
    APlusOne    = 0b0110111,
    /// The configuration of C bits that translates to the comp `M+1` in the Hack assembly specification.
    MPlusOne    = 0b1110111,
    /// The configuration of C bits that translates to the comp `D-1` in the Hack assembly specification.
    DMinusOne   = 0b0001110,
    /// The configuration of C bits that translates to the comp `A-1` in the Hack assembly specification.
    AMinusOne   = 0b0110010,
    /// The configuration of C bits that translates to the comp `M-1` in the Hack assembly specification.
    MMinusOne   = 0b1110010,
    /// The configuration of C bits that translates to the comp `D+A` in the Hack assembly specification.
    DPlusA      = 0b0000010,
    /// The configuration of C bits that translates to the comp `D+M` in the Hack assembly specification.
    DPlusM      = 0b1000010,
    /// The configuration of C bits that translates to the comp `D-A` in the Hack assembly specification.
    DMinusA     = 0b0010011,
    /// The configuration of C bits that translates to the comp `D-M` in the Hack assembly specification.
    DMinusM     = 0b1010011,
    /// The configuration of C bits that translates to the comp `A-D` in the Hack assembly specification.
    AMinusD     = 0b0000111,
    /// The configuration of C bits that translates to the comp `M-D` in the Hack assembly specification.
    MMinusD     = 0b1000111,
    /// The configuration of C bits that translates to the comp `D&A` in the Hack assembly specification.
    DAndA       = 0b0000000, // lol
    /// The configuration of C bits that translates to the comp `D&M` in the Hack assembly specification.
    DAndM       = 0b1000000,
    /// The configuration of C bits that translates to the comp `D|A` in the Hack assembly specification.
    DOrA        = 0b0010101,
    /// The configuration of C bits that translates to the comp `D|M` in the Hack assembly specification.
    DOrM        = 0b1010101,
}

#[rustfmt::skip]
#[bitenum(u6, exhaustive: true)]
#[derive(Debug, PartialEq)]
pub enum CBits {
    /********************************************
    *   Official ALU computations
    *
    *********************************************/
    /// The specified C-bit configuration evaluating to `0`
    /// 
    /// `0 + 0`
    Zero            = 0b101010,

    /// The specified C-bit configuration evaluating to `1`
    /// 
    /// `!(-1 + -1)`
    One             = 0b111111,

    /// The specified C-bit configuration evaluating to `-1`
    /// 
    /// `-1 + 0`
    NegOne          = 0b111010,

    /// The specified C-bit configuration evaluating to `D`
    /// 
    /// `D & -1`
    D               = 0b001100,

    /// The specified C-bit configuration evaluating to `A`
    /// 
    /// `-1 & A`
    A               = 0b110000,

    /// The specified C-bit configuration evaluating to `!D`
    /// 
    /// `!(D & -1)`
    NotD            = 0b001101,

    /// The specified C-bit configuration evaluating to `A`
    /// 
    /// `!(-1 & A)`
    NotA            = 0b110001,

    /// The specified C-bit configuration evaluating to `-D`
    /// 
    /// `!(D + -1)`
    NegD            = 0b001111,

    /// The specified C-bit configuration evaluating to `-A`
    /// 
    /// `!(-1 + A)`
    NegA            = 0b110011,

    /// The specified C-bit configuration evaluating to `D + 1`
    /// 
    /// `!(!D + -1)`
    DPlusOne        = 0b011111,

    /// The specified C-bit configuration evaluating to `A + 1`
    /// 
    /// `!(-1 + !A)`
    APlusOne        = 0b110111,

    /// The specified C-bit configuration evaluating to `D - 1`
    /// 
    /// `D + -1`
    DMinusOne       = 0b001110,

    /// The specified C-bit configuration evaluating to `A - 1`
    /// 
    /// `-1 + A`
    AMinusOne       = 0b110010,

    /// The specified C-bit configuration evaluating to `D + A`
    /// 
    /// `D + A`
    DPlusA          = 0b000010,

    /// The specified C-bit configuration evaluating to `D - A`
    /// 
    /// `!(!D + A)`
    DMinusA         = 0b010011,

    /// The specified C-bit configuration evaluating to `A - D`
    /// 
    /// `!(D + !A)`
    AMinusD         = 0b000111,

    /// The specified C-bit configuration evaluating to `D & A`
    /// 
    /// `D & A`
    DAndA           = 0b000000, // lol

    /// The specified C-bit configuration evaluating to `D | A`
    /// 
    /// `!(!D & !A)`
    DOrA            = 0b010101,

    /********************************************
    *   Unofficial (duplicate) computations
    *
    *********************************************/

    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `!(-1 & -1)`
    Zero0           = 0b111101,

    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `!(-1 + 0)`
    Zero1           = 0b111011,

    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `D & 0`
    Zero2           = 0b001000,

    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `!D & 0`
    Zero3           = 0b011000,

    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `0 & A`
    Zero4           = 0b100000,

    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `0 & !A`
    Zero5           = 0b100100,
    
    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `0 & 0`
    Zero6           = 0b101000,
    
    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `0 & -1`
    Zero7           = 0b101100,
    
    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `!(0 + -1)`
    Zero8           = 0b101111,
    
    /// An unspecified C-bit configuration evaluating to `0`
    /// 
    /// `-1 & 0`
    Zero9           = 0b111000,

    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne0         = 0b001001, // !(D & 0)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne1         = 0b011001, // !(!D & 0)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne2         = 0b100001, // !(0 & A)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne3         = 0b100101, // !(0 & !A)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne4         = 0b101001, // !(0 & 0)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne5         = 0b101011, // !(0 + 0)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne6         = 0b101101, // !(0 & -1)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne7         = 0b101110, // 0 + -1
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne8         = 0b111001, // !(-1 & 0)
    /// An unspecified C-bit configuration evaluating to `-1`
    NegOne9         = 0b111100, // -1 & -1
    
    /// An unspecified C-bit configuration evaluating to `D`
    D0              = 0b001010, // D + 0
    /// An unspecified C-bit configuration evaluating to `D`
    D1              = 0b011011, // !(!D + 0)
    /// An unspecified C-bit configuration evaluating to `D`
    D2              = 0b011101, // !(!D & -1)

    /// An unspecified C-bit configuration evaluating to `A`
    A0              = 0b100010, // 0 + A
    /// An unspecified C-bit configuration evaluating to `A`
    A1              = 0b100111, // !(0 + !A)
    /// An unspecified C-bit configuration evaluating to `A`
    A2              = 0b110101, // !(-1 & !A)
    // !D
    /// An unspecified C-bit configuration evaluating to `!D`
    NotD0           = 0b001011, // !(D + 0)

    /// An unspecified C-bit configuration evaluating to `!D`
    /// 
    /// `!D & -1`
    NotD1           = 0b011100,

    /// An unspecified C-bit configuration evaluating to `!D`
    /// 
    /// `!D + 0`
    NotD2           = 0b011010,

    // !A
    /// An unspecified C-bit configuration evaluating to `!A`:
    /// 
    /// `!(0 + A)`
    NotA0           = 0b100011,

    /// An unspecified C-bit configuration evaluating to `!A`
    /// 
    /// `0 + !A`
    NotA1           = 0b100110,

    /// An unspecified C-bit configuration evaluating to `!A`
    /// 
    /// `-1 & !A`
    NotA2           = 0b110100, 

    /********************************************
    *   The dark corners of the ALU
    *
    *********************************************/

    /// An unspecified C-bit configuration evaluating to `!D|!A`
    /// 
    /// `!(D & A)`
    NotDOrNotA      = 0b000001,

    /// An unspecified C-bit configuration evaluating to `!(D + A)`
    /// 
    /// `!(D + A)`
    NotOfDPlusA     = 0b000011,

    /// An unspecified C-bit configuration evaluating to `D & !A`
    /// 
    /// `D & !A`
    DAndNotA        = 0b000100,

    /// An unspecified C-bit configuration evaluating to `!D | A`
    /// 
    /// `!(D & !A)`
    NotDOrA         = 0b000101,

    /// An unspecified C-bit configuration evaluating to `D + !A`
    /// 
    /// `D + !A`
    DPlusNotA       = 0b000110,

    /// An unspecified C-bit configuration evaluating to `!D & A`
    /// 
    /// `D & A`
    NotDAndA        = 0b010000,

    /// An unspecified C-bit configuration evaluating to `D | !A`
    /// 
    /// `!(!D & A)`
    DOrNotA         = 0b010001,

    /// An unspecified C-bit configuration evaluating to `!D + A`
    /// 
    /// `!D + A`
    NotDPlusA       = 0b010010,

    /// An unspecified C-bit configuration evaluating to `!D & !A`
    /// 
    /// `!D & !A`
    NotDAndNotA     = 0b010100,

    /// An unspecified C-bit configuration evaluating to `!D + !A`
    /// 
    /// `!D + !A`
    NotDPlusNotA    = 0b010110,

    /// An unspecified C-bit configuration evaluating to `!D - 1`
    /// 
    /// `!D + -1`
    NotDMinus1      = 0b011110,

    /// An unspecified C-bit configuration evaluating to `!(!D + !A)`
    /// 
    /// `!(!D + !A)`
    NotNotDPlusNotA = 0b010111,

    /// An unspecified C-bit configuration evaluating to `!A - 1`
    /// 
    /// `-1 + !A`
    NotAMinus1      = 0b110110,

    /// An unspecified C-bit configuration evaluating to `-2` (or `!1`)
    /// 
    /// `-1 + -1`
    NegTwo          = 0b111110,     // Single instruction -2 constant!!!
}

/// This bit controls the input to the Hack ALU.
///
/// If not set, the value in the `A` register is passed in as the second input.
///
/// If set, the value at `Memory[A]` is passed in instead.
#[bitenum(u1, exhaustive: true)]
#[derive(Debug, PartialEq)]
pub enum Mode {
    A = 0,
    M = 1,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::A => write!(f, "A"),
            Mode::M => write!(f, "M"),
        }
    }
}

/// The combination of the address bit and C-bits, which determines the Computation portion of a C instruction.
///
/// Not all configurations are valid, those that are can be found in the `ValidComp` enum.
/// If the bit configuration is valid, it can be accessed with the `comp()` method.
///
/// However, all bit configurations and their resulting computations are mapped by the `Mode` and `CBits` enums,
/// and are accessible through the `mode()` and `c_bits()` methods.
#[bitfield(u7)]
pub(crate) struct Comp {
    #[bits(0..=6, rw)]
    comp: Option<ValidComp>,
    #[bit(6, rw)]
    mode: Mode,
    #[bits(0..=5, rw)]
    c_bits: CBits,
}

impl Comp {
    pub const fn new_valid(comp: ValidComp) -> Self {
        Self { raw_value: 0 }.with_comp(comp)
    }

    /// If the bits are in an officially specified configuration, returns that computation. Otherwise returns `None`.
    pub const fn get(self) -> Option<ValidComp> {
        match self.comp() {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}

impl std::fmt::Display for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CBits as C;
        match self.c_bits() {
            C::Zero
            | C::Zero0
            | C::Zero1
            | C::Zero2
            | C::Zero3
            | C::Zero4
            | C::Zero5
            | C::Zero6
            | C::Zero7
            | C::Zero8
            | C::Zero9 => write!(f, "0"),
            C::One => write!(f, "1"),
            C::NegOne
            | C::NegOne0
            | C::NegOne1
            | C::NegOne2
            | C::NegOne3
            | C::NegOne4
            | C::NegOne5
            | C::NegOne6
            | C::NegOne7
            | C::NegOne8
            | C::NegOne9 => write!(f, "-1"),
            C::D | C::D0 | C::D1 | C::D2 => write!(f, "D"),
            C::A | C::A0 | C::A1 | C::A2 => write!(f, "{}", self.mode()),
            C::NotD | C::NotD0 | C::NotD1 | C::NotD2 => write!(f, "!D"),
            C::NotA | C::NotA0 | C::NotA1 | C::NotA2 => write!(f, "!{}", self.mode()),
            C::NegD => write!(f, "-D"),
            C::NegA => write!(f, "-{}", self.mode()),
            C::DPlusOne => write!(f, "D+1"),
            C::APlusOne => write!(f, "{}+1", self.mode()),
            C::DMinusOne => write!(f, "D-1"),
            C::AMinusOne => write!(f, "{}-1", self.mode()),
            C::DPlusA => write!(f, "D+{}", self.mode()),
            C::DMinusA => write!(f, "D-{}", self.mode()),
            C::AMinusD => write!(f, "{}-D", self.mode()),
            C::DAndA => write!(f, "D&{}", self.mode()),
            C::DOrA => write!(f, "D|{}", self.mode()),
            _ => Err(std::fmt::Error),
        }
    }
}

#[bitenum(u3, exhaustive: true)]
#[derive(Debug, PartialEq)]
pub enum Jump {
    /// No jump bits set.
    ///
    /// Do not jump.
    Never = 0,

    /// `JGT` bit set.
    ///
    /// Jump to the address in the `A` register if `comp > 0`
    JGT = 0b001,

    /// `JEQ` bit set.
    ///
    /// Jump to the address in the `A` register if `comp == 0`
    JEQ = 0b010,

    /// `JGT` and `JEQ` bits set.
    ///
    /// Jump to the address in the `A` register if `comp >= 0`
    JGE = 0b011,

    /// `JLT` bit set.
    ///
    /// Jump to the address in the `A` register if `comp < 0`
    JLT = 0b100,

    /// `JGT` and `JLT` bits set.
    ///
    /// Jump to the address in the `A` register if `comp != 0`
    JNE = 0b101,

    /// `JEQ` and `JLT` bits set.
    ///
    /// Jump to the address in the `A` register if `comp <= 0`
    JLE = 0b110,

    /// `JGT`, `JEQ` and `JLT` bits set.
    ///
    /// Jump to the address in the `A` register unconditionally
    JMP = 0b111,
}

impl std::fmt::Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Jump::Never => Ok(()),
            Jump::JGT => write!(f, "JGT"),
            Jump::JEQ => write!(f, "JEQ"),
            Jump::JGE => write!(f, "JGE"),
            Jump::JLT => write!(f, "JLT"),
            Jump::JNE => write!(f, "JNE"),
            Jump::JLE => write!(f, "JLE"),
            Jump::JMP => write!(f, "JMP"),
        }
    }
}

/// These enums are for specifying valid bit configurations for Hack instruction types
/// All C instructions have the 3 most significant bits set, and all A instructions have the most significant bit clear
#[bitenum(u3, exhaustive: false)]
pub enum NonAInst {
    C = 0b111,
}

/// These enums are for specifying valid bit configurations for Hack instruction types
/// All C instructions have the 3 most significant bits set, and all A instructions have the most significant bit clear
#[bitenum(u1: exhaustive: false)]
pub enum AInst {
    A = 0,
}

#[bitfield(u15, default: 0b110_0000_0000_0000)]
pub struct CInstruction {
    /// The computation bits of a C-Instruction (bits 6-12)
    #[bits(6..=12, rw)]
    comp: Comp,

    /// The destination bits of a C-Instruction (bits 3, 4, and 5).
    #[bits(3..=5, rw)]
    dest: DestBits,

    // /// Bit 5 of a C-Instruction
    // ///
    // /// If this flag is set, the value computed by `comp` will be stored in register `A`
    // #[bit(5, rw)]
    // dest_a: bool,

    // /// Bit 4 of a C-Instruction
    // ///
    // /// If this flag is set, the value computed by `comp` will be stored in register `D`
    // #[bit(4, rw)]
    // dest_d: bool,

    // /// Bit 3 of a C-Instruction
    // ///
    // /// If this flag is set, the value computed by `comp` will be stored in register `M` (or `Mem[A]`)
    // #[bit(3, rw)]
    // dest_m: bool,
    /// The `jump` portion of a C-instruction (bits 0, 1, and 2)
    #[bits(0..=2, rw)]
    jump: Jump,

    /// Bit 2 of a C-Instruction
    ///
    /// If this flag is set, jump to the address in register `A` if `comp < 0`
    #[bit(2, r)]
    jlt: bool,

    /// Bit 1 of a C-Instruction
    ///
    /// If this flag is set, jump to the address in register `A` if `comp == 0`
    #[bit(1, r)]
    jeq: bool,

    /// Bit 0 of a C-Instruction
    ///
    /// If this flag is set, jump to the address in register `A` if `comp > 0`
    #[bit(0, r)]
    jgt: bool,
}

/// Convenience enum for accessing the valid portion of a given Hack Instruction.
///
/// If niche optimizations for the `arbitrary-int` crate are implemented,
/// this pattern could potentially replace the `Instruction` struct as a wrapper around the full valid instructions,
/// but as is these take up 4 bytes instead of 2.
#[repr(u16)]
pub enum InstructionType {
    A(u15),
    C(CInstruction),
}

#[bitfield(u16, default: 0b111_0_101010_000_000)]
#[derive(Debug, Eq)]
/// Struct representing a Hack instruction.
///
/// Default value is `0b111_0_101010_000_000`, or the C-Instruction `0`, which like all C-Instructions with no `dest` or `jump` bits set, is a no-op.
pub struct Instruction {
    #[bit(15, r)]
    a_inst: Option<AInst>,
    #[bits(13..=15, r)]
    non_a_inst: Option<NonAInst>,
    #[bits(0..=14, rw)]
    addr: u15,
    #[bits(0..=14, rw)]
    c_inst: CInstruction,
}

impl Instruction {
    #[inline]
    const fn is_ok(&self) -> bool {
        self.a_inst().is_ok() || self.non_a_inst().is_ok()
    }

    #[inline]
    /// Creates a new C instruction with the given `dest`, `comp`, and `jump` segments.
    pub(crate) const fn c(dest: Dest, comp: ValidComp, jump: Jump) -> Self {
        Instruction::DEFAULT.with_c_inst(
            CInstruction::DEFAULT
                .with_dest(DestBits::DEFAULT.with_get(dest))
                .with_comp(Comp::new_valid(comp))
                .with_jump(jump),
        )
    }

    #[inline]
    /// Creates a new A instruction from the given 16 bit signed integer.
    ///
    /// The sign bit of the input is cleared, so this will *always* be an A instruction, even if the input is negative.
    ///
    /// This could lead to unexpected behavior if called erroneously with a negative input.
    /// Should look into making that a compilation error.
    pub(crate) const fn a(addr: i16) -> Self {
        #[allow(overflowing_literals)]
        Self::new_with_raw_value((addr & i16::MAX) as u16)
    }

    /// Returns correctly represented instructions as the appropriate type.
    ///
    /// In implementations where bits 13 and 14 are fully specified (perhaps with extension chips)
    /// there is not necessarily a need to return a `Result`.
    pub(crate) const fn get(&self) -> Result<InstructionType> {
        match (self.a_inst(), self.non_a_inst()) {
            (Ok(_), _) => Ok(InstructionType::A(self.addr())),
            (_, Ok(NonAInst::C)) => Ok(InstructionType::C(self.c_inst())),
            _ => todo!(),
        }
    }

    /*******************************************************************************
     * MEMORY SEGMENT POINTERS
     *******************************************************************************/
    /// The address of the stack pointer is always held at address 0.
    ///
    /// In the official specification, the stack pointer is one past the top of the stack,
    /// which ends up making `pop` operations one instruction longer than `push` operations.
    ///
    /// However, most VM code involves more pushes than pops, because arithmetic operations move the stack pointer without popping,
    /// and function arguments and stack frames are pushed on but not (necessarily) popped off.
    ///
    /// Therefore, optimizing for `push` over `pop` can save instructions, and this can be done by making the stack pointer point directly at the top of the stack.
    pub const SP: Self = Self { raw_value: 0 };

    /// The address of the current frame's `local` memory segment is stored at address 1.
    pub const LCL: Self = Self { raw_value: 1 };

    /// The address of the current frame's `argument` memory segment is stored at address 2.
    pub const ARG: Self = Self { raw_value: 2 };

    /// The address of the current frame's `this` memory segment is stored at address 3.
    ///
    /// This is `pointer 0` in the VM abstraction.
    pub const THIS: Self = Self { raw_value: 3 };

    /// The address of the current frame's `that` memory segment is stored at address 4.
    ///
    /// This is `pointer 1` in the VM abstraction.
    pub const THAT: Self = Self { raw_value: 4 };

    /*******************************************************************************
     * VIRTUAL REGISTERS
     *******************************************************************************/

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R0: Self = Self { raw_value: 0 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R1: Self = Self { raw_value: 1 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R2: Self = Self { raw_value: 2 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R3: Self = Self { raw_value: 3 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R4: Self = Self { raw_value: 4 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R5: Self = Self { raw_value: 5 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R6: Self = Self { raw_value: 6 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R7: Self = Self { raw_value: 7 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R8: Self = Self { raw_value: 8 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R9: Self = Self { raw_value: 9 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R10: Self = Self { raw_value: 10 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R11: Self = Self { raw_value: 11 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R12: Self = Self { raw_value: 12 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R13: Self = Self { raw_value: 13 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R14: Self = Self { raw_value: 14 };

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R15: Self = Self { raw_value: 15 };

    /*******************************************************************************
     * HARDCODED ADDRESSES
     *******************************************************************************/
    /// The screen of the Hack platform is hardware-mapped to the address range `0x4000..=0x5FFF`
    ///
    /// The 1bpp screen is displayed least significant bit to most significant bit from left to right
    pub const SCREEN: Self = Self { raw_value: 16384 };

    /// The keyboard is hardware-mapped to the address 24576
    ///
    /// The `KBD` register is read-only.
    pub const KBD: Self = Self { raw_value: 24576 };

    /// Convenience for unconditional jumps that do not take advantage of computation/destination optimizations such as `A=0;JMP`
    ///
    /// Represented in assembly as `0;JMP`
    pub const JMP: Self = Self::c(Dest::None, ValidComp::Zero, Jump::JMP);

    /// The max addressable value of an A instruction.
    ///
    /// Not officially specified, but the intent is clearer than using `32767`
    ///
    /// The primary use case is as an adjacent value to `i16::MIN` (either with bitwise NOT or adding 1) which is useful for bitwise comparisons.
    pub const MAX: Self = Self {
        raw_value: i16::MAX as u16,
    };
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value == other.raw_value
    }
}

impl From<i16> for Instruction {
    #[allow(overflowing_literals)]
    fn from(value: i16) -> Self {
        Self {
            raw_value: value as u16,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.a_inst().is_ok() {
            write!(f, "@{}", self.addr())
        } else if self.non_a_inst().is_ok() {
            let c = self.c_inst();

            match c.dest().get() {
                Dest::None => Ok(()),
                d => write!(f, "{d}="),
            }?;

            write!(f, "{}", c.comp())?;

            match c.jump() {
                Jump::Never => Ok(()),
                j => write!(f, ";{j}"),
            }
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl std::fmt::UpperHex for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.raw_value)
    }
}

impl std::fmt::LowerHex for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.raw_value)
    }
}

impl std::fmt::Binary for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.raw_value)
    }
}

impl std::fmt::Octal for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06o}", self.raw_value)
    }
}

/// Represents a line of valid Hack assembly language.
// Consider making a NewLine variant and giving every variant an Option<&'a str> for a comment
// would make representing any user formatting pretty comfy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asm<'a> {
    Comment(Cow<'a, str>),
    Label(Cow<'a, str>),
    At(Cow<'a, str>),
    Asm(Instruction),
}

// trait Str<'a>: Into<Cow<'a, str>> {}

// impl<'a> Str<'a> for &'a str {}
// impl Str<'_> for String {}
// impl<'a> Str<'a> for Cow<'a, str> {}

// impl<'a, T: Str<'a>> From<T> for Asm<'a> {
//     fn from(value: T) -> Self {
//         Self::At(value.into())
//     }
// }

impl From<i16> for Asm<'_> {
    #[allow(overflowing_literals)]
    fn from(value: i16) -> Self {
        Self::Asm(Instruction {
            raw_value: value as u16,
        })
    }
}

impl From<u16> for Asm<'_> {
    fn from(value: u16) -> Self {
        Self::Asm(Instruction { raw_value: value })
    }
}

impl From<String> for Asm<'_> {
    fn from(value: String) -> Self {
        Self::At(Cow::Owned(value))
    }
}

impl<'a> From<&'a str> for Asm<'a> {
    fn from(value: &'a str) -> Self {
        Self::At(Cow::Borrowed(value))
    }
}

impl std::fmt::Display for Asm<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Asm::Comment(s) => {
                if !s.is_empty() {
                    write!(f, "// {s}")
                } else {
                    Ok(())
                }
            }
            Asm::Label(l) => write!(f, "({l})"),
            Asm::At(a) => write!(f, "@{a}"),
            Asm::Asm(i) => write!(f, "{i}"),
        }
    }
}
impl Asm<'static> {
    /// The address of the stack pointer is always held at address 0.
    ///
    /// In the official specification, the stack pointer is one past the top of the stack,
    /// which ends up making `pop` operations one instruction longer than `push` operations.
    ///
    /// However, most VM code involves more pushes than pops, because arithmetic operations move the stack pointer without popping,
    /// and function arguments and stack frames are pushed on but not (necessarily) popped off.
    ///
    /// Therefore, optimizing for `push` over `pop` can save instructions, and this can be done by making the stack pointer point directly at the top of the stack.
    pub const SP: Self = Self::At(Cow::Borrowed("SP"));

    /// The address of the current frame's `local` memory segment is stored at address 1.
    pub const LCL: Self = Self::At(Cow::Borrowed("LCL"));

    /// The address of the current frame's `argument` memory segment is stored at address 2.
    pub const ARG: Self = Self::At(Cow::Borrowed("ARG"));

    /// The address of the current frame's `this` memory segment is stored at address 3.
    ///
    /// This is `pointer 0` in the VM abstraction.
    pub const THIS: Self = Self::At(Cow::Borrowed("THIS"));

    /// The address of the current frame's `that` memory segment is stored at address 4.
    ///
    /// This is `pointer 1` in the VM abstraction.
    pub const THAT: Self = Self::At(Cow::Borrowed("THAT"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R0: Self = Self::At(Cow::Borrowed("R0"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R1: Self = Self::At(Cow::Borrowed("R1"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R2: Self = Self::At(Cow::Borrowed("R2"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R3: Self = Self::At(Cow::Borrowed("R3"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    ///
    /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
    pub const R4: Self = Self::At(Cow::Borrowed("R4"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R5: Self = Self::At(Cow::Borrowed("R5"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R6: Self = Self::At(Cow::Borrowed("R6"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R7: Self = Self::At(Cow::Borrowed("R7"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R8: Self = Self::At(Cow::Borrowed("R8"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R9: Self = Self::At(Cow::Borrowed("R9"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R10: Self = Self::At(Cow::Borrowed("R10"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R11: Self = Self::At(Cow::Borrowed("R11"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R12: Self = Self::At(Cow::Borrowed("R12"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R13: Self = Self::At(Cow::Borrowed("R13"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R14: Self = Self::At(Cow::Borrowed("R14"));

    /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing whether the value is being used as an address or a constant.
    pub const R15: Self = Self::At(Cow::Borrowed("R15"));

    /// The screen of the Hack platform is hardware-mapped to the address range `0x4000..=0x5FFF`
    ///
    /// Each address in this range corresponds to 16px on the screen, for 1bpp encoding
    /// 
    /// The bits are displayed least significant bit to most significant bit from left to right
    pub const SCREEN: Self = Self::At(Cow::Borrowed("SCREEN"));

    /// The keyboard is hardware-mapped to the address 24576
    ///
    /// The `KBD` register is read-only.
    pub const KBD: Self = Self::At(Cow::Borrowed("KBD"));

    /// The max addressable value of an A instruction.
    ///
    /// Not officially specified, but the intent is clearer than using `32767`
    ///
    /// The primary use case is as an adjacent value to `i16::MIN` (either with bitwise NOT or adding 1) which is useful for bitwise comparisons.
    pub const MAX: Self = Self::At(Cow::Borrowed("MAX"));
}

impl<'a> Asm<'a> {
    #[inline]
    pub fn at_opt(input: &'a str) -> Self {
        // Optional @
        Self::At(input.strip_prefix('@').unwrap_or(input).into())
    }
}

pub struct Assembler {
    pub labels: HashMap<String, i16>,
    pub var_counter: i16,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: HashMap::new(),
            var_counter: 15, // Starts at 15 so we can increment it pre insertion
        }
    }

    // Helper function to abstract over checking the static list first, then the labels unique to this assembly
    #[inline]
    fn get_label(&mut self, label: &str) -> Option<i16> {
        match label {
            "SP" | "R0" => Some(0),
            "LCL" | "R1" => Some(1),
            "ARG" | "R2" => Some(2),
            "THIS" | "R3" => Some(3),
            "THAT" | "R4" => Some(4),
            "R5" => Some(5),
            "R6" => Some(6),
            "R7" => Some(7),
            "R8" => Some(8),
            "R9" => Some(9),
            "R10" => Some(10),
            "R11" => Some(11),
            "R12" => Some(12),
            "R13" => Some(13),
            "R14" => Some(14),
            "R15" => Some(15),
            _ => self.labels.get(label).copied(),
        }
    }

    fn parse_a_instruction<'a>(&mut self, input: &'a str) -> Asm<'a> {
        match input.parse::<i16>() {
            Ok(n) if n >= 0 => Asm::Asm(Instruction::from(n)),
            // If the address given is not a valid positive signed 16-bit integer, interpret it as a variable
            // Could also return an error, But I think most written assembly would not have intentionally numericized labels
            _ => {
                if self.get_label(input).is_none() {
                    self.var_counter += 1;
                    self.labels.insert(String::from(input), self.var_counter);
                }
                Asm::At(input.into())
            }
        }
    }

    fn parse_c_instruction<'a>(&self, input: &'a str) -> Result<Asm<'a>> {
        // There will always be a computation field, so we set the bounds now
        let mut comp_start = 0;
        let mut comp_end = input.len();

        // DEST
        // All valid commands with a destination field include '='
        // Technically this current implementation allows including valid destinations in a non-standard order
        let dest = if let Some(i) = input.find('=') {
            // we know the start of the computation field comes immediately after the '='
            comp_start = i + 1;
            let dest = &input[..i];

            // Making sure that only valid destinations are used.
            if !dest.chars().all(|c| "AMD".contains(c)) {
                bail!("'{dest}' contains an invalid destination character.");
            }

            // Allowing the destinations in any order might be too permissive, but it's fine for now
            Dest::from_flags(dest.contains('A'), dest.contains('M'), dest.contains('D'))
        } else {
            Dest::None
        };

        // JUMP
        let jump = if let Some(i) = input.find(';') {
            comp_end = i;
            match &input[i + 1..] {
                "JGT" => Jump::JGT,
                "JEQ" => Jump::JEQ,
                "JGE" => Jump::JGE,
                "JLT" => Jump::JLT,
                "JNE" => Jump::JNE,
                "JLE" => Jump::JLE,
                "JMP" => Jump::JMP,
                _ => bail!("Semicolon requires a valid jump command!"),
            }
        } else {
            Jump::Never
        };

        // COMP
        let comp = match &input[comp_start..comp_end] {
            "0" => ValidComp::Zero,
            "1" => ValidComp::One,
            "-1" => ValidComp::NegOne,
            "D" => ValidComp::D,
            "A" => ValidComp::A,
            "M" => ValidComp::M,
            // Making the executive decision to allow '~' as a bitwise NOT operator
            // Especially because it's used instead of ! in the Jack standard
            "!D" | "~D" => ValidComp::NotD,
            "!A" | "~A" => ValidComp::NotA,
            "!M" | "~M" => ValidComp::NotM,
            // Since all of these are semantically equivalent, as long as it's a perfect match we'll allow either order
            "D+A" | "A+D" => ValidComp::DPlusA,
            "D+M" | "M+D" => ValidComp::DPlusM,
            "D&A" | "A&D" => ValidComp::DAndA,
            "D&M" | "M&D" => ValidComp::DAndM,
            "D|A" | "A|D" => ValidComp::DOrA,
            "D|M" | "M|D" => ValidComp::DOrM,
            // Back to your regularly scheduled standard
            "-D" => ValidComp::NegD,
            "-A" => ValidComp::NegA,
            "-M" => ValidComp::NegM,
            "D+1" => ValidComp::DPlusOne,
            "A+1" => ValidComp::APlusOne,
            "M+1" => ValidComp::MPlusOne,
            "D-1" => ValidComp::DMinusOne,
            "A-1" => ValidComp::AMinusOne,
            "M-1" => ValidComp::MMinusOne,
            "D-A" => ValidComp::DMinusA,
            "D-M" => ValidComp::DMinusM,
            "A-D" => ValidComp::AMinusD,
            "M-D" => ValidComp::MMinusD,
            _ => bail!("invalid or unsupported computation field"),
        };

        Ok(Asm::Asm(Instruction::c(dest, comp, jump)))
    }

    pub fn translate<'a>(&mut self, input: &'a str) -> Result<Asm<'a>> {
        // A or C instruction
        if let Some(i) = input.strip_prefix('@') {
            Ok(self.parse_a_instruction(i))
        } else {
            self.parse_c_instruction(input)
        }
    }

    pub fn assemble(&mut self, asm: &[Asm]) -> Vec<Instruction> {
        // first pass
        let mut line: i16 = 0;
        for com in asm {
            if let Asm::Label(s) = com {
                //println!("({com})");
                if self.get_label(s).is_none() {
                    self.labels.insert(s.to_string(), line);
                }
            } else {
                line += 1;
            }
        }
        asm.iter()
            .filter_map(|c| match c {
                Asm::At(l) => self.get_label(l.as_ref()).map(Instruction::from),
                Asm::Asm(i) => Some(*i),
                _ => None,
            })
            .collect()
    }
}

// fn write_bin() {
//     let args: Vec<String> = std::env::args().collect();
//     let filename = args[1].clone();
//     let mut asm = vec![];
//     let mut assembler = Assembler::new();
//     if let Ok(f) = File::open(filename) {
//         let reader = BufReader::new(f);
//         for line in reader.lines().flatten() {
//             let cmd = strip_line(&line);
//             if !cmd.is_empty() {
//                 asm.push(cmd);
//             }
//         }
//     }
//     if let Ok(bin) = assembler.assemble(&asm) {}
//     let bin = assembler.assemble(&asm).unwrap();
//     for b in bin {
//         println!("{b:016b}");
//     }
// }

// fn strip_line(input: &str) -> String {
//     input
//         .find("//")
//         .map(|i| &input[..i])
//         .unwrap_or(input)
//         .replace(' ', "")
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     #[test]
//     fn valid_c_bits() {
//         let c_bits =
//             Comp::try_from(Instruction::C_NO | Instruction::C_F | Instruction::C_ND).unwrap();
//         assert_eq!(c_bits, Comp::DMinusA);
//     }
// }
