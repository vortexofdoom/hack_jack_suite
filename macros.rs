#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod asm {
    use anyhow::{bail, Result};
    use arbitrary_int::{u15, u3, u7};
    use bitbybit::{bitenum, bitfield};
    use std::collections::HashMap;
    use std::borrow::Cow;
    /// The destination bits of a Hack C-Instruction.
    ///
    /// Each letter corresponds to a register that the result of the computation segment will be placed into.
    pub enum Dest {
        None = 0,
        M = 1,
        D = 2,
        MD = 3,
        A = 4,
        AM = 5,
        AD = 6,
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
    struct DestFields {
        raw_value: u8,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DestFields {}
    #[automatically_derived]
    impl ::core::clone::Clone for DestFields {
        #[inline]
        fn clone(&self) -> DestFields {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    impl DestFields {
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
        pub const fn new_with_raw_value(value: u3) -> DestFields {
            DestFields {
                raw_value: value.value(),
            }
        }
        #[inline]
        pub const fn dest(&self) -> Dest {
            let extracted_bits = arbitrary_int::u3::extract_u8(self.raw_value, 0usize);
            Dest::new_with_raw_value(extracted_bits)
        }
        #[inline]
        pub const fn with_dest(&self, field_value: Dest) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u8 << 3usize) - 1u8) << 0usize))
                    | ((field_value.raw_value().value() as u8) << 0usize),
            }
        }
        #[inline]
        pub const fn m(&self) -> bool {
            (self.raw_value & (1u8 << 0usize)) != 0
        }
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
        #[inline]
        pub const fn d(&self) -> bool {
            (self.raw_value & (1u8 << 1usize)) != 0
        }
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
        #[inline]
        pub const fn a(&self) -> bool {
            (self.raw_value & (1u8 << 2usize)) != 0
        }
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
    impl Dest {
        pub fn new(a: bool, m: bool, d: bool) -> Self {
            DestFields {
                raw_value: (((a as u8) << 2) | ((d as u8) << 1) | m as u8),
            }
                .dest()
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
        pub const fn dest(&self) -> Dest {
            let extracted_bits = arbitrary_int::u3::extract_u16(self.raw_value, 3usize);
            Dest::new_with_raw_value(extracted_bits)
        }
        /// The destination bits of a C-Instruction (bits 3, 4, and 5).
        #[inline]
        pub const fn with_dest(&self, field_value: Dest) -> Self {
            Self {
                raw_value: (self.raw_value & !(((1u16 << 3usize) - 1u16) << 3usize))
                    | ((field_value.raw_value().value() as u16) << 3usize),
            }
        }
        /// Bit 5 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `A`
        #[inline]
        pub const fn dest_a(&self) -> bool {
            (self.raw_value & (1u16 << 5usize)) != 0
        }
        /// Bit 5 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `A`
        #[inline]
        pub const fn with_dest_a(&self, field_value: bool) -> Self {
            Self {
                raw_value: if field_value {
                    self.raw_value | (1u16 << 5usize)
                } else {
                    self.raw_value & !(1u16 << 5usize)
                },
            }
        }
        /// Bit 4 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `D`
        #[inline]
        pub const fn dest_d(&self) -> bool {
            (self.raw_value & (1u16 << 4usize)) != 0
        }
        /// Bit 4 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `D`
        #[inline]
        pub const fn with_dest_d(&self, field_value: bool) -> Self {
            Self {
                raw_value: if field_value {
                    self.raw_value | (1u16 << 4usize)
                } else {
                    self.raw_value & !(1u16 << 4usize)
                },
            }
        }
        /// Bit 3 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `M` (or `Mem[A]`)
        #[inline]
        pub const fn dest_m(&self) -> bool {
            (self.raw_value & (1u16 << 3usize)) != 0
        }
        /// Bit 3 of a C-Instruction
        ///
        /// If this flag is set, the value computed by `comp` will be stored in register `M` (or `Mem[A]`)
        #[inline]
        pub const fn with_dest_m(&self, field_value: bool) -> Self {
            Self {
                raw_value: if field_value {
                    self.raw_value | (1u16 << 3usize)
                } else {
                    self.raw_value & !(1u16 << 3usize)
                },
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
                        .with_dest(dest)
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
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        ///
        /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
        pub const R0: Self = Self { raw_value: 0 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        ///
        /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
        pub const R1: Self = Self { raw_value: 1 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        ///
        /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
        pub const R2: Self = Self { raw_value: 2 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        ///
        /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
        pub const R3: Self = Self { raw_value: 3 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        ///
        /// Use of the `R0`-`R4` registers is to be avoided in favor of the `SP`, `LCL`, `ARG`, `THIS`, and `THAT` registers.
        pub const R4: Self = Self { raw_value: 4 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R5: Self = Self { raw_value: 5 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R6: Self = Self { raw_value: 6 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R7: Self = Self { raw_value: 7 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R8: Self = Self { raw_value: 8 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R9: Self = Self { raw_value: 9 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R10: Self = Self { raw_value: 10 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R11: Self = Self { raw_value: 11 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R12: Self = Self { raw_value: 12 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R13: Self = Self { raw_value: 13 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R14: Self = Self { raw_value: 14 };
        /// The values 0-15 are also accessible as built in virtual registers, mostly useful for distinguishing between their use as addresses and constant values.
        pub const R15: Self = Self { raw_value: 15 };
        /// The screen of the Hack platform is hardware-mapped starting at the address 16384
        ///
        /// It is a
        pub const SCREEN: Self = Self { raw_value: 16384 };
        /// The keyboard of the Hack platform is hardware-mapped to the address 24576
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
        pub const fn from(value: i16) -> Self {
            Self { raw_value: value as u16 }
        }
    }
    impl PartialEq for Instruction {
        fn eq(&self, other: &Self) -> bool {
            self.raw_value == other.raw_value
        }
    }
    impl std::fmt::Display for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.a_inst().is_ok() {
                f.write_fmt(format_args!("@{0}", self.addr()))
            } else if self.non_a_inst().is_ok() {
                let c = self.c_inst();
                match c.dest() {
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
    impl<'a> Asm<'a> {
        pub const SP: Self = Self::At(Cow::Borrowed("SP"));
        pub const LCL: Self = Self::At(Cow::Borrowed("LCL"));
        pub const ARG: Self = Self::At(Cow::Borrowed("ARG"));
        pub const THIS: Self = Self::At(Cow::Borrowed("THIS"));
        pub const THAT: Self = Self::At(Cow::Borrowed("THAT"));
        pub const R0: Self = Self::At(Cow::Borrowed("R0"));
        pub const R1: Self = Self::At(Cow::Borrowed("R1"));
        pub const R2: Self = Self::At(Cow::Borrowed("R2"));
        pub const R3: Self = Self::At(Cow::Borrowed("R3"));
        pub const R4: Self = Self::At(Cow::Borrowed("R4"));
        pub const R5: Self = Self::At(Cow::Borrowed("R5"));
        pub const R6: Self = Self::At(Cow::Borrowed("R6"));
        pub const R7: Self = Self::At(Cow::Borrowed("R7"));
        pub const R8: Self = Self::At(Cow::Borrowed("R8"));
        pub const R9: Self = Self::At(Cow::Borrowed("R9"));
        pub const R10: Self = Self::At(Cow::Borrowed("R10"));
        pub const R11: Self = Self::At(Cow::Borrowed("R11"));
        pub const R12: Self = Self::At(Cow::Borrowed("R12"));
        pub const R13: Self = Self::At(Cow::Borrowed("R13"));
        pub const R14: Self = Self::At(Cow::Borrowed("R14"));
        pub const R15: Self = Self::At(Cow::Borrowed("R15"));
        pub const SCREEN: Self = Self::At(Cow::Borrowed("SCREEN"));
        pub const KBD: Self = Self::At(Cow::Borrowed("KBD"));
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
                CInstruction::DEFAULT
                    .with_dest_a(dest.contains('A'))
                    .with_dest_m(dest.contains('M'))
                    .with_dest_d(dest.contains('D'))
                    .dest()
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
                    Asm::At(l) => self.get_label(&l).map(|i| Instruction::from(i)),
                    Asm::Asm(i) => Some(*i),
                    _ => None,
                })
                .collect()
        }
    }
}
mod code_writer {
    pub(crate) mod vm_writer {
        use std::{
            fmt::Display, fs::File, io::{BufWriter, Write},
            path::Path,
        };
        use super::CodeWriter;
        use crate::tokens::jack_tokens::{Keyword::*, Token};
        use crate::tokens::vm_commands::{MemSegment as Seg, VmCommand};
        pub struct VmWriter {
            writer: Option<BufWriter<File>>,
            if_counter: u16,
            while_counter: u16,
        }
        #[automatically_derived]
        impl ::core::default::Default for VmWriter {
            #[inline]
            fn default() -> VmWriter {
                VmWriter {
                    writer: ::core::default::Default::default(),
                    if_counter: ::core::default::Default::default(),
                    while_counter: ::core::default::Default::default(),
                }
            }
        }
        impl CodeWriter for VmWriter {
            fn new(filename: &str) -> Self {
                let file = File::create(Path::new(filename).with_extension("vm"))
                    .expect("could not create file");
                let writer = BufWriter::new(file);
                VmWriter {
                    writer: Some(writer),
                    if_counter: 0,
                    while_counter: 0,
                }
            }
            fn write(&mut self, contents: impl Display) {
                self.writer
                    .as_mut()
                    .expect("no writer")
                    .write_fmt(format_args!("{0}\n", contents))
                    .expect("failed to write");
            }
            fn flush(&mut self) {
                self.writer.as_mut().expect("no writer").flush().unwrap();
            }
        }
        impl VmWriter {
            pub fn generate_label(&mut self, label: &str) -> String {
                let counter = if label == "if" {
                    &mut self.if_counter
                } else {
                    &mut self.while_counter
                };
                let label = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}{1}", label, counter),
                    );
                    res
                };
                *counter += 1;
                label
            }
            pub fn write_constant(&mut self, t: Token) {
                match t {
                    Token::Keyword(True) => {
                        self.write(VmCommand::Push(Seg::Constant, 1));
                        self.write(VmCommand::Neg);
                    }
                    Token::Keyword(False) | Token::Keyword(Null) => {
                        self.write(VmCommand::Push(Seg::Constant, 0))
                    }
                    Token::Keyword(This) => self.write(VmCommand::Push(Seg::Pointer, 0)),
                    Token::IntConstant(i) => {
                        self.write(VmCommand::Push(Seg::Constant, i))
                    }
                    Token::StringConstant(s) => {
                        self.write(VmCommand::Push(Seg::Constant, s.len() as i16));
                        self.write(VmCommand::Call("String.new", 1));
                        for c in s.chars() {
                            self.write(VmCommand::Push(Seg::Constant, c as i16));
                            self.write(VmCommand::Call("String.appendChar", 2));
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("only passing constants")
                            ),
                        );
                    }
                }
            }
        }
    }
    pub(crate) mod xml_writer {
        use std::{
            fmt::Display, fs::File, io::{BufWriter, Write},
            path::Path,
        };
        use crate::code_writer::CodeWriter;
        use crate::tokens::jack_tokens::Token;
        struct XMLWrapper {
            inner: Token,
        }
        impl Display for XMLWrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.inner {
                    Token::Keyword(k) => {
                        f.write_fmt(format_args!("<keyword> {0} </keyword>", k))
                    }
                    Token::Identifier(s) => {
                        f.write_fmt(format_args!("<identifier> {0} </identifier>", s))
                    }
                    Token::StringConstant(s) => {
                        f.write_fmt(
                            format_args!("<stringConstant> {0} </stringConstant>", s),
                        )
                    }
                    Token::IntConstant(i) => {
                        f.write_fmt(
                            format_args!("<integerConstant> {0} </integerConstant>", i),
                        )
                    }
                    Token::Symbol(c) => {
                        match c {
                            '<' => f.write_fmt(format_args!("<symbol> &lt; </symbol>")),
                            '>' => f.write_fmt(format_args!("<symbol> &gt; </symbol>")),
                            '"' => f.write_fmt(format_args!("<symbol> &quot; </symbol>")),
                            '&' => f.write_fmt(format_args!("<symbol> &amp; </symbol>")),
                            _ => f.write_fmt(format_args!("<symbol> {0} </symbol>", c)),
                        }
                    }
                }
            }
        }
        pub struct XMLWriter {
            writer: Option<BufWriter<File>>,
        }
        #[automatically_derived]
        impl ::core::default::Default for XMLWriter {
            #[inline]
            fn default() -> XMLWriter {
                XMLWriter {
                    writer: ::core::default::Default::default(),
                }
            }
        }
        impl CodeWriter for XMLWriter {
            fn new(filename: &str) -> Self {
                let file = File::create(Path::new(filename).with_extension("xml"))
                    .expect("could not create file");
                let writer = BufWriter::new(file);
                XMLWriter { writer: Some(writer) }
            }
            fn write(&mut self, contents: impl Display) {
                self.writer
                    .as_mut()
                    .unwrap()
                    .write_fmt(format_args!("{0}\n", contents))
                    .expect("failed to write");
                self.flush();
            }
            fn flush(&mut self) {
                self.writer.as_mut().unwrap().flush().unwrap();
            }
        }
        impl XMLWriter {}
    }
    pub trait CodeWriter: Default {
        fn write(&mut self, contents: impl std::fmt::Display);
        fn flush(&mut self);
        fn new(filename: &str) -> Self;
    }
}
mod cpu {
    use anyhow::{bail, Result};
    use crate::{asm::*, tokens::vm_commands::{MemSegment as Seg, VmCommand}};
    const KBD: i16 = 0x6000;
    const SCREEN_START: i16 = 0x4000;
    const SCREEN_END: i16 = 0x5FFF;
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
    struct Cpu {
        ram: [i16; 0xFFFF],
        rom: [Instruction; 0xFFFF],
        pc: usize,
        d: i16,
        a: i16,
    }
    #[allow(overflowing_literals)]
    impl Cpu {
        fn new() -> Self {
            Self {
                ram: [0; 0xFFFF],
                rom: [Instruction::SP; 0xFFFF],
                pc: 0,
                d: 0,
                a: 0,
            }
        }
        #[inline]
        const fn m(&self) -> i16 {
            self.ram[self.a as usize]
        }
        #[inline]
        fn m_mut(&mut self) -> &mut i16 {
            &mut self.ram[self.a as usize]
        }
        #[inline]
        const fn at(&self, addr: i16) -> i16 {
            self.ram[addr as usize]
        }
        #[inline]
        fn at_mut(&mut self, addr: i16) -> &mut i16 {
            &mut self.ram[addr as usize]
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
        pub fn tick(&mut self) {}
        pub fn execute_asm(&mut self, asm: &[Instruction]) -> Result<()> {
            for i in asm.into_iter() {
                use InstructionType as Inst;
                match i.get()? {
                    Inst::A(addr) => self.a = addr.value() as i16,
                    Inst::C(c) => {
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
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("Invalid C Bits"),
                                    );
                                    error
                                });
                            }
                        };
                        if (c.jump() == Jump::JMP) || (c.jeq() && comp == 0)
                            || (c.jgt() && comp > 0) || (c.jlt() && comp < 0)
                        {
                            self.pc = self.a as usize;
                        }
                        if c.dest_m() && self.a != KBD {
                            if let a @ SCREEN_START..=SCREEN_END = self.a {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "not yet implemented: {0}", format_args!("Update screen")
                                        ),
                                    );
                                };
                            }
                            *self.m_mut() = comp;
                        }
                        if c.dest_a() {
                            self.a = comp;
                        }
                        if c.dest_d() {
                            self.d = comp;
                        }
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
                VmCommand::Compare(_) => ::core::panicking::panic("not yet implemented"),
                VmCommand::And => {
                    self.pop();
                    *self.stack_top() &= self.d;
                }
                VmCommand::Or => {
                    self.pop();
                    *self.stack_top() |= self.d;
                }
                VmCommand::Not => *self.stack_top() = !*self.stack_top(),
                VmCommand::Push(_, _) => ::core::panicking::panic("not yet implemented"),
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
                        Seg::Constant => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    };
                    *self.at_mut(addr) = self.d;
                }
                VmCommand::Label(_) => ::core::panicking::panic("not yet implemented"),
                VmCommand::Goto(_) => ::core::panicking::panic("not yet implemented"),
                VmCommand::IfGoto(_) => ::core::panicking::panic("not yet implemented"),
                VmCommand::Function(_, _) => {
                    ::core::panicking::panic("not yet implemented")
                }
                VmCommand::Call(_, _) => ::core::panicking::panic("not yet implemented"),
                VmCommand::Return => ::core::panicking::panic("not yet implemented"),
            }
        }
    }
}
mod io {
    use sdl2::keyboard::Keycode;
    fn get_key(code: Keycode) -> i16 {
        match code {
            Keycode::Backspace => 129,
            Keycode::Tab => code as i16,
            Keycode::Return => 128,
            Keycode::Escape => code as i16,
            Keycode::Space => code as i16,
            Keycode::Exclaim => code as i16,
            Keycode::Quotedbl => code as i16,
            Keycode::Hash => code as i16,
            Keycode::Dollar => code as i16,
            Keycode::Percent => code as i16,
            Keycode::Ampersand => code as i16,
            Keycode::Quote => code as i16,
            Keycode::LeftParen => code as i16,
            Keycode::RightParen => code as i16,
            Keycode::Asterisk => code as i16,
            Keycode::Plus => code as i16,
            Keycode::Comma => code as i16,
            Keycode::Minus => code as i16,
            Keycode::Period => code as i16,
            Keycode::Slash => code as i16,
            Keycode::Num0 => code as i16,
            Keycode::Num1 => code as i16,
            Keycode::Num2 => code as i16,
            Keycode::Num3 => code as i16,
            Keycode::Num4 => code as i16,
            Keycode::Num5 => code as i16,
            Keycode::Num6 => code as i16,
            Keycode::Num7 => code as i16,
            Keycode::Num8 => code as i16,
            Keycode::Num9 => code as i16,
            Keycode::Colon => code as i16,
            Keycode::Semicolon => code as i16,
            Keycode::Less => code as i16,
            Keycode::Equals => code as i16,
            Keycode::Greater => code as i16,
            Keycode::Question => code as i16,
            Keycode::At => code as i16,
            Keycode::LeftBracket => code as i16,
            Keycode::Backslash => code as i16,
            Keycode::RightBracket => code as i16,
            Keycode::Caret => code as i16,
            Keycode::Underscore => code as i16,
            Keycode::Backquote => code as i16,
            Keycode::A => code as i16,
            Keycode::B => code as i16,
            Keycode::C => code as i16,
            Keycode::D => code as i16,
            Keycode::E => code as i16,
            Keycode::F => code as i16,
            Keycode::G => code as i16,
            Keycode::H => code as i16,
            Keycode::I => code as i16,
            Keycode::J => code as i16,
            Keycode::K => code as i16,
            Keycode::L => code as i16,
            Keycode::M => code as i16,
            Keycode::N => code as i16,
            Keycode::O => code as i16,
            Keycode::P => code as i16,
            Keycode::Q => code as i16,
            Keycode::R => code as i16,
            Keycode::S => code as i16,
            Keycode::T => code as i16,
            Keycode::U => code as i16,
            Keycode::V => code as i16,
            Keycode::W => code as i16,
            Keycode::X => code as i16,
            Keycode::Y => code as i16,
            Keycode::Z => code as i16,
            Keycode::Delete => 139,
            Keycode::F1 => 141,
            Keycode::F2 => 142,
            Keycode::F3 => 143,
            Keycode::F4 => 144,
            Keycode::F5 => 145,
            Keycode::F6 => 146,
            Keycode::F7 => 147,
            Keycode::F8 => 148,
            Keycode::F9 => 149,
            Keycode::F10 => 150,
            Keycode::F11 => 151,
            Keycode::F12 => 152,
            Keycode::Insert => 138,
            Keycode::Home => 134,
            Keycode::PageUp => 136,
            Keycode::End => 135,
            Keycode::PageDown => 137,
            Keycode::Right => 132,
            Keycode::Left => 130,
            Keycode::Down => 133,
            Keycode::Up => 131,
            _ => -1,
        }
    }
}
mod jack_compiler {
    pub(crate) mod compilation_engine {
        use crate::code_writer::{vm_writer::VmWriter, CodeWriter};
        use crate::jack_compiler::{symbol_table::*, tokenizer::Tokenizer};
        use crate::tokens::{
            jack_tokens::{
                Keyword::{self, *},
                Token,
            },
            token_type::{TokenType, ValidToken},
            vm_commands::{Comparison::*, MemSegment as Mem, VmCommand},
        };
        use std::path::PathBuf;
        pub struct CompilationEngine {
            writer: VmWriter,
            tokenizer: Tokenizer,
            class_name: String,
            curr_token: Option<Token>,
            symbol_table: SymbolTable,
            errors: Vec<(CompilationError, Option<Token>)>,
        }
        pub enum CompilationError {
            DuplicateIdentifier,
            UnexpectedToken,
            InvalidInt,
            UnrecognizedToken,
            UndeclaredIdentifier,
            UnexpectedEndofTokens,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CompilationError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        CompilationError::DuplicateIdentifier => "DuplicateIdentifier",
                        CompilationError::UnexpectedToken => "UnexpectedToken",
                        CompilationError::InvalidInt => "InvalidInt",
                        CompilationError::UnrecognizedToken => "UnrecognizedToken",
                        CompilationError::UndeclaredIdentifier => "UndeclaredIdentifier",
                        CompilationError::UnexpectedEndofTokens => {
                            "UnexpectedEndofTokens"
                        }
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CompilationError {
            #[inline]
            fn clone(&self) -> CompilationError {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CompilationError {}
        use crate::tokens::token_type::TokenType::*;
        impl CompilationEngine {
            pub fn new() -> Self {
                CompilationEngine {
                    writer: VmWriter::default(),
                    tokenizer: Tokenizer::default(),
                    class_name: String::new(),
                    symbol_table: SymbolTable::default(),
                    curr_token: None,
                    errors: ::alloc::vec::Vec::new(),
                }
            }
            pub fn throw_error(&mut self, err: CompilationError) {
                let token = self.curr_token.as_ref();
                self.errors.push((err, Option::<&Token>::cloned(token)));
            }
            pub fn curr_token_is<T: ValidToken + PartialEq<Token>>(
                &self,
                other: T,
            ) -> bool {
                if let Some(t) = self.curr_token.as_ref() { other == *t } else { false }
            }
            pub fn compile(
                &mut self,
                file: PathBuf,
            ) -> Result<(), &[(CompilationError, Option<Token>)]> {
                let filename = file
                    .as_path()
                    .to_str()
                    .expect("could not convert to str");
                let tokenizer = Tokenizer::new(
                    std::fs::read_to_string(&file).expect("failed to read"),
                );
                self.writer = VmWriter::new(filename);
                self.tokenizer = tokenizer;
                self.curr_token = self.tokenizer.advance();
                self.symbol_table = SymbolTable::default();
                self.construct_class();
                self.writer.flush();
                let errors = &self.errors;
                if !errors.is_empty() { Err(errors) } else { Ok(()) }
            }
            fn consume<T: ValidToken + PartialEq<Token> + Copy>(
                &mut self,
                requested: T,
            ) -> Token {
                if self.curr_token.is_none() {
                    self.throw_error(CompilationError::UnexpectedEndofTokens);
                } else if !self.curr_token_is(requested) {
                    self.throw_error(CompilationError::UnexpectedToken);
                }
                let mut token = self.tokenizer.advance();
                std::mem::swap(&mut self.curr_token, &mut token);
                token.unwrap_or(Token::Symbol('?'))
            }
            fn construct_class(&mut self) {
                self.consume(Class);
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.class_name = name;
                }
                self.consume('{');
                while self.curr_token_is(TokenType::ClassVarDec) {
                    self.handle_class_var_dec();
                }
                while self.curr_token_is(TokenType::SubroutineDec) {
                    self.handle_subroutine_dec();
                }
                self.consume('}');
            }
            fn handle_class_var_dec(&mut self) {
                if let (
                    Token::Keyword(k @ (Static | Field)),
                    type_of,
                    Token::Identifier(name),
                )
                    = (
                        self.consume(TokenType::ClassVarDec),
                        self.consume(TokenType::Type),
                        self.consume(TokenType::Name),
                    ) {
                    let kind = if k == Static { Kind::Static } else { Kind::Field };
                    let type_str = type_of.as_type();
                    self.symbol_table
                        .define(kind, &type_str, name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                    while self.curr_token_is(',') {
                        self.consume(',');
                        if let Token::Identifier(name) = self.consume(TokenType::Name) {
                            self.symbol_table
                                .define(kind, &type_str, name)
                                .map_err(|e| self.throw_error(e))
                                .unwrap();
                        }
                    }
                    self.consume(';');
                }
            }
            fn handle_subroutine_dec(&mut self) {
                self.symbol_table.start_subroutine();
                if let (
                    Token::Keyword(func_type @ (Constructor | Function | Method)),
                    _return_type,
                    Token::Identifier(name),
                )
                    = (
                        self.consume(TokenType::SubroutineDec),
                        self.consume(TokenType::ReturnType),
                        self.consume(TokenType::Name),
                    ) {
                    if func_type == Method {
                        self.symbol_table
                            .define(Kind::Arg, &self.class_name, String::from("this"))
                            .map_err(|e| self.throw_error(e))
                            .unwrap();
                    }
                    self.consume('(');
                    self.handle_parameter_list();
                    self.consume(')');
                    self.handle_subroutine_body(func_type, name);
                }
            }
            fn handle_parameter_list(&mut self) {
                while !self.curr_token_is(')') {
                    if let (type_of, Token::Identifier(name))
                        = (
                            self.consume(TokenType::Type),
                            self.consume(TokenType::Name),
                        ) {
                        self.symbol_table
                            .define(Kind::Arg, &type_of.as_type(), name)
                            .map_err(|e| self.throw_error(e))
                            .unwrap();
                    }
                    if self.curr_token_is(',') {
                        self.consume(',');
                    }
                }
            }
            fn handle_subroutine_body(&mut self, func_type: Keyword, name: String) {
                self.consume('{');
                while self.curr_token_is(Keyword::Var) {
                    self.handle_var_dec();
                }
                self.writer
                    .write(
                        VmCommand::Function(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}.{1}", self.class_name, name),
                                );
                                res
                            },
                            self.symbol_table.var_count(Kind::Var),
                        ),
                    );
                if func_type == Constructor {
                    self.writer
                        .write(
                            VmCommand::Push(
                                Mem::Constant,
                                self.symbol_table.var_count(Kind::Field),
                            ),
                        );
                    self.writer.write(VmCommand::Call("Memory.alloc", 1));
                    self.writer.write(VmCommand::Pop(Mem::Pointer, 0));
                } else if func_type == Method {
                    self.writer.write(VmCommand::Push(Mem::Argument, 0));
                    self.writer.write(VmCommand::Pop(Mem::Pointer, 0));
                }
                self.handle_statements();
                self.consume('}');
            }
            fn handle_var_dec(&mut self) {
                if let (Token::Keyword(_k @ Var), type_of, Token::Identifier(name))
                    = (
                        self.consume(Var),
                        self.consume(TokenType::Type),
                        self.consume(TokenType::Name),
                    ) {
                    self.symbol_table
                        .define(Kind::Var, &type_of.as_type(), name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                    while self.curr_token_is(',') {
                        self.consume(',');
                        if let Token::Identifier(name) = self.consume(TokenType::Name) {
                            self.symbol_table
                                .define(Kind::Var, &type_of.as_type(), name)
                                .map_err(|e| self.throw_error(e))
                                .unwrap();
                        }
                    }
                    self.consume(';');
                }
            }
            fn handle_statements(&mut self) {
                while self.curr_token_is(TokenType::Statement) {
                    match self.curr_token.as_ref() {
                        Some(Token::Keyword(Let)) => self.handle_let(),
                        Some(Token::Keyword(If)) => self.handle_if(),
                        Some(Token::Keyword(While)) => self.handle_while(),
                        Some(Token::Keyword(Do)) => self.handle_do(),
                        Some(Token::Keyword(Return)) => self.handle_return(),
                        _ => break,
                    }
                }
            }
            fn handle_let(&mut self) {
                self.consume(Let);
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    let (mut seg, mut id) = if let Some(entry)
                        = self.symbol_table.get(&name)
                    {
                        (
                            match entry.get_kind() {
                                Kind::Static => Mem::Static,
                                Kind::Field => Mem::This,
                                Kind::Arg => Mem::Argument,
                                Kind::Var => Mem::Local,
                            },
                            entry.get_id(),
                        )
                    } else {
                        self.throw_error(CompilationError::UndeclaredIdentifier);
                        (Mem::Constant, 0)
                    };
                    let arr = if self.curr_token_is('[') {
                        self.consume('[');
                        self.handle_expression();
                        self.consume(']');
                        self.writer.write(VmCommand::Push(seg, id));
                        self.writer.write(VmCommand::Add);
                        (seg, id) = (Mem::That, 0);
                        true
                    } else {
                        false
                    };
                    self.consume('=');
                    self.handle_expression();
                    if arr {
                        self.writer.write(VmCommand::Pop(Mem::Temp, 0));
                        self.writer.write(VmCommand::Pop(Mem::Pointer, 1));
                        self.writer.write(VmCommand::Push(Mem::Temp, 0));
                    }
                    self.writer.write(VmCommand::Pop(seg, id));
                    self.consume(';');
                }
            }
            fn handle_while(&mut self) {
                self.consume(While);
                self.consume('(');
                let start_label = self.writer.generate_label("while");
                let end_label = self.writer.generate_label("while");
                self.writer.write(VmCommand::Label(&start_label));
                self.handle_expression();
                self.writer.write(VmCommand::Not);
                self.writer.write(VmCommand::IfGoto(&end_label));
                self.consume(')');
                self.consume('{');
                self.handle_statements();
                self.writer.write(VmCommand::Goto(&start_label));
                self.consume('}');
                self.writer.write(VmCommand::Label(&end_label));
            }
            fn handle_if(&mut self) {
                self.consume(If);
                self.consume('(');
                self.handle_expression();
                self.consume(')');
                self.writer.write(VmCommand::Not);
                let label1 = self.writer.generate_label("if");
                let label2 = self.writer.generate_label("if");
                self.writer.write(VmCommand::IfGoto(&label1));
                self.consume('{');
                self.handle_statements();
                self.consume('}');
                self.writer.write(VmCommand::Goto(&label2));
                self.writer.write(VmCommand::Label(&label1));
                if self.curr_token_is(Else) {
                    self.consume(Else);
                    if self.curr_token_is(If) {
                        self.handle_if();
                    } else {
                        self.consume('{');
                        self.handle_statements();
                        self.consume('}');
                    }
                }
                self.writer.write(VmCommand::Label(&label2));
            }
            fn handle_do(&mut self) {
                self.consume(Do);
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    if let Some(Token::Symbol(c @ ('.' | '('))) = self.curr_token {
                        self.handle_subroutine_call(name, c);
                    }
                }
                self.consume(';');
                self.writer.write(VmCommand::Pop(Mem::Temp, 0));
            }
            fn handle_return(&mut self) {
                self.consume(Return);
                if !self.curr_token_is(';') {
                    self.handle_expression();
                } else {
                    self.writer.write(VmCommand::Push(Mem::Constant, 0));
                }
                self.writer.write(VmCommand::Return);
                self.consume(';');
            }
            fn handle_subroutine_call(&mut self, name: String, next: char) {
                let mut method = false;
                let func_label: String;
                self.consume(next);
                if next == '.' {
                    let token = self.consume(Name);
                    self.consume('(');
                    match (self.symbol_table.get(&name), token) {
                        (Some(entry), Token::Identifier(f)) => {
                            self.writer
                                .write(
                                    VmCommand::Push(
                                        entry.get_kind().to_mem_seg(),
                                        entry.get_id(),
                                    ),
                                );
                            func_label = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}.{1}", entry.get_type(), f),
                                );
                                res
                            };
                            method = true;
                        }
                        (None, Token::Identifier(f)) => {
                            func_label = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}.{1}", name, f),
                                );
                                res
                            };
                        }
                        _ => func_label = String::from("error"),
                    }
                } else {
                    self.writer.write(VmCommand::Push(Mem::Pointer, 0));
                    method = true;
                    func_label = {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.{1}", self.class_name, name),
                        );
                        res
                    };
                }
                let args = self.handle_expression_list();
                self.consume(')');
                self.writer.write(VmCommand::Call(&func_label, args + method as i16));
            }
            fn handle_term(&mut self) {
                let op = if self.curr_token_is(TokenType::UnaryOp) {
                    match self.consume(TokenType::UnaryOp) {
                        Token::Symbol('-') => Some(VmCommand::Neg),
                        Token::Symbol('~') => Some(VmCommand::Not),
                        _ => None,
                    }
                } else {
                    None
                };
                if self.curr_token_is('(') {
                    self.consume('(');
                    self.handle_expression();
                    self.consume(')');
                } else if self.curr_token_is(TokenType::Constant) {
                    let token = self.consume(Constant);
                    self.writer.write_constant(token);
                } else if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    match (self.symbol_table.get(&name), &self.curr_token) {
                        (_, Some(Token::Symbol(c @ ('.' | '(')))) => {
                            self.handle_subroutine_call(name, *c)
                        }
                        (Some(entry), _) => {
                            let (kind, id) = (
                                entry.get_kind().to_mem_seg(),
                                entry.get_id(),
                            );
                            if self.curr_token_is('[') {
                                self.consume('[');
                                self.handle_expression();
                                self.consume(']');
                                self.writer.write(VmCommand::Push(kind, id));
                                self.writer.write(VmCommand::Add);
                                self.writer.write(VmCommand::Pop(Mem::Pointer, 1));
                                self.writer.write(VmCommand::Push(Mem::That, 0));
                            } else {
                                self.writer.write(VmCommand::Push(kind, id));
                            }
                        }
                        (None, _) => {
                            self.throw_error(CompilationError::UndeclaredIdentifier)
                        }
                    }
                }
                if let Some(o) = op {
                    self.writer.write(o);
                }
            }
            fn handle_expression(&mut self) {
                self.handle_term();
                if self.curr_token_is(TokenType::BinaryOp) {
                    let op = self.consume(TokenType::BinaryOp);
                    self.handle_term();
                    let op_cmd = match op {
                        Token::Symbol('+') => VmCommand::Add,
                        Token::Symbol('-') => VmCommand::Sub,
                        Token::Symbol('&') => VmCommand::And,
                        Token::Symbol('|') => VmCommand::Or,
                        Token::Symbol('=') => VmCommand::Compare(Eq),
                        Token::Symbol('>') => VmCommand::Compare(GT),
                        Token::Symbol('<') => VmCommand::Compare(LT),
                        Token::Symbol('*') => VmCommand::Call("Math.multiply", 2),
                        Token::Symbol('/') => VmCommand::Call("Math.divide", 2),
                        Token::Symbol('%') => VmCommand::Call("Math.modulo", 2),
                        _ => VmCommand::Label("not a binary op"),
                    };
                    self.writer.write(op_cmd);
                }
            }
            fn handle_expression_list(&mut self) -> i16 {
                let mut count: i16 = 0;
                while !self.curr_token_is(')') {
                    self.handle_expression();
                    count += 1;
                    if self.curr_token_is(',') {
                        self.consume(',');
                    }
                }
                count
            }
        }
    }
    pub(crate) mod symbol_table {
        use std::{collections::HashMap, fmt::Display};
        use crate::jack_compiler::compilation_engine::CompilationError;
        use crate::tokens::vm_commands::MemSegment as Seg;
        pub enum Kind {
            Static,
            Field,
            Arg,
            Var,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Kind::Static => "Static",
                        Kind::Field => "Field",
                        Kind::Arg => "Arg",
                        Kind::Var => "Var",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Kind {
            #[inline]
            fn clone(&self) -> Kind {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Kind {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Kind {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Kind {
            #[inline]
            fn eq(&self, other: &Kind) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Kind {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Kind {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl Display for Kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    Kind::Static => "static",
                    Kind::Field => "this",
                    Kind::Arg => "argument",
                    Kind::Var => "local",
                };
                f.write_fmt(format_args!("{0}", s))
            }
        }
        impl Kind {
            pub fn to_mem_seg(self) -> Seg {
                match self {
                    Kind::Static => Seg::Static,
                    Kind::Field => Seg::This,
                    Kind::Arg => Seg::Argument,
                    Kind::Var => Seg::Local,
                }
            }
        }
        pub struct SymbolTable {
            static_count: i16,
            field_count: i16,
            arg_count: i16,
            local_count: i16,
            class_lvl_table: HashMap<String, SymbolEntry>,
            subroutine_lvl_table: HashMap<String, SymbolEntry>,
        }
        #[automatically_derived]
        impl ::core::default::Default for SymbolTable {
            #[inline]
            fn default() -> SymbolTable {
                SymbolTable {
                    static_count: ::core::default::Default::default(),
                    field_count: ::core::default::Default::default(),
                    arg_count: ::core::default::Default::default(),
                    local_count: ::core::default::Default::default(),
                    class_lvl_table: ::core::default::Default::default(),
                    subroutine_lvl_table: ::core::default::Default::default(),
                }
            }
        }
        impl SymbolTable {
            pub fn define(
                &mut self,
                kind: Kind,
                type_of: &str,
                name: String,
            ) -> Result<(), CompilationError> {
                let (table, counter) = match kind {
                    Kind::Static => (&mut self.class_lvl_table, &mut self.static_count),
                    Kind::Field => (&mut self.class_lvl_table, &mut self.field_count),
                    Kind::Arg => (&mut self.subroutine_lvl_table, &mut self.arg_count),
                    Kind::Var => (&mut self.subroutine_lvl_table, &mut self.local_count),
                };
                if table.get(&name).is_none() {
                    table
                        .insert(
                            name,
                            SymbolEntry {
                                var_type: String::from(type_of),
                                kind,
                                id: *counter,
                            },
                        );
                    *counter += 1;
                    Ok(())
                } else {
                    Err(CompilationError::DuplicateIdentifier)
                }
            }
            pub fn var_count(&self, kind: Kind) -> i16 {
                match kind {
                    Kind::Static => self.static_count,
                    Kind::Field => self.field_count,
                    Kind::Arg => self.arg_count,
                    Kind::Var => self.local_count,
                }
            }
            pub fn get(&self, name: &str) -> Option<&SymbolEntry> {
                if let Some(e) = self.class_lvl_table.get(name) {
                    Some(e)
                } else if let Some(e) = self.subroutine_lvl_table.get(name) {
                    Some(e)
                } else {
                    None
                }
            }
            pub fn start_subroutine(&mut self) {
                self.subroutine_lvl_table.clear();
                self.arg_count = 0;
                self.local_count = 0;
            }
        }
        pub struct SymbolEntry {
            var_type: String,
            kind: Kind,
            id: i16,
        }
        impl SymbolEntry {
            pub fn get_type(&self) -> &str {
                &self.var_type
            }
            pub fn get_kind(&self) -> Kind {
                self.kind
            }
            pub fn get_id(&self) -> i16 {
                self.id
            }
        }
    }
    pub(crate) mod tokenizer {
        use crate::jack_compiler::compilation_engine::CompilationError;
        use crate::tokens::jack_tokens::*;
        use std::collections::VecDeque;
        impl From<std::num::ParseIntError> for CompilationError {
            fn from(_: std::num::ParseIntError) -> Self {
                CompilationError::InvalidInt
            }
        }
        pub struct Tokenizer {
            chars: VecDeque<char>,
            errors: Vec<CompilationError>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Tokenizer {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Tokenizer",
                    "chars",
                    &self.chars,
                    "errors",
                    &&self.errors,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Tokenizer {
            #[inline]
            fn default() -> Tokenizer {
                Tokenizer {
                    chars: ::core::default::Default::default(),
                    errors: ::core::default::Default::default(),
                }
            }
        }
        impl Tokenizer {
            pub fn new(file: String) -> Self {
                Tokenizer {
                    chars: file.trim().chars().collect(),
                    errors: ::alloc::vec::Vec::new(),
                }
            }
            fn advance_past_comment(&mut self) -> bool {
                match self.chars.get(0) {
                    Some('*') => {
                        while let Some(c) = self.chars.pop_front() {
                            if c == '*' && self.chars.get(0) == Some(&'/') {
                                self.chars.pop_front();
                                break;
                            }
                        }
                        true
                    }
                    Some('/') => {
                        while let Some(c) = self.chars.pop_front() {
                            if c == '\n' {
                                break;
                            }
                        }
                        true
                    }
                    _ => false,
                }
            }
            fn get_string(&mut self) -> Option<Token> {
                let mut end = self.chars.len();
                for (i, &c) in self.chars.iter().enumerate() {
                    if c == '"' {
                        end = i;
                        break;
                    }
                }
                let s: String = self.chars.drain(..end).collect();
                self.chars.pop_front();
                Some(Token::StringConstant(s))
            }
            pub fn advance(&mut self) -> Option<Token> {
                if let Some(c) = self.chars.pop_front() {
                    if SYMBOLS.contains(&c) {
                        match c {
                            '"' => self.get_string(),
                            _ => {
                                if c == '/' && self.advance_past_comment() {
                                    self.advance()
                                } else {
                                    Some(Token::Symbol(c))
                                }
                            }
                        }
                    } else if c.is_numeric() {
                        let mut num = String::from(c);
                        let mut end = self.chars.len();
                        for (i, &c) in self.chars.iter().enumerate() {
                            if !c.is_numeric() {
                                end = i;
                                break;
                            }
                        }
                        num.extend(self.chars.drain(..end));
                        if let Ok(i) = num.parse::<i16>() {
                            Some(Token::IntConstant(i))
                        } else {
                            self.errors.push(CompilationError::InvalidInt);
                            self.advance()
                        }
                    } else if c.is_alphabetic() || c == '_' {
                        let mut word = String::from(c);
                        let mut end = self.chars.len();
                        for (i, &c) in self.chars.iter().enumerate() {
                            if !(c.is_alphanumeric() || c == '_') {
                                end = i;
                                break;
                            }
                        }
                        word.extend(self.chars.drain(..end));
                        if let Some(&k) = KEYWORDS.get(word.as_str()) {
                            Some(Token::Keyword(k))
                        } else {
                            Some(Token::Identifier(word))
                        }
                    } else if !c.is_whitespace() {
                        self.errors.push(CompilationError::UnrecognizedToken);
                        self.advance()
                    } else {
                        self.advance()
                    }
                } else {
                    None
                }
            }
        }
    }
}
mod tokens {
    pub(crate) mod jack_tokens {
        use std::{
            collections::{HashMap, HashSet},
            fmt::{Debug, Display},
        };
        use Keyword::*;
        use crate::tokens::token_type::TokenType;
        pub enum Token {
            Keyword(Keyword),
            Symbol(char),
            Identifier(String),
            IntConstant(i16),
            StringConstant(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Token {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Token::Keyword(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Keyword",
                            &__self_0,
                        )
                    }
                    Token::Symbol(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Symbol",
                            &__self_0,
                        )
                    }
                    Token::Identifier(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Identifier",
                            &__self_0,
                        )
                    }
                    Token::IntConstant(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "IntConstant",
                            &__self_0,
                        )
                    }
                    Token::StringConstant(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "StringConstant",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Token {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Token {
            #[inline]
            fn eq(&self, other: &Token) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Token::Keyword(__self_0), Token::Keyword(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Token::Symbol(__self_0), Token::Symbol(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Token::Identifier(__self_0), Token::Identifier(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Token::IntConstant(__self_0), Token::IntConstant(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (
                            Token::StringConstant(__self_0),
                            Token::StringConstant(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Token {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Token {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Keyword>;
                let _: ::core::cmp::AssertParamIsEq<char>;
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<i16>;
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Token {
            #[inline]
            fn clone(&self) -> Token {
                match self {
                    Token::Keyword(__self_0) => {
                        Token::Keyword(::core::clone::Clone::clone(__self_0))
                    }
                    Token::Symbol(__self_0) => {
                        Token::Symbol(::core::clone::Clone::clone(__self_0))
                    }
                    Token::Identifier(__self_0) => {
                        Token::Identifier(::core::clone::Clone::clone(__self_0))
                    }
                    Token::IntConstant(__self_0) => {
                        Token::IntConstant(::core::clone::Clone::clone(__self_0))
                    }
                    Token::StringConstant(__self_0) => {
                        Token::StringConstant(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        impl Token {
            pub fn as_type(&self) -> String {
                match self {
                    Token::Keyword(k @ (Int | Char | Boolean)) => {
                        let res = ::alloc::fmt::format(format_args!("{0}", k));
                        res
                    }
                    Token::Identifier(s) => s.clone(),
                    _ => String::from("invalid type"),
                }
            }
        }
        impl Display for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Token::Keyword(k) => f.write_fmt(format_args!("{0}", k)),
                    Token::Identifier(s) => f.write_fmt(format_args!("{0}", s)),
                    Token::StringConstant(s) => f.write_fmt(format_args!("{0}", s)),
                    Token::IntConstant(i) => f.write_fmt(format_args!("{0}", i)),
                    Token::Symbol(c) => f.write_fmt(format_args!("{0}", c)),
                }
            }
        }
        impl PartialEq<TokenType> for Token {
            fn eq(&self, other: &TokenType) -> bool {
                match self {
                    Token::Keyword(t) => t == other,
                    Token::Symbol(t) => t == other,
                    Token::Identifier(t) => t == other,
                    Token::IntConstant(t) => t == other,
                    Token::StringConstant(t) => t == other,
                }
            }
        }
        impl PartialEq<Option<Token>> for Token {
            fn eq(&self, other: &Option<Token>) -> bool {
                match (self, other) {
                    (Self::Keyword(l0), Some(Self::Keyword(r0))) => l0 == r0,
                    (Self::Symbol(l0), Some(Self::Symbol(r0))) => l0 == r0,
                    (Self::Identifier(l0), Some(Self::Identifier(r0))) => l0 == r0,
                    (Self::IntConstant(l0), Some(Self::IntConstant(r0))) => l0 == r0,
                    (Self::StringConstant(l0), Some(Self::StringConstant(r0))) => {
                        l0 == r0
                    }
                    _ => false,
                }
            }
        }
        impl PartialEq<char> for Token {
            fn eq(&self, other: &char) -> bool {
                if let Self::Symbol(t) = &self { t == other } else { false }
            }
        }
        impl PartialEq<Keyword> for Token {
            fn eq(&self, other: &Keyword) -> bool {
                if let Self::Keyword(t) = &self { t == other } else { false }
            }
        }
        impl PartialEq<String> for Token {
            fn eq(&self, other: &String) -> bool {
                match self {
                    Self::Identifier(s) | Self::StringConstant(s) => s == other,
                    _ => false,
                }
            }
        }
        impl PartialEq<i16> for Token {
            fn eq(&self, other: &i16) -> bool {
                if let Self::IntConstant(t) = &self { t == other } else { false }
            }
        }
        impl PartialEq<Token> for char {
            fn eq(&self, other: &Token) -> bool {
                other == self
            }
        }
        impl PartialEq<Token> for Keyword {
            fn eq(&self, other: &Token) -> bool {
                other == self
            }
        }
        impl PartialEq<Token> for String {
            fn eq(&self, other: &Token) -> bool {
                other == self
            }
        }
        impl PartialEq<Token> for i16 {
            fn eq(&self, other: &Token) -> bool {
                other == self
            }
        }
        pub enum Keyword {
            Class,
            Constructor,
            Function,
            Method,
            Field,
            Static,
            Var,
            Int,
            Char,
            Boolean,
            Void,
            True,
            False,
            Null,
            This,
            Let,
            Do,
            If,
            Else,
            While,
            Return,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Keyword {
            #[inline]
            fn clone(&self) -> Keyword {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Keyword {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Keyword {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Keyword::Class => "Class",
                        Keyword::Constructor => "Constructor",
                        Keyword::Function => "Function",
                        Keyword::Method => "Method",
                        Keyword::Field => "Field",
                        Keyword::Static => "Static",
                        Keyword::Var => "Var",
                        Keyword::Int => "Int",
                        Keyword::Char => "Char",
                        Keyword::Boolean => "Boolean",
                        Keyword::Void => "Void",
                        Keyword::True => "True",
                        Keyword::False => "False",
                        Keyword::Null => "Null",
                        Keyword::This => "This",
                        Keyword::Let => "Let",
                        Keyword::Do => "Do",
                        Keyword::If => "If",
                        Keyword::Else => "Else",
                        Keyword::While => "While",
                        Keyword::Return => "Return",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Keyword {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Keyword {
            #[inline]
            fn eq(&self, other: &Keyword) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Keyword {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Keyword {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let kw = match self {
                    Class => "class",
                    Constructor => "constructor",
                    Function => "function",
                    Method => "method",
                    Field => "field",
                    Static => "static",
                    Var => "var",
                    Int => "int",
                    Char => "char",
                    Boolean => "boolean",
                    Void => "void",
                    True => "true",
                    False => "false",
                    Null => "null",
                    This => "this",
                    Let => "let",
                    Do => "do",
                    If => "if",
                    Else => "else",
                    While => "while",
                    Return => "return",
                };
                f.write_fmt(format_args!("{0}", kw))
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct KEYWORDS {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static KEYWORDS: KEYWORDS = KEYWORDS { __private_field: () };
        impl ::lazy_static::__Deref for KEYWORDS {
            type Target = HashMap<&'static str, Keyword>;
            fn deref(&self) -> &HashMap<&'static str, Keyword> {
                #[inline(always)]
                fn __static_ref_initialize() -> HashMap<&'static str, Keyword> {
                    {
                        let mut hm = HashMap::new();
                        hm.insert("class", Class);
                        hm.insert("constructor", Constructor);
                        hm.insert("function", Function);
                        hm.insert("method", Method);
                        hm.insert("field", Field);
                        hm.insert("static", Static);
                        hm.insert("var", Var);
                        hm.insert("int", Int);
                        hm.insert("char", Char);
                        hm.insert("boolean", Boolean);
                        hm.insert("void", Void);
                        hm.insert("true", True);
                        hm.insert("false", False);
                        hm.insert("null", Null);
                        hm.insert("this", This);
                        hm.insert("let", Let);
                        hm.insert("do", Do);
                        hm.insert("if", If);
                        hm.insert("else", Else);
                        hm.insert("while", While);
                        hm.insert("return", Return);
                        hm
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static HashMap<&'static str, Keyword> {
                    static LAZY: ::lazy_static::lazy::Lazy<
                        HashMap<&'static str, Keyword>,
                    > = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for KEYWORDS {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct SYMBOLS {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static SYMBOLS: SYMBOLS = SYMBOLS { __private_field: () };
        impl ::lazy_static::__Deref for SYMBOLS {
            type Target = HashSet<char>;
            fn deref(&self) -> &HashSet<char> {
                #[inline(always)]
                fn __static_ref_initialize() -> HashSet<char> {
                    {
                        let mut hs = HashSet::new();
                        hs.insert('{');
                        hs.insert('}');
                        hs.insert('(');
                        hs.insert(')');
                        hs.insert('[');
                        hs.insert(']');
                        hs.insert('.');
                        hs.insert(',');
                        hs.insert(';');
                        hs.insert('+');
                        hs.insert('-');
                        hs.insert('*');
                        hs.insert('/');
                        hs.insert('&');
                        hs.insert('|');
                        hs.insert('<');
                        hs.insert('>');
                        hs.insert('=');
                        hs.insert('~');
                        hs.insert('"');
                        hs.insert('%');
                        hs
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static HashSet<char> {
                    static LAZY: ::lazy_static::lazy::Lazy<HashSet<char>> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for SYMBOLS {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
    }
    pub(crate) mod token_type {
        use std::fmt::{Debug, Display};
        use crate::tokens::jack_tokens::{
            Keyword::{self, *},
            Token,
        };
        pub trait ValidToken: Display + Debug + PartialEq<TokenType> {}
        impl ValidToken for Token {}
        impl ValidToken for Keyword {}
        impl ValidToken for char {}
        impl ValidToken for i16 {}
        impl ValidToken for String {}
        pub enum TokenType {
            ClassVarDec,
            Constant,
            Name,
            BinaryOp,
            UnaryOp,
            Statement,
            SubroutineDec,
            Type,
            ReturnType,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TokenType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        TokenType::ClassVarDec => "ClassVarDec",
                        TokenType::Constant => "Constant",
                        TokenType::Name => "Name",
                        TokenType::BinaryOp => "BinaryOp",
                        TokenType::UnaryOp => "UnaryOp",
                        TokenType::Statement => "Statement",
                        TokenType::SubroutineDec => "SubroutineDec",
                        TokenType::Type => "Type",
                        TokenType::ReturnType => "ReturnType",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TokenType {
            #[inline]
            fn clone(&self) -> TokenType {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TokenType {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TokenType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TokenType {
            #[inline]
            fn eq(&self, other: &TokenType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for TokenType {}
        #[automatically_derived]
        impl ::core::cmp::Eq for TokenType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl ValidToken for TokenType {}
        impl PartialEq<Token> for TokenType {
            fn eq(&self, other: &Token) -> bool {
                match other {
                    Token::Keyword(k) => k == self,
                    Token::Symbol(c) => c == self,
                    Token::Identifier(_) => {
                        match self {
                            &TokenType::Name
                            | &TokenType::Type
                            | &TokenType::ReturnType => true,
                            _ => false,
                        }
                    }
                    Token::StringConstant(_) | Token::IntConstant(_) => {
                        self == &TokenType::Constant
                    }
                }
            }
        }
        impl Display for TokenType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    TokenType::ClassVarDec => {
                        f.write_fmt(format_args!("class var declaration"))
                    }
                    TokenType::Constant => f.write_fmt(format_args!("constant")),
                    TokenType::Name => f.write_fmt(format_args!("name")),
                    TokenType::BinaryOp => f.write_fmt(format_args!("binary op")),
                    TokenType::UnaryOp => f.write_fmt(format_args!("unary op")),
                    TokenType::Statement => f.write_fmt(format_args!("statement")),
                    TokenType::SubroutineDec => {
                        f.write_fmt(format_args!("subroutine declaration"))
                    }
                    TokenType::Type => f.write_fmt(format_args!("value type")),
                    TokenType::ReturnType => f.write_fmt(format_args!("return type")),
                }
            }
        }
        impl PartialEq<TokenType> for i16 {
            fn eq(&self, other: &TokenType) -> bool {
                other == &TokenType::Constant
            }
        }
        impl PartialEq<TokenType> for char {
            fn eq(&self, other: &TokenType) -> bool {
                match other {
                    TokenType::BinaryOp => {
                        match self {
                            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '<' | '>' | '=' => {
                                true
                            }
                            _ => false,
                        }
                    }
                    TokenType::UnaryOp => {
                        match self {
                            '-' | '~' => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
        }
        impl PartialEq<TokenType> for Keyword {
            fn eq(&self, other: &TokenType) -> bool {
                match other {
                    TokenType::Constant => {
                        match self {
                            True | False | This | Null => true,
                            _ => false,
                        }
                    }
                    TokenType::ClassVarDec => {
                        match self {
                            Static | Field => true,
                            _ => false,
                        }
                    }
                    TokenType::Statement => {
                        match self {
                            Let | If | While | Do | Return => true,
                            _ => false,
                        }
                    }
                    TokenType::SubroutineDec => {
                        match self {
                            Constructor | Function | Method => true,
                            _ => false,
                        }
                    }
                    TokenType::Type => {
                        match self {
                            Int | Char | Boolean => true,
                            _ => false,
                        }
                    }
                    TokenType::ReturnType => {
                        match self {
                            Void | Int | Char | Boolean => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
        }
        impl PartialEq<Keyword> for TokenType {
            fn eq(&self, other: &Keyword) -> bool {
                match self {
                    TokenType::Constant => {
                        match other {
                            True | False | This | Null => true,
                            _ => false,
                        }
                    }
                    TokenType::ClassVarDec => {
                        match other {
                            Static | Field => true,
                            _ => false,
                        }
                    }
                    TokenType::Statement => {
                        match other {
                            Let | If | While | Do | Return => true,
                            _ => false,
                        }
                    }
                    TokenType::SubroutineDec => {
                        match other {
                            Constructor | Function | Method => true,
                            _ => false,
                        }
                    }
                    TokenType::Type => {
                        match other {
                            Int | Char | Boolean => true,
                            _ => false,
                        }
                    }
                    TokenType::ReturnType => {
                        match other {
                            Void | Int | Char | Boolean => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
        }
        impl PartialEq<TokenType> for String {
            fn eq(&self, other: &TokenType) -> bool {
                other == &TokenType::Constant
            }
        }
    }
    pub(crate) mod vm_commands {
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
            Eq,
            GT,
            LT,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Comparison {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Comparison::Eq => "Eq",
                        Comparison::GT => "GT",
                        Comparison::LT => "LT",
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
                    Self::Eq => f.write_fmt(format_args!("eq")),
                    Self::GT => f.write_fmt(format_args!("gt")),
                    Self::LT => f.write_fmt(format_args!("lt")),
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
                    VmCommand::Label(label) => {
                        f.write_fmt(format_args!("label {0}", label))
                    }
                    VmCommand::Goto(label) => {
                        f.write_fmt(format_args!("goto {0}", label))
                    }
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
}
mod vm_translator {
    use std::borrow::Cow;
    use std::fmt::Display;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::{Path, PathBuf};
    use std::vec;
    use anyhow::{Result, bail};
    use crate::asm::{Asm, Dest, Instruction, Jump, ValidComp, Mode};
    use crate::tokens::vm_commands::{MemSegment as Seg, VmCommand, Comparison};
    pub mod parser {
        use anyhow::{anyhow, Result};
        use crate::tokens::vm_commands::{
            Comparison::{Eq, GT, LT},
            MemSegment as Seg, VmCommand,
        };
        pub fn parse(cmd: &str) -> Result<VmCommand> {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let command = match parts.len() {
                1 => {
                    match parts[0] {
                        "add" => VmCommand::Add,
                        "sub" => VmCommand::Sub,
                        "neg" => VmCommand::Neg,
                        "eq" => VmCommand::Compare(Eq),
                        "gt" => VmCommand::Compare(GT),
                        "lt" => VmCommand::Compare(LT),
                        "and" => VmCommand::And,
                        "or" => VmCommand::Or,
                        "not" => VmCommand::Not,
                        "return" => VmCommand::Return,
                        _ => {
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("No one word command \"{0}\"", cmd),
                                    );
                                    error
                                }),
                            );
                        }
                    }
                }
                2 => {
                    match parts[0] {
                        "label" => VmCommand::Label(parts[1]),
                        "goto" => VmCommand::Goto(parts[1]),
                        "if-goto" => VmCommand::IfGoto(parts[1]),
                        _ => {
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("No two word command \"{0}\"", cmd),
                                    );
                                    error
                                }),
                            );
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
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("No three word command \"{0}\"", cmd),
                                    );
                                    error
                                }),
                            );
                        }
                    }
                }
                _ => {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("\"{0}\" is not a valid VM command", cmd),
                            );
                            error
                        }),
                    );
                }
            };
            Ok(command)
        }
    }
    use asm_macro::asm;
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
                        let vm_cmd = parser::parse(&cmd)
                            .expect("could not parse command");
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
                    Asm::Asm(Instruction::from(256)),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::A, Jump::Never)),
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::D, Jump::Never)),
                    Asm::Comment(std::borrow::Cow::Borrowed("call Sys.init")),
                    Asm::At(std::borrow::Cow::Borrowed("Sys.init")),
                    Asm::Asm(Instruction::c(Dest::None, ValidComp::Zero, Jump::JMP)),
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
        fn generate_asm(&mut self, command: VmCommand, comment: bool) -> Result<()> {
            let mut res = ::alloc::vec::Vec::new();
            if comment {
                res.push(
                    Asm::Comment(
                        Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", command));
                            res
                        }),
                    ),
                );
            }
            match command {
                VmCommand::Add => {
                    self.binary_op(
                        Asm::Asm(Instruction::c(Dest::M, ValidComp::DPlusM, Jump::Never)),
                    )
                }
                VmCommand::Sub => {
                    self.binary_op(
                        Asm::Asm(
                            Instruction::c(Dest::M, ValidComp::MMinusD, Jump::Never),
                        ),
                    )
                }
                VmCommand::Neg => {
                    self.unary_op(
                        Asm::Asm(Instruction::c(Dest::M, ValidComp::NegM, Jump::Never)),
                    )
                }
                VmCommand::Compare(comp) => self.comparison(comp),
                VmCommand::And => {
                    self.binary_op(
                        Asm::Asm(Instruction::c(Dest::M, ValidComp::DAndM, Jump::Never)),
                    )
                }
                VmCommand::Or => {
                    self.binary_op(
                        Asm::Asm(Instruction::c(Dest::M, ValidComp::DOrM, Jump::Never)),
                    )
                }
                VmCommand::Not => {
                    self.unary_op(
                        Asm::Asm(Instruction::c(Dest::M, ValidComp::NotM, Jump::Never)),
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
                                Asm::At(
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
                                Asm::At(std::borrow::Cow::Borrowed("$$RETURN")),
                                Asm::Asm(
                                    Instruction::c(Dest::None, ValidComp::Zero, Jump::JMP),
                                ),
                            ])
                    } else {
                        self.asm
                            .extend([
                                Asm::Comment(
                                    std::borrow::Cow::Borrowed("Shared return subroutine"),
                                ),
                                Asm::Label(std::borrow::Cow::Borrowed("$$RETURN")),
                                Asm::Asm(Instruction::from(5)),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::A, Jump::Never),
                                ),
                                Asm::LCL,
                                Asm::Asm(
                                    Instruction::c(Dest::A, ValidComp::MMinusD, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::M, Jump::Never),
                                ),
                                Asm::R14,
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::SP,
                                Asm::Asm(
                                    Instruction::c(Dest::A, ValidComp::MMinusOne, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::M, Jump::Never),
                                ),
                                Asm::ARG,
                                Asm::Asm(
                                    Instruction::c(Dest::A, ValidComp::M, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::APlusOne, Jump::Never),
                                ),
                                Asm::SP,
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::LCL,
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::MMinusOne, Jump::Never),
                                ),
                                Asm::R13,
                                Asm::Asm(
                                    Instruction::c(Dest::AM, ValidComp::D, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::M, Jump::Never),
                                ),
                                Asm::THAT,
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::R13,
                                Asm::Asm(
                                    Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::M, Jump::Never),
                                ),
                                Asm::THIS,
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::R13,
                                Asm::Asm(
                                    Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::M, Jump::Never),
                                ),
                                Asm::ARG,
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::R13,
                                Asm::Asm(
                                    Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::D, ValidComp::M, Jump::Never),
                                ),
                                Asm::LCL,
                                Asm::Asm(
                                    Instruction::c(Dest::M, ValidComp::D, Jump::Never),
                                ),
                                Asm::R14,
                                Asm::Asm(
                                    Instruction::c(Dest::A, ValidComp::M, Jump::Never),
                                ),
                                Asm::Asm(
                                    Instruction::c(Dest::None, ValidComp::Zero, Jump::JMP),
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
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::A, ValidComp::MMinusOne, Jump::Never)),
                ]);
            self.asm.push(last_line);
        }
        fn comparison(&mut self, comparison: Comparison) {
            let counter = self.comp_count;
            self.comp_count += 1;
            self.binary_op(
                Asm::Asm(Instruction::c(Dest::MD, ValidComp::MMinusD, Jump::Never)),
            );
            self.asm
                .extend(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            Asm::At(
                                std::borrow::Cow::Owned({
                                    let res = ::alloc::fmt::format(
                                        format_args!("END_COMP{0}", counter),
                                    );
                                    res
                                }),
                            ),
                            match comparison {
                                Comparison::Eq => {
                                    Asm::Asm(
                                        Instruction::c(Dest::None, ValidComp::D, Jump::JNE),
                                    )
                                }
                                Comparison::GT => {
                                    Asm::Asm(
                                        Instruction::c(Dest::None, ValidComp::D, Jump::JLE),
                                    )
                                }
                                Comparison::LT => {
                                    Asm::Asm(
                                        Instruction::c(Dest::None, ValidComp::D, Jump::JGE),
                                    )
                                }
                            },
                        ]),
                    ),
                );
            self.asm
                .extend([
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::DPlusOne, Jump::Never)),
                    Asm::Label(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(
                                format_args!("END_COMP{0}", counter),
                            );
                            res
                        }),
                    ),
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::A, ValidComp::MMinusOne, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::MMinusD, Jump::Never)),
                ]);
        }
        fn binary_op(&mut self, last_line: Asm<'a>) {
            self.asm
                .extend(
                    [
                        Asm::SP,
                        Asm::Asm(
                            Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                        ),
                        Asm::Asm(Instruction::c(Dest::D, ValidComp::M, Jump::Never)),
                        Asm::Asm(
                            Instruction::c(Dest::A, ValidComp::AMinusOne, Jump::Never),
                        ),
                    ]
                        .into_iter()
                        .chain(std::iter::once(last_line)),
                );
        }
        pub fn push_segment<T>(&mut self, segment: T, n: i16)
        where
            T: Display,
        {
            self.segment(segment, n);
            self.asm
                .extend([
                    Asm::Asm(Instruction::c(Dest::A, ValidComp::DPlusM, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::M, Jump::Never)),
                ]);
            self.push();
        }
        pub fn segment<T: Display>(&mut self, segment: T, n: i16) {
            self.asm
                .extend([
                    Asm::from(n),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::A, Jump::Never)),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", segment));
                            res
                        }),
                    ),
                ]);
        }
        pub fn pop_segment<T>(&mut self, segment: T, n: i16)
        where
            T: Display,
        {
            self.segment(segment, n);
            self.asm
                .extend([
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::DPlusM, Jump::Never)),
                    Asm::SP,
                    Asm::Asm(
                        Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                    ),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::DPlusM, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::A, ValidComp::DMinusM, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::DMinusA, Jump::Never)),
                ]);
        }
        fn push_value<T>(&mut self, var: T, mode: Mode)
        where
            T: Display,
        {
            self.asm
                .push(
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", var));
                            res
                        }),
                    ),
                );
            self.asm
                .push(
                    match mode {
                        Mode::A => {
                            Asm::Asm(Instruction::c(Dest::D, ValidComp::A, Jump::Never))
                        }
                        Mode::M => {
                            Asm::Asm(Instruction::c(Dest::D, ValidComp::M, Jump::Never))
                        }
                    },
                );
            self.push();
        }
        fn push_constant(&mut self, var: i16) {
            if var < 0 {
                self.asm
                    .extend([
                        Asm::from(!var),
                        Asm::Asm(Instruction::c(Dest::D, ValidComp::NotA, Jump::Never)),
                    ]);
            } else {
                self.asm
                    .extend([
                        Asm::from(var),
                        Asm::Asm(Instruction::c(Dest::D, ValidComp::A, Jump::Never)),
                    ]);
            }
            self.push()
        }
        fn push(&mut self) {
            self.asm
                .extend([
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::MPlusOne, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::A, ValidComp::MMinusOne, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::D, Jump::Never)),
                ])
        }
        fn pop_value<T>(&mut self, var: T)
        where
            T: Display,
        {
            self.asm
                .extend([
                    Asm::SP,
                    Asm::Asm(
                        Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                    ),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::M, Jump::Never)),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", var));
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::D, Jump::Never)),
                ]);
        }
        fn def_label(&mut self, label: String) {
            self.asm
                .push(
                    Asm::Label(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", label));
                            res
                        }),
                    ),
                );
        }
        fn goto(&mut self, label: String) {
            self.asm
                .extend([
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", label));
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::None, ValidComp::Zero, Jump::JMP)),
                ]);
        }
        fn if_goto(&mut self, label: String) {
            self.asm
                .extend([
                    Asm::SP,
                    Asm::Asm(
                        Instruction::c(Dest::AM, ValidComp::MMinusOne, Jump::Never),
                    ),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::M, Jump::Never)),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", label));
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::None, ValidComp::D, Jump::JNE)),
                ]);
        }
        fn func(&mut self, fn_name: &str, n_vars: i16) {
            self.curr_func = String::from(fn_name);
            self.asm
                .extend([
                    Asm::Label(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", fn_name));
                            res
                        }),
                    ),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", n_vars));
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::A, Jump::Never)),
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::AM, ValidComp::DPlusM, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::DMinusOne, Jump::Never)),
                    Asm::Label(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}$LocalLoop", fn_name),
                            );
                            res
                        }),
                    ),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}$LocalLoopEnd", fn_name),
                            );
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::None, ValidComp::D, Jump::JLT)),
                    Asm::LCL,
                    Asm::Asm(Instruction::c(Dest::A, ValidComp::DPlusM, Jump::Never)),
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::Zero, Jump::Never)),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}$LocalLoop", fn_name),
                            );
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::DMinusOne, Jump::JMP)),
                    Asm::Label(
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
            self.push_value(&return_label, Mode::A);
            self.push_value("LCL", Mode::M);
            self.push_value("ARG", Mode::M);
            self.push_value("THIS", Mode::M);
            self.push_value("THAT", Mode::M);
            self.asm
                .extend([
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(format_args!("{0}", n_args));
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::A, Jump::Never)),
                    Asm::Asm(Instruction::from(5)),
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::DPlusA, Jump::Never)),
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::MMinusD, Jump::Never)),
                    Asm::ARG,
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::D, Jump::Never)),
                    Asm::SP,
                    Asm::Asm(Instruction::c(Dest::D, ValidComp::M, Jump::Never)),
                    Asm::LCL,
                    Asm::Asm(Instruction::c(Dest::M, ValidComp::D, Jump::Never)),
                    Asm::At(
                        std::borrow::Cow::Owned({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}", function),
                            );
                            res
                        }),
                    ),
                    Asm::Asm(Instruction::c(Dest::None, ValidComp::Zero, Jump::JMP)),
                    Asm::Label(
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
use crate::jack_compiler::compilation_engine::CompilationEngine;
use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};
#[macro_use]
extern crate lazy_static;
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
    let mut parser = CompilationEngine::new();
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
    for file in files {
        parser.compile(file).expect("error");
    }
}
