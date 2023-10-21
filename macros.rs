#![feature(prelude_import)]
#![allow(dead_code)]
#![allow(clippy::unusual_byte_groupings)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod asm {
    use anyhow::{bail, Result};
    use arbitrary_int::{u15, u3, u7};
    use bitbybit::{bitenum, bitfield};
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::convert::Infallible;
    use std::ops::Deref;
    use std::str::FromStr;
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Dest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Dest::None => "None",
                    Dest::M => "M",
                    Dest::D => "D",
                    Dest::MD => "MD",
                    Dest::A => "A",
                    Dest::AM => "AM",
                    Dest::AD => "AD",
                    Dest::AMD => "AMD",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Dest {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Dest {
        #[inline]
        fn eq(&self, other: &Dest) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Dest {}
    #[automatically_derived]
    impl ::core::clone::Clone for Dest {
        #[inline]
        fn clone(&self) -> Dest {
            *self
        }
    }
    impl Dest {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 3usize> {
            arbitrary_int::UInt::<u8, 3usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// As the enum is exhaustive, this function will always return a valid result
        pub const fn new_with_raw_value(value: arbitrary_int::UInt<u8, 3usize>) -> Self {
            match value.value() {
                0 => Self::None,
                1 => Self::M,
                2 => Self::D,
                3 => Self::MD,
                4 => Self::A,
                5 => Self::AM,
                6 => Self::AD,
                7 => Self::AMD,
                _ => {
                    ::core::panicking::panic_fmt(format_args!("Dest: Unhandled value"));
                }
            }
        }
    }
    #[repr(C)]
    pub struct DestBits {
        raw_value: u8,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DestBits {}
    #[automatically_derived]
    impl ::core::clone::Clone for DestBits {
        #[inline]
        fn clone(&self) -> DestBits {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    impl DestBits {
        const DEFAULT_RAW_VALUE: u3 = u3::new(0);
        ///An instance that uses the default value 0
        #[inline]
        pub const DEFAULT: Self = Self::new_with_raw_value(Self::DEFAULT_RAW_VALUE);
        #[deprecated(
            note = "Use DestBits::Default (or DestBits::DEFAULT in const context) instead"
        )]
        pub const fn new() -> Self {
            Self::DEFAULT
        }
        /// Returns the underlying raw value of this bitfield
        #[inline]
        pub const fn raw_value(&self) -> u3 {
            u3::extract_u8(self.raw_value, 0)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// No checks are performed on the value, so it is possible to set bits that don't have any
        /// accessors specified.
        #[inline]
        pub const fn new_with_raw_value(value: u3) -> DestBits {
            DestBits {
                raw_value: value.value(),
            }
        }
        #[inline]
        pub const fn get(&self) -> Dest {
            let extracted_bits = arbitrary_int::u3::extract_u8(self.raw_value, 0usize);
            Dest::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn with_get(&self, field_value: Dest) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u8 << 3usize) - 1u8) << 0usize))
                    | ((field_value.raw_value().value() as u8) << 0usize),
            }
        }
        /// Bit 3 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `M`
        #[inline]
        pub const fn m(&self) -> bool {
            (self.raw_value & (1u8 << 0usize)) != 0
        }
        /// Bit 3 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `M`
        #[inline]
        pub const fn with_m(&self, field_value: bool) -> Self {
            Self {
                raw_value: if field_value {
                    self.raw_value | (1u8 << 0usize)
                } else {
                    self.raw_value & !(1u8 << 0usize)
                },
            }
        }
        /// Bit 4 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `D`
        #[inline]
        pub const fn d(&self) -> bool {
            (self.raw_value & (1u8 << 1usize)) != 0
        }
        /// Bit 4 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `D`
        #[inline]
        pub const fn with_d(&self, field_value: bool) -> Self {
            Self {
                raw_value: if field_value {
                    self.raw_value | (1u8 << 1usize)
                } else {
                    self.raw_value & !(1u8 << 1usize)
                },
            }
        }
        /// Bit 5 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `A`
        #[inline]
        pub const fn a(&self) -> bool {
            (self.raw_value & (1u8 << 2usize)) != 0
        }
        /// Bit 5 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `A`
        #[inline]
        pub const fn with_a(&self, field_value: bool) -> Self {
            Self {
                raw_value: if field_value {
                    self.raw_value | (1u8 << 2usize)
                } else {
                    self.raw_value & !(1u8 << 2usize)
                },
            }
        }
    }
    impl Default for DestBits {
        fn default() -> Self {
            Self::DEFAULT
        }
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
                Self::M => f.write_fmt(format_args!("M")),
                Self::D => f.write_fmt(format_args!("D")),
                Self::MD => f.write_fmt(format_args!("MD")),
                Self::A => f.write_fmt(format_args!("A")),
                Self::AM => f.write_fmt(format_args!("AM")),
                Self::AD => f.write_fmt(format_args!("AD")),
                Self::AMD => f.write_fmt(format_args!("AMD")),
            }
        }
    }
    /// The comp segment (bits 6-11) represents the computations performed in a C-Instruction by the Hack ALU
    ///
    /// They are named based on the A register in all cases where its value is used in the calculation
    ///
    /// Bit 12 is used to determine whether A is read by value (`A`) or as a pointer (`M`/`memory[A]`)
    #[rustfmt::skip]
    pub enum ValidComp {
        /// The configuration of C bits that translates to the comp `0` in the Hack assembly specification.
        Zero = 0b0101010,
        /// The configuration of C bits that translates to the comp `1` in the Hack assembly specification.
        One = 0b0111111,
        /// The configuration of C bits that translates to the comp `-1` in the Hack assembly specification.
        NegOne = 0b0111010,
        /// The configuration of C bits that translates to the comp `D` in the Hack assembly specification.
        D = 0b0001100,
        /// The configuration of C bits that translates to the comp `A` in the Hack assembly specification.
        A = 0b0110000,
        /// The configuration of C bits that translates to the comp `M` in the Hack assembly specification.
        M = 0b1110000,
        /// The configuration of C bits that translates to the comp `!D` in the Hack assembly specification.
        NotD = 0b0001101,
        /// The configuration of C bits that translates to the comp `!A` in the Hack assembly specification.
        NotA = 0b0110001,
        /// The configuration of C bits that translates to the comp `!M` in the Hack assembly specification.
        NotM = 0b1110001,
        /// The configuration of C bits that translates to the comp `-D` in the Hack assembly specification.
        NegD = 0b0001111,
        /// The configuration of C bits that translates to the comp `-A` in the Hack assembly specification.
        NegA = 0b0110011,
        /// The configuration of C bits that translates to the comp `-M` in the Hack assembly specification.
        NegM = 0b1110011,
        /// The configuration of C bits that translates to the comp `D+1` in the Hack assembly specification.
        DPlusOne = 0b0011111,
        /// The configuration of C bits that translates to the comp `A+1` in the Hack assembly specification.
        APlusOne = 0b0110111,
        /// The configuration of C bits that translates to the comp `M+1` in the Hack assembly specification.
        MPlusOne = 0b1110111,
        /// The configuration of C bits that translates to the comp `D-1` in the Hack assembly specification.
        DMinusOne = 0b0001110,
        /// The configuration of C bits that translates to the comp `A-1` in the Hack assembly specification.
        AMinusOne = 0b0110010,
        /// The configuration of C bits that translates to the comp `M-1` in the Hack assembly specification.
        MMinusOne = 0b1110010,
        /// The configuration of C bits that translates to the comp `D+A` in the Hack assembly specification.
        DPlusA = 0b0000010,
        /// The configuration of C bits that translates to the comp `D+M` in the Hack assembly specification.
        DPlusM = 0b1000010,
        /// The configuration of C bits that translates to the comp `D-A` in the Hack assembly specification.
        DMinusA = 0b0010011,
        /// The configuration of C bits that translates to the comp `D-M` in the Hack assembly specification.
        DMinusM = 0b1010011,
        /// The configuration of C bits that translates to the comp `A-D` in the Hack assembly specification.
        AMinusD = 0b0000111,
        /// The configuration of C bits that translates to the comp `M-D` in the Hack assembly specification.
        MMinusD = 0b1000111,
        /// The configuration of C bits that translates to the comp `D&A` in the Hack assembly specification.
        DAndA = 0b0000000,
        /// The configuration of C bits that translates to the comp `D&M` in the Hack assembly specification.
        DAndM = 0b1000000,
        /// The configuration of C bits that translates to the comp `D|A` in the Hack assembly specification.
        DOrA = 0b0010101,
        /// The configuration of C bits that translates to the comp `D|M` in the Hack assembly specification.
        DOrM = 0b1010101,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ValidComp {}
    #[automatically_derived]
    impl ::core::clone::Clone for ValidComp {
        #[inline]
        fn clone(&self) -> ValidComp {
            *self
        }
    }
    impl ValidComp {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 7usize> {
            arbitrary_int::UInt::<u8, 7usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value, or
        /// Err(value) if the value does not exist in the enum.
        pub const fn new_with_raw_value(
            value: arbitrary_int::UInt<u8, 7usize>,
        ) -> Result<Self, u8> {
            match value.value() {
                0b0101010 => Ok(Self::Zero),
                0b0111111 => Ok(Self::One),
                0b0111010 => Ok(Self::NegOne),
                0b0001100 => Ok(Self::D),
                0b0110000 => Ok(Self::A),
                0b1110000 => Ok(Self::M),
                0b0001101 => Ok(Self::NotD),
                0b0110001 => Ok(Self::NotA),
                0b1110001 => Ok(Self::NotM),
                0b0001111 => Ok(Self::NegD),
                0b0110011 => Ok(Self::NegA),
                0b1110011 => Ok(Self::NegM),
                0b0011111 => Ok(Self::DPlusOne),
                0b0110111 => Ok(Self::APlusOne),
                0b1110111 => Ok(Self::MPlusOne),
                0b0001110 => Ok(Self::DMinusOne),
                0b0110010 => Ok(Self::AMinusOne),
                0b1110010 => Ok(Self::MMinusOne),
                0b0000010 => Ok(Self::DPlusA),
                0b1000010 => Ok(Self::DPlusM),
                0b0010011 => Ok(Self::DMinusA),
                0b1010011 => Ok(Self::DMinusM),
                0b0000111 => Ok(Self::AMinusD),
                0b1000111 => Ok(Self::MMinusD),
                0b0000000 => Ok(Self::DAndA),
                0b1000000 => Ok(Self::DAndM),
                0b0010101 => Ok(Self::DOrA),
                0b1010101 => Ok(Self::DOrM),
                _ => Err(value.value()),
            }
        }
    }
    #[rustfmt::skip]
    pub enum CBits {
        /// The specified C-bit configuration evaluating to `0`
        ///
        /// `0 + 0`
        Zero = 0b101010,
        /// The specified C-bit configuration evaluating to `1`
        ///
        /// `!(-1 + -1)`
        One = 0b111111,
        /// The specified C-bit configuration evaluating to `-1`
        ///
        /// `-1 + 0`
        NegOne = 0b111010,
        /// The specified C-bit configuration evaluating to `D`
        ///
        /// `D & -1`
        D = 0b001100,
        /// The specified C-bit configuration evaluating to `A`
        ///
        /// `-1 & A`
        A = 0b110000,
        /// The specified C-bit configuration evaluating to `!D`
        ///
        /// `!(D & -1)`
        NotD = 0b001101,
        /// The specified C-bit configuration evaluating to `A`
        ///
        /// `!(-1 & A)`
        NotA = 0b110001,
        /// The specified C-bit configuration evaluating to `-D`
        ///
        /// `!(D + -1)`
        NegD = 0b001111,
        /// The specified C-bit configuration evaluating to `-A`
        ///
        /// `!(-1 + A)`
        NegA = 0b110011,
        /// The specified C-bit configuration evaluating to `D + 1`
        ///
        /// `!(!D + -1)`
        DPlusOne = 0b011111,
        /// The specified C-bit configuration evaluating to `A + 1`
        ///
        /// `!(-1 + !A)`
        APlusOne = 0b110111,
        /// The specified C-bit configuration evaluating to `D - 1`
        ///
        /// `D + -1`
        DMinusOne = 0b001110,
        /// The specified C-bit configuration evaluating to `A - 1`
        ///
        /// `-1 + A`
        AMinusOne = 0b110010,
        /// The specified C-bit configuration evaluating to `D + A`
        ///
        /// `D + A`
        DPlusA = 0b000010,
        /// The specified C-bit configuration evaluating to `D - A`
        ///
        /// `!(!D + A)`
        DMinusA = 0b010011,
        /// The specified C-bit configuration evaluating to `A - D`
        ///
        /// `!(D + !A)`
        AMinusD = 0b000111,
        /// The specified C-bit configuration evaluating to `D & A`
        ///
        /// `D & A`
        DAndA = 0b000000,
        /// The specified C-bit configuration evaluating to `D | A`
        ///
        /// `!(!D & !A)`
        DOrA = 0b010101,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `!(-1 & -1)`
        Zero0 = 0b111101,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `!(-1 + 0)`
        Zero1 = 0b111011,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `D & 0`
        Zero2 = 0b001000,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `!D & 0`
        Zero3 = 0b011000,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `0 & A`
        Zero4 = 0b100000,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `0 & !A`
        Zero5 = 0b100100,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `0 & 0`
        Zero6 = 0b101000,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `0 & -1`
        Zero7 = 0b101100,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `!(0 + -1)`
        Zero8 = 0b101111,
        /// An unspecified C-bit configuration evaluating to `0`
        ///
        /// `-1 & 0`
        Zero9 = 0b111000,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne0 = 0b001001,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne1 = 0b011001,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne2 = 0b100001,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne3 = 0b100101,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne4 = 0b101001,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne5 = 0b101011,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne6 = 0b101101,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne7 = 0b101110,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne8 = 0b111001,
        /// An unspecified C-bit configuration evaluating to `-1`
        NegOne9 = 0b111100,
        /// An unspecified C-bit configuration evaluating to `D`
        D0 = 0b001010,
        /// An unspecified C-bit configuration evaluating to `D`
        D1 = 0b011011,
        /// An unspecified C-bit configuration evaluating to `D`
        D2 = 0b011101,
        /// An unspecified C-bit configuration evaluating to `A`
        A0 = 0b100010,
        /// An unspecified C-bit configuration evaluating to `A`
        A1 = 0b100111,
        /// An unspecified C-bit configuration evaluating to `A`
        A2 = 0b110101,
        /// An unspecified C-bit configuration evaluating to `!D`
        NotD0 = 0b001011,
        /// An unspecified C-bit configuration evaluating to `!D`
        ///
        /// `!D & -1`
        NotD1 = 0b011100,
        /// An unspecified C-bit configuration evaluating to `!D`
        ///
        /// `!D + 0`
        NotD2 = 0b011010,
        /// An unspecified C-bit configuration evaluating to `!A`:
        ///
        /// `!(0 + A)`
        NotA0 = 0b100011,
        /// An unspecified C-bit configuration evaluating to `!A`
        ///
        /// `0 + !A`
        NotA1 = 0b100110,
        /// An unspecified C-bit configuration evaluating to `!A`
        ///
        /// `-1 & !A`
        NotA2 = 0b110100,
        /// An unspecified C-bit configuration evaluating to `!D|!A`
        ///
        /// `!(D & A)`
        NotDOrNotA = 0b000001,
        /// An unspecified C-bit configuration evaluating to `!(D + A)`
        ///
        /// `!(D + A)`
        NotOfDPlusA = 0b000011,
        /// An unspecified C-bit configuration evaluating to `D & !A`
        ///
        /// `D & !A`
        DAndNotA = 0b000100,
        /// An unspecified C-bit configuration evaluating to `!D | A`
        ///
        /// `!(D & !A)`
        NotDOrA = 0b000101,
        /// An unspecified C-bit configuration evaluating to `D + !A`
        ///
        /// `D + !A`
        DPlusNotA = 0b000110,
        /// An unspecified C-bit configuration evaluating to `!D & A`
        ///
        /// `D & A`
        NotDAndA = 0b010000,
        /// An unspecified C-bit configuration evaluating to `D | !A`
        ///
        /// `!(!D & A)`
        DOrNotA = 0b010001,
        /// An unspecified C-bit configuration evaluating to `!D + A`
        ///
        /// `!D + A`
        NotDPlusA = 0b010010,
        /// An unspecified C-bit configuration evaluating to `!D & !A`
        ///
        /// `!D & !A`
        NotDAndNotA = 0b010100,
        /// An unspecified C-bit configuration evaluating to `!D + !A`
        ///
        /// `!D + !A`
        NotDPlusNotA = 0b010110,
        /// An unspecified C-bit configuration evaluating to `!D - 1`
        ///
        /// `!D + -1`
        NotDMinus1 = 0b011110,
        /// An unspecified C-bit configuration evaluating to `!(!D + !A)`
        ///
        /// `!(!D + !A)`
        NotNotDPlusNotA = 0b010111,
        /// An unspecified C-bit configuration evaluating to `!A - 1`
        ///
        /// `-1 + !A`
        NotAMinus1 = 0b110110,
        /// An unspecified C-bit configuration evaluating to `-2` (or `!1`)
        ///
        /// `-1 + -1`
        NegTwo = 0b111110,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CBits {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    CBits::Zero => "Zero",
                    CBits::One => "One",
                    CBits::NegOne => "NegOne",
                    CBits::D => "D",
                    CBits::A => "A",
                    CBits::NotD => "NotD",
                    CBits::NotA => "NotA",
                    CBits::NegD => "NegD",
                    CBits::NegA => "NegA",
                    CBits::DPlusOne => "DPlusOne",
                    CBits::APlusOne => "APlusOne",
                    CBits::DMinusOne => "DMinusOne",
                    CBits::AMinusOne => "AMinusOne",
                    CBits::DPlusA => "DPlusA",
                    CBits::DMinusA => "DMinusA",
                    CBits::AMinusD => "AMinusD",
                    CBits::DAndA => "DAndA",
                    CBits::DOrA => "DOrA",
                    CBits::Zero0 => "Zero0",
                    CBits::Zero1 => "Zero1",
                    CBits::Zero2 => "Zero2",
                    CBits::Zero3 => "Zero3",
                    CBits::Zero4 => "Zero4",
                    CBits::Zero5 => "Zero5",
                    CBits::Zero6 => "Zero6",
                    CBits::Zero7 => "Zero7",
                    CBits::Zero8 => "Zero8",
                    CBits::Zero9 => "Zero9",
                    CBits::NegOne0 => "NegOne0",
                    CBits::NegOne1 => "NegOne1",
                    CBits::NegOne2 => "NegOne2",
                    CBits::NegOne3 => "NegOne3",
                    CBits::NegOne4 => "NegOne4",
                    CBits::NegOne5 => "NegOne5",
                    CBits::NegOne6 => "NegOne6",
                    CBits::NegOne7 => "NegOne7",
                    CBits::NegOne8 => "NegOne8",
                    CBits::NegOne9 => "NegOne9",
                    CBits::D0 => "D0",
                    CBits::D1 => "D1",
                    CBits::D2 => "D2",
                    CBits::A0 => "A0",
                    CBits::A1 => "A1",
                    CBits::A2 => "A2",
                    CBits::NotD0 => "NotD0",
                    CBits::NotD1 => "NotD1",
                    CBits::NotD2 => "NotD2",
                    CBits::NotA0 => "NotA0",
                    CBits::NotA1 => "NotA1",
                    CBits::NotA2 => "NotA2",
                    CBits::NotDOrNotA => "NotDOrNotA",
                    CBits::NotOfDPlusA => "NotOfDPlusA",
                    CBits::DAndNotA => "DAndNotA",
                    CBits::NotDOrA => "NotDOrA",
                    CBits::DPlusNotA => "DPlusNotA",
                    CBits::NotDAndA => "NotDAndA",
                    CBits::DOrNotA => "DOrNotA",
                    CBits::NotDPlusA => "NotDPlusA",
                    CBits::NotDAndNotA => "NotDAndNotA",
                    CBits::NotDPlusNotA => "NotDPlusNotA",
                    CBits::NotDMinus1 => "NotDMinus1",
                    CBits::NotNotDPlusNotA => "NotNotDPlusNotA",
                    CBits::NotAMinus1 => "NotAMinus1",
                    CBits::NegTwo => "NegTwo",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CBits {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CBits {
        #[inline]
        fn eq(&self, other: &CBits) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CBits {}
    #[automatically_derived]
    impl ::core::clone::Clone for CBits {
        #[inline]
        fn clone(&self) -> CBits {
            *self
        }
    }
    impl CBits {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 6usize> {
            arbitrary_int::UInt::<u8, 6usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// As the enum is exhaustive, this function will always return a valid result
        pub const fn new_with_raw_value(value: arbitrary_int::UInt<u8, 6usize>) -> Self {
            match value.value() {
                0b101010 => Self::Zero,
                0b111111 => Self::One,
                0b111010 => Self::NegOne,
                0b001100 => Self::D,
                0b110000 => Self::A,
                0b001101 => Self::NotD,
                0b110001 => Self::NotA,
                0b001111 => Self::NegD,
                0b110011 => Self::NegA,
                0b011111 => Self::DPlusOne,
                0b110111 => Self::APlusOne,
                0b001110 => Self::DMinusOne,
                0b110010 => Self::AMinusOne,
                0b000010 => Self::DPlusA,
                0b010011 => Self::DMinusA,
                0b000111 => Self::AMinusD,
                0b000000 => Self::DAndA,
                0b010101 => Self::DOrA,
                0b111101 => Self::Zero0,
                0b111011 => Self::Zero1,
                0b001000 => Self::Zero2,
                0b011000 => Self::Zero3,
                0b100000 => Self::Zero4,
                0b100100 => Self::Zero5,
                0b101000 => Self::Zero6,
                0b101100 => Self::Zero7,
                0b101111 => Self::Zero8,
                0b111000 => Self::Zero9,
                0b001001 => Self::NegOne0,
                0b011001 => Self::NegOne1,
                0b100001 => Self::NegOne2,
                0b100101 => Self::NegOne3,
                0b101001 => Self::NegOne4,
                0b101011 => Self::NegOne5,
                0b101101 => Self::NegOne6,
                0b101110 => Self::NegOne7,
                0b111001 => Self::NegOne8,
                0b111100 => Self::NegOne9,
                0b001010 => Self::D0,
                0b011011 => Self::D1,
                0b011101 => Self::D2,
                0b100010 => Self::A0,
                0b100111 => Self::A1,
                0b110101 => Self::A2,
                0b001011 => Self::NotD0,
                0b011100 => Self::NotD1,
                0b011010 => Self::NotD2,
                0b100011 => Self::NotA0,
                0b100110 => Self::NotA1,
                0b110100 => Self::NotA2,
                0b000001 => Self::NotDOrNotA,
                0b000011 => Self::NotOfDPlusA,
                0b000100 => Self::DAndNotA,
                0b000101 => Self::NotDOrA,
                0b000110 => Self::DPlusNotA,
                0b010000 => Self::NotDAndA,
                0b010001 => Self::DOrNotA,
                0b010010 => Self::NotDPlusA,
                0b010100 => Self::NotDAndNotA,
                0b010110 => Self::NotDPlusNotA,
                0b011110 => Self::NotDMinus1,
                0b010111 => Self::NotNotDPlusNotA,
                0b110110 => Self::NotAMinus1,
                0b111110 => Self::NegTwo,
                _ => {
                    ::core::panicking::panic_fmt(format_args!("CBits: Unhandled value"));
                }
            }
        }
    }
    /// This bit controls the input to the Hack ALU.
    ///
    /// If not set, the value in the `A` register is passed in as the second input.
    ///
    /// If set, the value at `Memory[A]` is passed in instead.
    pub enum Mode {
        A = 0,
        M = 1,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Mode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Mode::A => "A",
                    Mode::M => "M",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Mode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Mode {
        #[inline]
        fn eq(&self, other: &Mode) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Mode {}
    #[automatically_derived]
    impl ::core::clone::Clone for Mode {
        #[inline]
        fn clone(&self) -> Mode {
            *self
        }
    }
    impl Mode {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 1usize> {
            arbitrary_int::UInt::<u8, 1usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// As the enum is exhaustive, this function will always return a valid result
        pub const fn new_with_raw_value(value: arbitrary_int::UInt<u8, 1usize>) -> Self {
            match value.value() {
                0 => Self::A,
                1 => Self::M,
                _ => {
                    ::core::panicking::panic_fmt(format_args!("Mode: Unhandled value"));
                }
            }
        }
    }
    impl std::fmt::Display for Mode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Mode::A => f.write_fmt(format_args!("A")),
                Mode::M => f.write_fmt(format_args!("M")),
            }
        }
    }
    #[repr(C)]
    /// The combination of the address bit and C-bits, which determines the Computation portion of a C instruction.
    ///
    /// Not all configurations are valid, those that are can be found in the `ValidComp` enum.
    /// If the bit configuration is valid, it can be accessed with the `comp()` method.
    ///
    /// However, all bit configurations and their resulting computations are mapped by the `Mode` and `CBits` enums,
    /// and are accessible through the `mode()` and `c_bits()` methods.
    pub(crate) struct Comp {
        raw_value: u8,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Comp {}
    #[automatically_derived]
    impl ::core::clone::Clone for Comp {
        #[inline]
        fn clone(&self) -> Comp {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    impl Comp {
        /// Returns the underlying raw value of this bitfield
        #[inline]
        pub const fn raw_value(&self) -> u7 {
            u7::extract_u8(self.raw_value, 0)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// No checks are performed on the value, so it is possible to set bits that don't have any
        /// accessors specified.
        #[inline]
        pub const fn new_with_raw_value(value: u7) -> Comp {
            Comp { raw_value: value.value() }
        }
        #[inline]
        pub const fn comp(&self) -> Result<ValidComp, u8> {
            let extracted_bits = arbitrary_int::u7::extract_u8(self.raw_value, 0usize);
            ValidComp::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn with_comp(&self, field_value: ValidComp) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u8 << 7usize) - 1u8) << 0usize))
                    | ((field_value.raw_value().value() as u8) << 0usize),
            }
        }
        #[inline]
        pub const fn mode(&self) -> Mode {
            let extracted_bits = arbitrary_int::u1::extract_u8(self.raw_value, 6usize);
            Mode::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn with_mode(&self, field_value: Mode) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u8 << 1usize) - 1u8) << 6usize))
                    | ((field_value.raw_value().value() as u8) << 6usize),
            }
        }
        #[inline]
        pub const fn c_bits(&self) -> CBits {
            let extracted_bits = arbitrary_int::u6::extract_u8(self.raw_value, 0usize);
            CBits::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn with_c_bits(&self, field_value: CBits) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u8 << 6usize) - 1u8) << 0usize))
                    | ((field_value.raw_value().value() as u8) << 0usize),
            }
        }
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
                | C::Zero9 => f.write_fmt(format_args!("0")),
                C::One => f.write_fmt(format_args!("1")),
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
                | C::NegOne9 => f.write_fmt(format_args!("-1")),
                C::D | C::D0 | C::D1 | C::D2 => f.write_fmt(format_args!("D")),
                C::A | C::A0 | C::A1 | C::A2 => {
                    f.write_fmt(format_args!("{0}", self.mode()))
                }
                C::NotD | C::NotD0 | C::NotD1 | C::NotD2 => {
                    f.write_fmt(format_args!("!D"))
                }
                C::NotA | C::NotA0 | C::NotA1 | C::NotA2 => {
                    f.write_fmt(format_args!("!{0}", self.mode()))
                }
                C::NegD => f.write_fmt(format_args!("-D")),
                C::NegA => f.write_fmt(format_args!("-{0}", self.mode())),
                C::DPlusOne => f.write_fmt(format_args!("D+1")),
                C::APlusOne => f.write_fmt(format_args!("{0}+1", self.mode())),
                C::DMinusOne => f.write_fmt(format_args!("D-1")),
                C::AMinusOne => f.write_fmt(format_args!("{0}-1", self.mode())),
                C::DPlusA => f.write_fmt(format_args!("D+{0}", self.mode())),
                C::DMinusA => f.write_fmt(format_args!("D-{0}", self.mode())),
                C::AMinusD => f.write_fmt(format_args!("{0}-D", self.mode())),
                C::DAndA => f.write_fmt(format_args!("D&{0}", self.mode())),
                C::DOrA => f.write_fmt(format_args!("D|{0}", self.mode())),
                _ => Err(std::fmt::Error),
            }
        }
    }
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Jump {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Jump::Never => "Never",
                    Jump::JGT => "JGT",
                    Jump::JEQ => "JEQ",
                    Jump::JGE => "JGE",
                    Jump::JLT => "JLT",
                    Jump::JNE => "JNE",
                    Jump::JLE => "JLE",
                    Jump::JMP => "JMP",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Jump {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Jump {
        #[inline]
        fn eq(&self, other: &Jump) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Jump {}
    #[automatically_derived]
    impl ::core::clone::Clone for Jump {
        #[inline]
        fn clone(&self) -> Jump {
            *self
        }
    }
    impl Jump {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 3usize> {
            arbitrary_int::UInt::<u8, 3usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// As the enum is exhaustive, this function will always return a valid result
        pub const fn new_with_raw_value(value: arbitrary_int::UInt<u8, 3usize>) -> Self {
            match value.value() {
                0 => Self::Never,
                0b001 => Self::JGT,
                0b010 => Self::JEQ,
                0b011 => Self::JGE,
                0b100 => Self::JLT,
                0b101 => Self::JNE,
                0b110 => Self::JLE,
                0b111 => Self::JMP,
                _ => {
                    ::core::panicking::panic_fmt(format_args!("Jump: Unhandled value"));
                }
            }
        }
    }
    impl std::fmt::Display for Jump {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Jump::Never => Ok(()),
                Jump::JGT => f.write_fmt(format_args!("JGT")),
                Jump::JEQ => f.write_fmt(format_args!("JEQ")),
                Jump::JGE => f.write_fmt(format_args!("JGE")),
                Jump::JLT => f.write_fmt(format_args!("JLT")),
                Jump::JNE => f.write_fmt(format_args!("JNE")),
                Jump::JLE => f.write_fmt(format_args!("JLE")),
                Jump::JMP => f.write_fmt(format_args!("JMP")),
            }
        }
    }
    /// These enums are for specifying valid bit configurations for Hack instruction types
    /// All C instructions have the 3 most significant bits set, and all A instructions have the most significant bit clear
    pub enum NonAInst {
        C = 0b111,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for NonAInst {}
    #[automatically_derived]
    impl ::core::clone::Clone for NonAInst {
        #[inline]
        fn clone(&self) -> NonAInst {
            *self
        }
    }
    impl NonAInst {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 3usize> {
            arbitrary_int::UInt::<u8, 3usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value, or
        /// Err(value) if the value does not exist in the enum.
        pub const fn new_with_raw_value(
            value: arbitrary_int::UInt<u8, 3usize>,
        ) -> Result<Self, u8> {
            match value.value() {
                0b111 => Ok(Self::C),
                _ => Err(value.value()),
            }
        }
    }
    /// These enums are for specifying valid bit configurations for Hack instruction types
    /// All C instructions have the 3 most significant bits set, and all A instructions have the most significant bit clear
    pub enum AInst {
        A = 0,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AInst {}
    #[automatically_derived]
    impl ::core::clone::Clone for AInst {
        #[inline]
        fn clone(&self) -> AInst {
            *self
        }
    }
    impl AInst {
        /// Returns the underlying raw value of this bitfield
        pub const fn raw_value(self) -> arbitrary_int::UInt<u8, 1usize> {
            arbitrary_int::UInt::<u8, 1usize>::new(self as u8)
        }
        /// Creates a new instance of this bitfield with the given raw value, or
        /// Err(value) if the value does not exist in the enum.
        pub const fn new_with_raw_value(
            value: arbitrary_int::UInt<u8, 1usize>,
        ) -> Result<Self, u8> {
            match value.value() {
                0 => Ok(Self::A),
                _ => Err(value.value()),
            }
        }
    }
    #[repr(C)]
    pub struct CInstruction {
        raw_value: u16,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CInstruction {}
    #[automatically_derived]
    impl ::core::clone::Clone for CInstruction {
        #[inline]
        fn clone(&self) -> CInstruction {
            let _: ::core::clone::AssertParamIsClone<u16>;
            *self
        }
    }
    impl CInstruction {
        const DEFAULT_RAW_VALUE: u15 = u15::new(0b110_0000_0000_0000);
        ///An instance that uses the default value 0b110_0000_0000_0000
        #[inline]
        pub const DEFAULT: Self = Self::new_with_raw_value(Self::DEFAULT_RAW_VALUE);
        #[deprecated(
            note = "Use CInstruction::Default (or CInstruction::DEFAULT in const context) instead"
        )]
        pub const fn new() -> Self {
            Self::DEFAULT
        }
        /// Returns the underlying raw value of this bitfield
        #[inline]
        pub const fn raw_value(&self) -> u15 {
            u15::extract_u16(self.raw_value, 0)
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// No checks are performed on the value, so it is possible to set bits that don't have any
        /// accessors specified.
        #[inline]
        pub const fn new_with_raw_value(value: u15) -> CInstruction {
            CInstruction {
                raw_value: value.value(),
            }
        }
        /// The computation bits of a C-Instruction (bits 6-12)
        #[inline]
        pub const fn comp(&self) -> Comp {
            let extracted_bits = arbitrary_int::u7::extract_u16(self.raw_value, 6usize);
            Comp::new_with_raw_value(extracted_bits)
        }
        /// The computation bits of a C-Instruction (bits 6-12)
        #[inline]
        pub const fn with_comp(&self, field_value: Comp) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u16 << 7usize) - 1u16) << 6usize))
                    | ((field_value.raw_value().value() as u16) << 6usize),
            }
        }
        /// The destination bits of a C-Instruction (bits 3, 4, and 5).
        #[inline]
        pub const fn dest(&self) -> DestBits {
            let extracted_bits = arbitrary_int::u3::extract_u16(self.raw_value, 3usize);
            DestBits::new_with_raw_value(extracted_bits)
        }
        /// The destination bits of a C-Instruction (bits 3, 4, and 5).
        #[inline]
        pub const fn with_dest(&self, field_value: DestBits) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u16 << 3usize) - 1u16) << 3usize))
                    | ((field_value.raw_value().value() as u16) << 3usize),
            }
        }
        /// The `jump` portion of a C-instruction (bits 0, 1, and 2)
        #[inline]
        pub const fn jump(&self) -> Jump {
            let extracted_bits = arbitrary_int::u3::extract_u16(self.raw_value, 0usize);
            Jump::new_with_raw_value(extracted_bits)
        }
        /// The `jump` portion of a C-instruction (bits 0, 1, and 2)
        #[inline]
        pub const fn with_jump(&self, field_value: Jump) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u16 << 3usize) - 1u16) << 0usize))
                    | ((field_value.raw_value().value() as u16) << 0usize),
            }
        }
        /// Bit 2 of a C-Instruction
        ///
        /// If this flag is set, jump to the address in register `A` if `comp < 0`
        #[inline]
        pub const fn jlt(&self) -> bool {
            (self.raw_value & (1u16 << 2usize)) != 0
        }
        /// Bit 1 of a C-Instruction
        ///
        /// If this flag is set, jump to the address in register `A` if `comp == 0`
        #[inline]
        pub const fn jeq(&self) -> bool {
            (self.raw_value & (1u16 << 1usize)) != 0
        }
        /// Bit 0 of a C-Instruction
        ///
        /// If this flag is set, jump to the address in register `A` if `comp > 0`
        #[inline]
        pub const fn jgt(&self) -> bool {
            (self.raw_value & (1u16 << 0usize)) != 0
        }
    }
    impl Default for CInstruction {
        fn default() -> Self {
            Self::DEFAULT
        }
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
    #[repr(C)]
    /// Struct representing a Hack instruction.
    ///
    /// Default value is `0b111_0_101010_000_000`, or the C-Instruction `0`, which like all C-Instructions with no `dest` or `jump` bits set, is a no-op.
    pub struct Instruction {
        raw_value: u16,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Instruction {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Instruction",
                "raw_value",
                &&self.raw_value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Instruction {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Instruction {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u16>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Instruction {}
    #[automatically_derived]
    impl ::core::clone::Clone for Instruction {
        #[inline]
        fn clone(&self) -> Instruction {
            let _: ::core::clone::AssertParamIsClone<u16>;
            *self
        }
    }
    impl Instruction {
        const DEFAULT_RAW_VALUE: u16 = 0b111_0_101010_000_000;
        ///An instance that uses the default value 0b111_0_101010_000_000
        #[inline]
        pub const DEFAULT: Self = Self::new_with_raw_value(Self::DEFAULT_RAW_VALUE);
        #[deprecated(
            note = "Use Instruction::Default (or Instruction::DEFAULT in const context) instead"
        )]
        pub const fn new() -> Self {
            Self::DEFAULT
        }
        /// Returns the underlying raw value of this bitfield
        #[inline]
        pub const fn raw_value(&self) -> u16 {
            self.raw_value
        }
        /// Creates a new instance of this bitfield with the given raw value.
        ///
        /// No checks are performed on the value, so it is possible to set bits that don't have any
        /// accessors specified.
        #[inline]
        pub const fn new_with_raw_value(value: u16) -> Instruction {
            Instruction { raw_value: value }
        }
        #[inline]
        pub const fn a_inst(&self) -> Result<AInst, u8> {
            let extracted_bits = arbitrary_int::u1::extract_u16(self.raw_value, 15usize);
            AInst::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn non_a_inst(&self) -> Result<NonAInst, u8> {
            let extracted_bits = arbitrary_int::u3::extract_u16(self.raw_value, 13usize);
            NonAInst::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn addr(&self) -> u15 {
            arbitrary_int::u15::extract_u16(self.raw_value, 0usize)
        }
        #[inline]
        pub const fn with_addr(&self, field_value: u15) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u16 << 15usize) - 1u16) << 0usize))
                    | ((field_value.value() as u16) << 0usize),
            }
        }
        #[inline]
        pub const fn c_inst(&self) -> CInstruction {
            let extracted_bits = arbitrary_int::u15::extract_u16(self.raw_value, 0usize);
            CInstruction::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn with_c_inst(&self, field_value: CInstruction) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u16 << 15usize) - 1u16) << 0usize))
                    | ((field_value.raw_value().value() as u16) << 0usize),
            }
        }
    }
    impl Default for Instruction {
        fn default() -> Self {
            Self::DEFAULT
        }
    }
    impl Instruction {
        #[inline]
        const fn is_ok(&self) -> bool {
            self.a_inst().is_ok() || self.non_a_inst().is_ok()
        }
        #[inline]
        /// Creates a new C instruction with the given `dest`, `comp`, and `jump` segments.
        pub(crate) const fn c(dest: Dest, comp: ValidComp, jump: Jump) -> Self {
            Instruction::DEFAULT
                .with_c_inst(
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
                _ => ::core::panicking::panic("not yet implemented"),
            }
        }
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
        pub const MAX: Self = Self { raw_value: i16::MAX as u16 };
    }
    impl PartialEq for Instruction {
        fn eq(&self, other: &Self) -> bool {
            self.raw_value == other.raw_value
        }
    }
    impl From<i16> for Instruction {
        #[allow(overflowing_literals)]
        fn from(value: i16) -> Self {
            Self { raw_value: value as u16 }
        }
    }
    impl std::fmt::Display for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.a_inst().is_ok() {
                f.write_fmt(format_args!("@{0}", self.addr()))
            } else if self.non_a_inst().is_ok() {
                let c = self.c_inst();
                match c.dest().get() {
                    Dest::None => Ok(()),
                    d => f.write_fmt(format_args!("{0}=", d)),
                }?;
                f.write_fmt(format_args!("{0}", c.comp()))?;
                match c.jump() {
                    Jump::Never => Ok(()),
                    j => f.write_fmt(format_args!(";{0}", j)),
                }
            } else {
                Err(std::fmt::Error)
            }
        }
    }
    impl std::fmt::UpperHex for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0:04X}", self.raw_value))
        }
    }
    impl std::fmt::LowerHex for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0:04x}", self.raw_value))
        }
    }
    impl std::fmt::Binary for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0:016b}", self.raw_value))
        }
    }
    impl std::fmt::Octal for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0:06o}", self.raw_value))
        }
    }
    /// Represents a line of valid Hack assembly language.
    pub enum Asm<'a> {
        Comment(Cow<'a, str>),
        Label(Cow<'a, str>),
        At(Cow<'a, str>),
        Asm(Instruction),
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Asm<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Asm::Comment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Comment",
                        &__self_0,
                    )
                }
                Asm::Label(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Label",
                        &__self_0,
                    )
                }
                Asm::At(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "At", &__self_0)
                }
                Asm::Asm(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Asm",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Asm<'a> {
        #[inline]
        fn clone(&self) -> Asm<'a> {
            match self {
                Asm::Comment(__self_0) => {
                    Asm::Comment(::core::clone::Clone::clone(__self_0))
                }
                Asm::Label(__self_0) => Asm::Label(::core::clone::Clone::clone(__self_0)),
                Asm::At(__self_0) => Asm::At(::core::clone::Clone::clone(__self_0)),
                Asm::Asm(__self_0) => Asm::Asm(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Asm<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Asm<'a> {
        #[inline]
        fn eq(&self, other: &Asm<'a>) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Asm::Comment(__self_0), Asm::Comment(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Asm::Label(__self_0), Asm::Label(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Asm::At(__self_0), Asm::At(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Asm::Asm(__self_0), Asm::Asm(__arg1_0)) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralEq for Asm<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for Asm<'a> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Cow<'a, str>>;
            let _: ::core::cmp::AssertParamIsEq<Cow<'a, str>>;
            let _: ::core::cmp::AssertParamIsEq<Cow<'a, str>>;
            let _: ::core::cmp::AssertParamIsEq<Instruction>;
        }
    }
    trait Str<'a>: Into<Cow<'a, str>> {}
    impl<'a> Str<'a> for &'a str {}
    impl Str<'_> for String {}
    impl<'a> Str<'a> for Cow<'a, str> {}
    impl<'a, T: Str<'a>> From<T> for Asm<'a> {
        fn from(value: T) -> Self {
            Self::At(value.into())
        }
    }
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
    impl std::fmt::Display for Asm<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Asm::Comment(s) => {
                    if !s.is_empty() {
                        f.write_fmt(format_args!("// {0}", s))
                    } else {
                        Ok(())
                    }
                }
                Asm::Label(l) => f.write_fmt(format_args!("({0})", l)),
                Asm::At(a) => f.write_fmt(format_args!("@{0}", a)),
                Asm::Asm(i) => f.write_fmt(format_args!("{0}", i)),
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
        /// The 1bpp screen is displayed least significant bit to most significant bit from left to right
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
                var_counter: 15,
            }
        }
        #[inline]
        fn get_label(&mut self, label: &str) -> Option<i16> {
            match label {
                "SP" => Some(0),
                "LCL" => Some(1),
                "ARG" => Some(2),
                "THIS" => Some(3),
                "THAT" => Some(4),
                "R0" => Some(0),
                "R1" => Some(1),
                "R2" => Some(2),
                "R3" => Some(3),
                "R4" => Some(4),
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
            let mut comp_start = 0;
            let mut comp_end = input.len();
            let dest = if let Some(i) = input.find('=') {
                comp_start = i + 1;
                let dest = &input[..i];
                if !dest.chars().all(|c| "AMD".contains(c)) {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "\'{0}\' contains an invalid destination character.", dest
                            ),
                        );
                        error
                    });
                }
                Dest::from_flags(
                    dest.contains('A'),
                    dest.contains('M'),
                    dest.contains('D'),
                )
            } else {
                Dest::None
            };
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
                    _ => {
                        return ::anyhow::__private::Err({
                            let error = ::anyhow::__private::format_err(
                                format_args!("Semicolon requires a valid jump command!"),
                            );
                            error
                        });
                    }
                }
            } else {
                Jump::Never
            };
            let comp = match &input[comp_start..comp_end] {
                "0" => ValidComp::Zero,
                "1" => ValidComp::One,
                "-1" => ValidComp::NegOne,
                "D" => ValidComp::D,
                "A" => ValidComp::A,
                "M" => ValidComp::M,
                "!D" | "~D" => ValidComp::NotD,
                "!A" | "~A" => ValidComp::NotA,
                "!M" | "~M" => ValidComp::NotM,
                "D+A" | "A+D" => ValidComp::DPlusA,
                "D+M" | "M+D" => ValidComp::DPlusM,
                "D&A" | "A&D" => ValidComp::DAndA,
                "D&M" | "M&D" => ValidComp::DAndM,
                "D|A" | "A|D" => ValidComp::DOrA,
                "D|M" | "M|D" => ValidComp::DOrM,
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
                _ => {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("invalid or unsupported computation field"),
                        );
                        error
                    });
                }
            };
            Ok(Asm::Asm(Instruction::c(dest, comp, jump)))
        }
        pub fn translate<'a>(&mut self, input: &'a str) -> Result<Asm<'a>> {
            if let Some(i) = input.strip_prefix('@') {
                Ok(self.parse_a_instruction(i))
            } else {
                self.parse_c_instruction(input)
            }
        }
        pub fn assemble(&mut self, asm: &[Asm]) -> Vec<Instruction> {
            let mut line: i16 = 0;
            for com in asm {
                if let Asm::Label(s) = com {
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
}
mod vm {
    pub mod translator {
        use std::borrow::Cow;
        use std::fmt::Display;
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::path::{Path, PathBuf};
        use std::vec;
        use anyhow::{anyhow, bail, Result};
        use super::{Comparison as Cmp, MemSegment as Seg, VmCommand};
        use crate::asm::{Asm, Instruction, Mode};
        use asm_macro::asm;
        pub fn parse(cmd: &str) -> Result<VmCommand> {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let command = match parts.len() {
                1 => {
                    match parts[0] {
                        "add" => VmCommand::Add,
                        "sub" => VmCommand::Sub,
                        "neg" => VmCommand::Neg,
                        "eq" => VmCommand::Compare(Cmp::EQ),
                        "gt" => VmCommand::Compare(Cmp::GT),
                        "lt" => VmCommand::Compare(Cmp::LT),
                        "and" => VmCommand::And,
                        "or" => VmCommand::Or,
                        "not" => VmCommand::Not,
                        "return" => VmCommand::Return,
                        _ => {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("No one word command \"{0}\"", cmd),
                                );
                                error
                            });
                        }
                    }
                }
                2 => {
                    match parts[0] {
                        "label" => VmCommand::Label(parts[1]),
                        "goto" => VmCommand::Goto(parts[1]),
                        "if-goto" => VmCommand::IfGoto(parts[1]),
                        _ => {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("No two word command \"{0}\"", cmd),
                                );
                                error
                            });
                        }
                    }
                }
                3 => {
                    let arg = parts[2]
                        .parse::<i16>()
                        .map_err(|_| ::anyhow::Error::msg({
                            let res = ::alloc::fmt::format(
                                format_args!("{0} is not a valid 16 bit integer", parts[2]),
                            );
                            res
                        }))?;
                    match (parts[0], parts[1]) {
                        ("push", "local") => VmCommand::Push(Seg::Local, arg),
                        ("pop", "local") => VmCommand::Pop(Seg::Local, arg),
                        ("push", "argument") => VmCommand::Push(Seg::Argument, arg),
                        ("pop", "argument") => VmCommand::Pop(Seg::Argument, arg),
                        ("push", "this") => VmCommand::Push(Seg::This, arg),
                        ("pop", "this") => VmCommand::Pop(Seg::This, arg),
                        ("push", "that") => VmCommand::Push(Seg::That, arg),
                        ("pop", "that") => VmCommand::Pop(Seg::That, arg),
                        ("push", "constant") => VmCommand::Push(Seg::Constant, arg),
                        ("push", "static") => VmCommand::Push(Seg::Static, arg),
                        ("pop", "static") => VmCommand::Pop(Seg::Static, arg),
                        ("push", "pointer") => VmCommand::Push(Seg::Pointer, arg),
                        ("pop", "pointer") => VmCommand::Pop(Seg::Pointer, arg),
                        ("push", "temp") => VmCommand::Push(Seg::Temp, arg),
                        ("pop", "temp") => VmCommand::Pop(Seg::Temp, arg),
                        ("function", _) => VmCommand::Function(parts[1], arg),
                        ("call", _) => VmCommand::Call(parts[1], arg),
                        _ => {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("No three word command \"{0}\"", cmd),
                                );
                                error
                            });
                        }
                    }
                }
                _ => {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("\"{0}\" is not a valid VM command", cmd),
                        );
                        error
                    });
                }
            };
            Ok(command)
        }
        fn translate_vm(bootstrap: bool) -> Result<()> {
            let args: Vec<String> = std::env::args().collect();
            let mut files: Vec<PathBuf> = ::alloc::vec::Vec::new();
            let file_path = Path::new(&args[1]);
            let filename = file_path.file_stem().unwrap().to_str().unwrap();
            if file_path.is_dir() {
                for entry in file_path.read_dir().unwrap() {
                    if let Some(x) = entry.as_ref().unwrap().path().extension() {
                        if x.to_str().unwrap() == "vm" {
                            files.push(entry.as_ref().unwrap().path())
                        }
                    }
                }
            } else if let Some("vm") = file_path.extension().unwrap().to_str() {
                files.push(file_path.to_path_buf())
            }
            let mut writer = VmTranslator::new(filename, bootstrap);
            for file in files {
                if let Ok(f) = File::open(&file) {
                    writer.set_filename(file.file_stem().unwrap().to_str().unwrap());
                    let reader = BufReader::new(f);
                    for line in reader.lines().flatten() {
                        let cmd = line
                            .find("//")
                            .map(|i| &line[..i])
                            .unwrap_or(&line)
                            .trim()
                            .to_string();
                        if !cmd.is_empty() {
                            let vm_cmd = parse(&cmd).expect("could not parse command");
                            writer.generate_asm(vm_cmd, true)?;
                        }
                    }
                }
            }
            Ok(())
        }
        struct VmTranslator<'a> {
            filename: String,
            curr_func: String,
            comp_count: i16,
            call_count: i16,
            return_written: bool,
            asm: Vec<Asm<'a>>,
        }
        impl<'a> VmTranslator<'a> {
            pub fn new(filename: &str, bootstrap: bool) -> Self {
                let asm = if bootstrap {
                    Vec::from([
                        crate::asm::Asm::Asm(Instruction::from(256)),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::A,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Comment(
                            std::borrow::Cow::Borrowed("call Sys.init"),
                        ),
                        crate::asm::Asm::At(std::borrow::Cow::Borrowed("Sys.init")),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::None,
                                crate::asm::ValidComp::Zero,
                                crate::asm::Jump::JMP,
                            ),
                        ),
                    ])
                } else {
                    ::alloc::vec::Vec::new()
                };
                Self {
                    filename: filename.to_string(),
                    curr_func: {
                        let res = ::alloc::fmt::format(format_args!("${0}$", filename));
                        res
                    },
                    comp_count: 0,
                    call_count: 0,
                    return_written: false,
                    asm,
                }
            }
            fn set_filename(&mut self, filename: &str) {
                self.filename = filename.to_string();
            }
            /// Naively generates assembly on demand per VM Command.
            #[rustfmt::skip]
            fn generate_asm(&mut self, command: VmCommand, comment: bool) -> Result<()> {
                if comment {
                    self.asm
                        .push(
                            crate::asm::Asm::Comment(
                                std::borrow::Cow::Owned({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}", command),
                                    );
                                    res
                                }),
                            ),
                        );
                }
                match command {
                    VmCommand::Add => {
                        self.binary_op(
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::M,
                                    crate::asm::ValidComp::DPlusM,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        )
                    }
                    VmCommand::Sub => {
                        self.binary_op(
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::M,
                                    crate::asm::ValidComp::MMinusD,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        )
                    }
                    VmCommand::Neg => {
                        self.unary_op(
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::M,
                                    crate::asm::ValidComp::NegM,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        )
                    }
                    VmCommand::Compare(comp) => self.comparison(comp),
                    VmCommand::And => {
                        self.binary_op(
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::M,
                                    crate::asm::ValidComp::DAndM,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        )
                    }
                    VmCommand::Or => {
                        self.binary_op(
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::M,
                                    crate::asm::ValidComp::DOrM,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        )
                    }
                    VmCommand::Not => {
                        self.unary_op(
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::M,
                                    crate::asm::ValidComp::NotM,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        )
                    }
                    VmCommand::Push(seg, n) => {
                        match seg {
                            Seg::Argument => self.push_segment("ARG", n),
                            Seg::Local => self.push_segment("LCL", n),
                            Seg::This => self.push_segment("THIS", n),
                            Seg::That => self.push_segment("THAT", n),
                            Seg::Static => {
                                self.push_value(
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0}.{1}", self.filename, n),
                                        );
                                        res
                                    },
                                    Mode::M,
                                )
                            }
                            Seg::Pointer => {
                                self.push_value(
                                    if n == 0 { "THIS" } else { "THAT" },
                                    Mode::M,
                                )
                            }
                            Seg::Temp => {
                                self.push_value(
                                    crate::asm::Asm::At(
                                        std::borrow::Cow::Owned({
                                            let res = ::alloc::fmt::format(format_args!("R{0}", n));
                                            res
                                        }),
                                    ),
                                    Mode::M,
                                )
                            }
                            Seg::Constant => self.push_constant(n),
                        }
                    }
                    VmCommand::Pop(seg, n) => {
                        match seg {
                            Seg::Argument => self.pop_segment("ARG", n),
                            Seg::Local => self.pop_segment("LCL", n),
                            Seg::This => self.pop_segment("THIS", n),
                            Seg::That => self.pop_segment("THAT", n),
                            Seg::Static => {
                                self.pop_value({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}.{1}", self.filename, n),
                                    );
                                    res
                                })
                            }
                            Seg::Pointer => {
                                self.pop_value(if n == 0 { "THIS" } else { "THAT" })
                            }
                            Seg::Temp => {
                                self.pop_value(
                                    Asm::At(
                                        Cow::Owned({
                                            let res = ::alloc::fmt::format(format_args!("R{0}", n + 5));
                                            res
                                        }),
                                    ),
                                )
                            }
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("cannot pop to constant"),
                                    );
                                    error
                                });
                            }
                        }
                    }
                    VmCommand::Label(l) => {
                        self.def_label({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}${1}", self.curr_func, l),
                            );
                            res
                        })
                    }
                    VmCommand::Goto(l) => {
                        self.goto({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}${1}", self.curr_func, l),
                            );
                            res
                        })
                    }
                    VmCommand::IfGoto(l) => {
                        self.if_goto({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}${1}", self.curr_func, l),
                            );
                            res
                        })
                    }
                    VmCommand::Function(f, n) => self.func(f, n),
                    VmCommand::Call(f, n) => self.call_func(f, n),
                    VmCommand::Return => {
                        if self.return_written {
                            self.asm
                                .extend([
                                    crate::asm::Asm::At(std::borrow::Cow::Borrowed("$$RETURN")),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::None,
                                            crate::asm::ValidComp::Zero,
                                            crate::asm::Jump::JMP,
                                        ),
                                    ),
                                ])
                        } else {
                            self.asm
                                .extend([
                                    crate::asm::Asm::Comment(
                                        std::borrow::Cow::Borrowed("Shared return subroutine"),
                                    ),
                                    crate::asm::Asm::Label(
                                        std::borrow::Cow::Borrowed("$$RETURN"),
                                    ),
                                    crate::asm::Asm::Comment(
                                        std::borrow::Cow::Borrowed(
                                            "Get the return address from 5 slots before the current local segment and store it in R14",
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(Instruction::from(5)),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::A,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::LCL,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::A,
                                            crate::asm::ValidComp::MMinusD,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::R14,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::SP,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::A,
                                            crate::asm::ValidComp::MMinusOne,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::ARG,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::A,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::APlusOne,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::SP,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::LCL,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::MMinusOne,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::R13,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::AM,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::THAT,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::R13,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::AM,
                                            crate::asm::ValidComp::MMinusOne,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::THIS,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::R13,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::AM,
                                            crate::asm::ValidComp::MMinusOne,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::ARG,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::R13,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::AM,
                                            crate::asm::ValidComp::MMinusOne,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::LCL,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::M,
                                            crate::asm::ValidComp::D,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Comment(std::borrow::Cow::Borrowed("")),
                                    crate::asm::Asm::R14,
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::A,
                                            crate::asm::ValidComp::M,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::None,
                                            crate::asm::ValidComp::Zero,
                                            crate::asm::Jump::JMP,
                                        ),
                                    ),
                                ])
                        }
                    }
                }
                Ok(())
            }
            fn unary_op(&mut self, last_line: Asm<'a>) {
                self.asm
                    .extend([
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::A,
                                crate::asm::ValidComp::MMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ]);
                self.asm.push(last_line);
            }
            fn comparison(&mut self, comparison: Cmp) {
                let counter = self.comp_count;
                self.comp_count += 1;
                self.binary_op(
                    crate::asm::Asm::Asm(
                        Instruction::c(
                            crate::asm::Dest::MD,
                            crate::asm::ValidComp::MMinusD,
                            crate::asm::Jump::Never,
                        ),
                    ),
                );
                self.asm
                    .extend(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                crate::asm::Asm::At(
                                    std::borrow::Cow::Owned({
                                        let res = ::alloc::fmt::format(
                                            format_args!("END_COMP{0}", counter),
                                        );
                                        res
                                    }),
                                ),
                                match comparison {
                                    Cmp::EQ => {
                                        crate::asm::Asm::Asm(
                                            Instruction::c(
                                                crate::asm::Dest::None,
                                                crate::asm::ValidComp::D,
                                                crate::asm::Jump::JNE,
                                            ),
                                        )
                                    }
                                    Cmp::GT => {
                                        crate::asm::Asm::Asm(
                                            Instruction::c(
                                                crate::asm::Dest::None,
                                                crate::asm::ValidComp::D,
                                                crate::asm::Jump::JLE,
                                            ),
                                        )
                                    }
                                    Cmp::LT => {
                                        crate::asm::Asm::Asm(
                                            Instruction::c(
                                                crate::asm::Dest::None,
                                                crate::asm::ValidComp::D,
                                                crate::asm::Jump::JGE,
                                            ),
                                        )
                                    }
                                    Cmp::LE => {
                                        crate::asm::Asm::Asm(
                                            Instruction::c(
                                                crate::asm::Dest::None,
                                                crate::asm::ValidComp::D,
                                                crate::asm::Jump::JGT,
                                            ),
                                        )
                                    }
                                    Cmp::GE => {
                                        crate::asm::Asm::Asm(
                                            Instruction::c(
                                                crate::asm::Dest::None,
                                                crate::asm::ValidComp::D,
                                                crate::asm::Jump::JLT,
                                            ),
                                        )
                                    }
                                    Cmp::NE => {
                                        crate::asm::Asm::Asm(
                                            Instruction::c(
                                                crate::asm::Dest::None,
                                                crate::asm::ValidComp::D,
                                                crate::asm::Jump::JEQ,
                                            ),
                                        )
                                    }
                                },
                            ]),
                        ),
                    );
                self.asm
                    .extend([
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::DPlusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Label(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("END_COMP{0}", counter),
                                );
                                res
                            }),
                        ),
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::A,
                                crate::asm::ValidComp::MMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::MMinusD,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ]);
            }
            fn binary_op(&mut self, last_line: Asm<'a>) {
                self.asm
                    .extend(
                        [
                            crate::asm::Asm::SP,
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::AM,
                                    crate::asm::ValidComp::MMinusOne,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::D,
                                    crate::asm::ValidComp::M,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                            crate::asm::Asm::Asm(
                                Instruction::c(
                                    crate::asm::Dest::A,
                                    crate::asm::ValidComp::AMinusOne,
                                    crate::asm::Jump::Never,
                                ),
                            ),
                        ]
                            .into_iter()
                            .chain(std::iter::once(last_line)),
                    );
            }
            pub fn push_segment<T: Display>(&mut self, segment: T, n: i16) {
                self.segment(segment, n);
                self.asm
                    .extend([
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::A,
                                crate::asm::ValidComp::DPlusM,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::M,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ]);
                self.push();
            }
            pub fn segment<T: Display>(&mut self, segment: T, n: i16) {
                self.asm
                    .extend([
                        Asm::from(n),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::A,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::At(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", segment),
                                );
                                res
                            }),
                        ),
                    ]);
            }
            pub fn pop_segment<T: Display>(&mut self, segment: T, n: i16) {
                self.segment(segment, n);
                self.asm
                    .extend([
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::DPlusM,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::AM,
                                crate::asm::ValidComp::MMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::DPlusM,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::A,
                                crate::asm::ValidComp::DMinusM,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::DMinusA,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ]);
            }
            fn push_value<T: Display + Into<Asm<'a>>>(&mut self, var: T, mode: Mode) {
                self.asm.push(var.into());
                self.asm
                    .push(
                        match mode {
                            Mode::A => {
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::D,
                                        crate::asm::ValidComp::A,
                                        crate::asm::Jump::Never,
                                    ),
                                )
                            }
                            Mode::M => {
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::D,
                                        crate::asm::ValidComp::M,
                                        crate::asm::Jump::Never,
                                    ),
                                )
                            }
                        },
                    );
                self.push();
            }
            fn push_constant(&mut self, var: i16) {
                match var {
                    v @ -1..=1 => {
                        self.asm
                            .push(
                                crate::asm::Asm::Comment(
                                    std::borrow::Cow::Owned({
                                        let res = ::alloc::fmt::format(
                                            format_args!("push constant {0}", v),
                                        );
                                        res
                                    }),
                                ),
                            );
                        self.push();
                        let idx = self.asm.len() - 1;
                        self
                            .asm[idx] = match v {
                            -1 => {
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::M,
                                        crate::asm::ValidComp::NegOne,
                                        crate::asm::Jump::Never,
                                    ),
                                )
                            }
                            0 => {
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::M,
                                        crate::asm::ValidComp::Zero,
                                        crate::asm::Jump::Never,
                                    ),
                                )
                            }
                            1 => {
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::M,
                                        crate::asm::ValidComp::One,
                                        crate::asm::Jump::Never,
                                    ),
                                )
                            }
                            _ => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        };
                    }
                    v => {
                        if var < 0 {
                            self.asm
                                .extend([
                                    Asm::from(!v),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::NotA,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                ])
                        } else {
                            self.asm
                                .extend([
                                    Asm::from(v),
                                    crate::asm::Asm::Asm(
                                        Instruction::c(
                                            crate::asm::Dest::D,
                                            crate::asm::ValidComp::A,
                                            crate::asm::Jump::Never,
                                        ),
                                    ),
                                ])
                        }
                        self.push();
                    }
                }
            }
            fn push(&mut self) {
                self.asm
                    .extend([
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::MPlusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::A,
                                crate::asm::ValidComp::MMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ])
            }
            fn pop_value<T: Display>(&mut self, var: T)
            where
                Asm<'a>: From<T>,
            {
                self.asm
                    .extend([
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::AM,
                                crate::asm::ValidComp::MMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::M,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::from(var),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ]);
            }
            fn def_label(&mut self, label: String) {
                self.asm.push(crate::asm::Asm::from(label));
            }
            fn goto(&mut self, label: String) {
                self.asm
                    .extend([
                        crate::asm::Asm::from(label),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::None,
                                crate::asm::ValidComp::Zero,
                                crate::asm::Jump::JMP,
                            ),
                        ),
                    ]);
            }
            fn if_goto(&mut self, label: String) {
                self.asm
                    .extend([
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::AM,
                                crate::asm::ValidComp::MMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::M,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::At(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(format_args!("{0}", label));
                                res
                            }),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::None,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::JNE,
                            ),
                        ),
                    ]);
            }
            fn func(&mut self, fn_name: &str, n_vars: i16) {
                self.curr_func = String::from(fn_name);
                self.asm
                    .extend([
                        crate::asm::Asm::Label(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", fn_name),
                                );
                                res
                            }),
                        ),
                        crate::asm::Asm::At(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(format_args!("{0}", n_vars));
                                res
                            }),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::A,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::AM,
                                crate::asm::ValidComp::DPlusM,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::DMinusOne,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Label(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}$LocalLoop", fn_name),
                                );
                                res
                            }),
                        ),
                        crate::asm::Asm::At(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}$LocalLoopEnd", fn_name),
                                );
                                res
                            }),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::None,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::JLT,
                            ),
                        ),
                        crate::asm::Asm::LCL,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::A,
                                crate::asm::ValidComp::DPlusM,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::Zero,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::At(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}$LocalLoop", fn_name),
                                );
                                res
                            }),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::DMinusOne,
                                crate::asm::Jump::JMP,
                            ),
                        ),
                        crate::asm::Asm::Label(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}$LocalLoopEnd", fn_name),
                                );
                                res
                            }),
                        ),
                    ]);
            }
            fn call_func(&mut self, function: &str, n_args: i16) {
                let return_label = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.ret${1}", self.filename, self.call_count),
                    );
                    res
                };
                self.call_count += 1;
                self.push_value("{return_label}", Mode::A);
                match n_args {
                    0 => {
                        self.asm
                            .extend([
                                crate::asm::Asm::R14,
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::M,
                                        crate::asm::ValidComp::Zero,
                                        crate::asm::Jump::Never,
                                    ),
                                ),
                            ])
                    }
                    1 => {
                        self.asm
                            .extend([
                                crate::asm::Asm::R14,
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::M,
                                        crate::asm::ValidComp::One,
                                        crate::asm::Jump::Never,
                                    ),
                                ),
                            ])
                    }
                    a => {
                        self.asm
                            .extend([
                                crate::asm::Asm::from(a),
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::D,
                                        crate::asm::ValidComp::A,
                                        crate::asm::Jump::Never,
                                    ),
                                ),
                                crate::asm::Asm::R14,
                                crate::asm::Asm::Asm(
                                    Instruction::c(
                                        crate::asm::Dest::M,
                                        crate::asm::ValidComp::D,
                                        crate::asm::Jump::Never,
                                    ),
                                ),
                            ])
                    }
                }
                self.asm
                    .extend([
                        crate::asm::Asm::from(n_args),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::A,
                                crate::asm::Jump::Never,
                            ),
                        ),
                    ]);
                self.asm
                    .push(crate::asm::Asm::Label(std::borrow::Cow::Borrowed("$$Call")));
                self.push_value("LCL", Mode::M);
                self.push_value("ARG", Mode::M);
                self.push_value("THIS", Mode::M);
                self.push_value("THAT", Mode::M);
                self.asm
                    .extend([
                        crate::asm::Asm::R14,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::M,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::Asm(Instruction::from(5)),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::DPlusA,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::MMinusD,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::ARG,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::SP,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::D,
                                crate::asm::ValidComp::M,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::LCL,
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::M,
                                crate::asm::ValidComp::D,
                                crate::asm::Jump::Never,
                            ),
                        ),
                        crate::asm::Asm::At(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", function),
                                );
                                res
                            }),
                        ),
                        crate::asm::Asm::Asm(
                            Instruction::c(
                                crate::asm::Dest::None,
                                crate::asm::ValidComp::Zero,
                                crate::asm::Jump::JMP,
                            ),
                        ),
                        crate::asm::Asm::Label(
                            std::borrow::Cow::Owned({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}", return_label),
                                );
                                res
                            }),
                        ),
                    ])
            }
        }
    }
    pub enum VmCommand<'a> {
        Add,
        Sub,
        Neg,
        Compare(Comparison),
        And,
        Or,
        Not,
        Push(MemSegment, i16),
        Pop(MemSegment, i16),
        Label(&'a str),
        Goto(&'a str),
        IfGoto(&'a str),
        Function(&'a str, i16),
        Call(&'a str, i16),
        Return,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for VmCommand<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                VmCommand::Add => ::core::fmt::Formatter::write_str(f, "Add"),
                VmCommand::Sub => ::core::fmt::Formatter::write_str(f, "Sub"),
                VmCommand::Neg => ::core::fmt::Formatter::write_str(f, "Neg"),
                VmCommand::Compare(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Compare",
                        &__self_0,
                    )
                }
                VmCommand::And => ::core::fmt::Formatter::write_str(f, "And"),
                VmCommand::Or => ::core::fmt::Formatter::write_str(f, "Or"),
                VmCommand::Not => ::core::fmt::Formatter::write_str(f, "Not"),
                VmCommand::Push(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Push",
                        __self_0,
                        &__self_1,
                    )
                }
                VmCommand::Pop(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Pop",
                        __self_0,
                        &__self_1,
                    )
                }
                VmCommand::Label(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Label",
                        &__self_0,
                    )
                }
                VmCommand::Goto(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Goto",
                        &__self_0,
                    )
                }
                VmCommand::IfGoto(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IfGoto",
                        &__self_0,
                    )
                }
                VmCommand::Function(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Function",
                        __self_0,
                        &__self_1,
                    )
                }
                VmCommand::Call(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Call",
                        __self_0,
                        &__self_1,
                    )
                }
                VmCommand::Return => ::core::fmt::Formatter::write_str(f, "Return"),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for VmCommand<'a> {
        #[inline]
        fn clone(&self) -> VmCommand<'a> {
            let _: ::core::clone::AssertParamIsClone<Comparison>;
            let _: ::core::clone::AssertParamIsClone<MemSegment>;
            let _: ::core::clone::AssertParamIsClone<i16>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            *self
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::Copy for VmCommand<'a> {}
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for VmCommand<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for VmCommand<'a> {
        #[inline]
        fn eq(&self, other: &VmCommand<'a>) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (VmCommand::Compare(__self_0), VmCommand::Compare(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        VmCommand::Push(__self_0, __self_1),
                        VmCommand::Push(__arg1_0, __arg1_1),
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (
                        VmCommand::Pop(__self_0, __self_1),
                        VmCommand::Pop(__arg1_0, __arg1_1),
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (VmCommand::Label(__self_0), VmCommand::Label(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (VmCommand::Goto(__self_0), VmCommand::Goto(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (VmCommand::IfGoto(__self_0), VmCommand::IfGoto(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        VmCommand::Function(__self_0, __self_1),
                        VmCommand::Function(__arg1_0, __arg1_1),
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (
                        VmCommand::Call(__self_0, __self_1),
                        VmCommand::Call(__arg1_0, __arg1_1),
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    _ => true,
                }
        }
    }
    pub enum MemSegment {
        Argument,
        Local,
        Static,
        Constant,
        This,
        That,
        Pointer,
        Temp,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MemSegment {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MemSegment::Argument => "Argument",
                    MemSegment::Local => "Local",
                    MemSegment::Static => "Static",
                    MemSegment::Constant => "Constant",
                    MemSegment::This => "This",
                    MemSegment::That => "That",
                    MemSegment::Pointer => "Pointer",
                    MemSegment::Temp => "Temp",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MemSegment {
        #[inline]
        fn clone(&self) -> MemSegment {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MemSegment {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MemSegment {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MemSegment {
        #[inline]
        fn eq(&self, other: &MemSegment) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    pub enum Comparison {
        EQ,
        GT,
        LT,
        LE,
        GE,
        NE,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comparison {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Comparison::EQ => "EQ",
                    Comparison::GT => "GT",
                    Comparison::LT => "LT",
                    Comparison::LE => "LE",
                    Comparison::GE => "GE",
                    Comparison::NE => "NE",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comparison {
        #[inline]
        fn clone(&self) -> Comparison {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Comparison {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Comparison {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Comparison {
        #[inline]
        fn eq(&self, other: &Comparison) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl std::fmt::Display for Comparison {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::EQ => f.write_fmt(format_args!("eq")),
                Self::GT => f.write_fmt(format_args!("gt")),
                Self::LT => f.write_fmt(format_args!("lt")),
                Self::LE => f.write_fmt(format_args!("le")),
                Self::GE => f.write_fmt(format_args!("ge")),
                Self::NE => f.write_fmt(format_args!("ne")),
            }
        }
    }
    impl std::fmt::Display for MemSegment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Local => f.write_fmt(format_args!("local")),
                Self::Argument => f.write_fmt(format_args!("argument")),
                Self::This => f.write_fmt(format_args!("this")),
                Self::That => f.write_fmt(format_args!("that")),
                Self::Constant => f.write_fmt(format_args!("constant")),
                Self::Static => f.write_fmt(format_args!("static")),
                Self::Pointer => f.write_fmt(format_args!("pointer")),
                Self::Temp => f.write_fmt(format_args!("temp")),
            }
        }
    }
    impl std::fmt::Display for VmCommand<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                VmCommand::Add => f.write_fmt(format_args!("add")),
                VmCommand::Sub => f.write_fmt(format_args!("sub")),
                VmCommand::Neg => f.write_fmt(format_args!("neg")),
                VmCommand::Compare(cmp) => f.write_fmt(format_args!("{0}", cmp)),
                VmCommand::And => f.write_fmt(format_args!("and")),
                VmCommand::Or => f.write_fmt(format_args!("or")),
                VmCommand::Not => f.write_fmt(format_args!("not")),
                VmCommand::Push(seg, arg) => {
                    f.write_fmt(format_args!("push {0} {1}", seg, arg))
                }
                VmCommand::Pop(seg, arg) => {
                    f.write_fmt(format_args!("pop {0} {1}", seg, arg))
                }
                VmCommand::Label(label) => f.write_fmt(format_args!("label {0}", label)),
                VmCommand::Goto(label) => f.write_fmt(format_args!("goto {0}", label)),
                VmCommand::IfGoto(label) => {
                    f.write_fmt(format_args!("if-goto {0}", label))
                }
                VmCommand::Function(func, n) => {
                    f.write_fmt(format_args!("function {0} {1}", func, n))
                }
                VmCommand::Call(func, n) => {
                    f.write_fmt(format_args!("call {0} {1}", func, n))
                }
                VmCommand::Return => f.write_fmt(format_args!("return")),
            }
        }
    }
}
use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};
pub struct ProgArgs {
    #[clap(subcommand)]
    pub sub_command: HackArgs,
}
#[automatically_derived]
impl ::core::fmt::Debug for ProgArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "ProgArgs",
            "sub_command",
            &&self.sub_command,
        )
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl clap::Parser for ProgArgs {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::CommandFactory for ProgArgs {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("hack_jack_suite");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("hack_jack_suite");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::FromArgMatches for ProgArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = ProgArgs {
            sub_command: {
                <HackArgs as clap::FromArgMatches>::from_arg_matches_mut(
                    __clap_arg_matches,
                )?
            },
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        {
            #[allow(non_snake_case)]
            let sub_command = &mut self.sub_command;
            <HackArgs as clap::FromArgMatches>::update_from_arg_matches_mut(
                sub_command,
                __clap_arg_matches,
            )?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::Args for ProgArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("ProgArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("ProgArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            let __clap_app = <HackArgs as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true);
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("ProgArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            let __clap_app = <HackArgs as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand_required(false)
                .arg_required_else_help(false);
            __clap_app
        }
    }
}
pub enum HackArgs {
    Compile(CompileArgs),
}
#[automatically_derived]
impl ::core::fmt::Debug for HackArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            HackArgs::Compile(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Compile",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::FromArgMatches for HackArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches))
            = __clap_arg_matches.remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "compile" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(
                    Self::Compile(
                        <CompileArgs as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "The subcommand \'{0}\' wasn\'t recognized", __clap_name
                            ),
                        );
                        res
                    },
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Compile(ref mut __clap_arg) if "compile" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::Subcommand for HackArgs {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("compile");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <CompileArgs as clap::Args>::augment_args(__clap_subcommand)
                };
                __clap_subcommand
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("compile");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <CompileArgs as clap::Args>::augment_args_for_update(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "compile" == __clap_name {
            return true;
        }
        false
    }
}
pub struct CompileArgs {
    /// Path to the file to be compiled
    pub path: PathBuf,
    pub vm: bool,
}
#[automatically_derived]
impl ::core::fmt::Debug for CompileArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CompileArgs",
            "path",
            &self.path,
            "vm",
            &&self.vm,
        )
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::FromArgMatches for CompileArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = CompileArgs {
            path: __clap_arg_matches
                .remove_one::<PathBuf>("path")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: path",
                ))?,
            vm: __clap_arg_matches
                .remove_one::<bool>("vm")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: vm",
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("path") {
            #[allow(non_snake_case)]
            let path = &mut self.path;
            *path = __clap_arg_matches
                .remove_one::<PathBuf>("path")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: path",
                ))?;
        }
        if __clap_arg_matches.contains_id("vm") {
            #[allow(non_snake_case)]
            let vm = &mut self.vm;
            *vm = __clap_arg_matches
                .remove_one::<bool>("vm")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: vm",
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
)]
#[automatically_derived]
impl clap::Args for CompileArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("CompileArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("CompileArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 2usize] = [
                                clap::Id::from("path"),
                                clap::Id::from("vm"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("path")
                        .value_name("PATH")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("Path to the file to be compiled")
                        .long_help(None);
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("vm")
                        .value_name("VM")
                        .required(true && clap::ArgAction::SetTrue.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                bool,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::SetTrue);
                    let arg = arg;
                    let arg = arg;
                    arg
                });
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("CompileArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 2usize] = [
                                clap::Id::from("path"),
                                clap::Id::from("vm"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("path")
                        .value_name("PATH")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("Path to the file to be compiled")
                        .long_help(None);
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("vm")
                        .value_name("VM")
                        .required(true && clap::ArgAction::SetTrue.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                bool,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::SetTrue);
                    let arg = arg;
                    let arg = arg.required(false);
                    arg
                });
            __clap_app
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<PathBuf> = ::alloc::vec::Vec::new();
    let file_path = Path::new(&args[1]);
    if file_path.is_dir() {
        for entry in file_path.read_dir().unwrap() {
            if let Some(x) = entry.as_ref().unwrap().path().extension() {
                if x.to_str().unwrap() == "jack" {
                    files.push(entry.as_ref().unwrap().path())
                }
            }
        }
    } else if let Some("jack") = file_path.extension().unwrap().to_str() {
        files.push(file_path.to_path_buf())
    }
}
