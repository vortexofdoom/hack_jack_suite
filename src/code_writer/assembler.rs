use anyhow::{anyhow, Result};
use bitflags::bitflags;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::Binary;
use std::fs::File;
use std::io::{BufRead, BufReader};

static BUILTIN_LABELS: Lazy<HashMap<&'static str, u16>> = Lazy::new(|| {
    let mut labels = HashMap::new();
    labels.insert("SP", 0);
    labels.insert("LCL", 1);
    labels.insert("ARG", 2);
    labels.insert("THIS", 3);
    labels.insert("THAT", 4);
    labels.insert("R0", 0);
    labels.insert("R1", 1);
    labels.insert("R2", 2);
    labels.insert("R3", 3);
    labels.insert("R4", 4);
    labels.insert("R5", 5);
    labels.insert("R6", 6);
    labels.insert("R7", 7);
    labels.insert("R8", 8);
    labels.insert("R9", 9);
    labels.insert("R10", 10);
    labels.insert("R11", 11);
    labels.insert("R12", 12);
    labels.insert("R13", 13);
    labels.insert("R14", 14);
    labels.insert("R15", 15);
    labels.insert("SCREEN", 16384);
    labels.insert("KBD", 24576);
    labels
});

bitflags! {
    struct AsmFlags: i16 {
        // Jump bits
        const JGT       = 1;        // Jump if greater than
        const JEQ       = 1 << 1;   // Jump if equal to
        const JLT       = 1 << 2;   // jump if less than

        // Destination bits
        const DEST_M    = 1 << 3;   // Store in M register (Address specified by A register)
        const DEST_D    = 1 << 4;   // Store in D register
        const DEST_A    = 1 << 5;   // Store in A register

        // Control bits
        // These are semantically named based on the ALU chip of the CPU
        const C_NO      = 1 << 6;
        const C_F       = 1 << 7;
        const C_NA      = 1 << 8;
        const C_ZA      = 1 << 9;
        const C_ND      = 1 << 10;
        const C_ZD      = 1 << 11;
        const ADDR      = 1 << 12;  // A as Address vs value (M vs A)

        // C instruction bits
        // For C instructions these are always set in the Hack specification, but never used within the Hack architecture
        // We define all of them so that we can use this under the hood for both A and C instructions
        const B0        = 1 << 13;
        const B1        = 1 << 14;
        const C         = 1 << 15;

        // Combined flags for J and D bits, since they are easily enumerable and have intuitive semantics
        // Destination combinations might not be needed, but can revisit that later
        const JGE       = 0b011;    // Jump if greater than or equal to
        const JNE       = 0b101;    // Jump if not equal to
        const JLE       = 0b110;    // Jump if less than or equal to
        const JMP       = 0b111;    // Unconditional Jump
        const DEST_MD   = 0b011 << 3;
        const DEST_AM   = 0b101 << 3;
        const DEST_AD   = 0b110 << 3;
        const DEST_AMD  = 0b111 << 3;

        // All C bits
        const C_BITS    = Self::C_NO.bits() | Self::C_F.bits() | Self::C_NA.bits() | Self::C_ZA.bits() | Self::C_ND.bits() | Self::C_ZD.bits();
    }
}

impl Default for AsmFlags {
    fn default() -> Self {
        Self::C | Self::B1 | Self::B0
    }
}

impl ToString for AsmFlags {
    fn to_string(&self) -> String {
        let dest = match self.intersection(Self::DEST_AMD) {
            Self::DEST_A => "A=",
            Self::DEST_M => "M=",
            Self::DEST_D => "D=",
            Self::DEST_AD => "AD=",
            Self::DEST_AM => "AM=",
            Self::DEST_MD => "MD=",
            Self::DEST_AMD => "AMD=",
            _ => "",
        };

        let jump = match self.intersection(Self::JMP) {
            Self::JEQ => ";JEQ",
            Self::JGE => ";JGE",
            Self::JGT => ";JGT",
            Self::JLE => ";JLE",
            Self::JLT => ";JLT",
            Self::JNE => ";JNE",
            Self::JMP => ";JMP",
            _ => "",
        };

        format!(
            "{dest}{}{jump}",
            CBits::try_from(self).expect("All valid bit configurations mapped")
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
/// An enum to provide an exhaustive list of supported `c` bit configurations without exposing intermediate constants
pub enum CBits {
    Zero    = 0b0_101010 << 6,
    One     = 0b0_111111 << 6,
    NegOne  = 0b0_111010 << 6,
    D       = 0b0_001100 << 6,
    A       = 0b0_110000 << 6,
    M       = 0b1_110000 << 6,
    NotD    = 0b0_001101 << 6,
    NotA    = 0b0_110001 << 6,
    NotM    = 0b1_110001 << 6,
    NegD    = 0b0_001111 << 6,
    NegA    = 0b0_110011 << 6,
    NegM    = 0b1_110011 << 6,
    DPlus1  = 0b0_011111 << 6,
    APlus1  = 0b0_110111 << 6,
    MPlus1  = 0b1_110111 << 6,
    DMinus1 = 0b0_001110 << 6,
    AMinus1 = 0b0_110010 << 6,
    MMinus1 = 0b1_110010 << 6,
    DPlusA  = 0b0_000010 << 6,
    DPlusM  = 0b1_000010 << 6,
    DMinusA = 0b0_010011 << 6,
    DMinusM = 0b1_010011 << 6,
    AMinusD = 0b0_000111 << 6,
    MMinusD = 0b1_000111 << 6,
    DAndA   = 0b0_000000 << 6, // lol
    DAndM   = 0b1_000000 << 6,
    DOrA    = 0b0_010101 << 6,
    DOrM    = 0b1_010101 << 6,
}

// might be able to just add all the variants eventually and implement From<AsmFlags> instead
impl TryFrom<&AsmFlags> for CBits {
    type Error = anyhow::Error;
    fn try_from(value: &AsmFlags) -> std::result::Result<Self, Self::Error> {
        let bits = (value.bits >> 6) & 0b1111111;
        //let bit_7 = bits & 0 << 7 == bits;
        match (bits & 0b111111, bits >> 6 == 0) {
            // official opcodes
            (0b101010, _) => Ok(Self::Zero),
            (0b111111, _) => Ok(Self::One),
            (0b111010, _) => Ok(Self::NegOne),
            (0b001100, _) => Ok(Self::D),
            (0b110000, true) => Ok(Self::A),
            (0b110000, false) => Ok(Self::M),
            (0b001101, _) => Ok(Self::NotD),
            (0b110001, true) => Ok(Self::NotA),
            (0b110001, false) => Ok(Self::NotM),
            (0b001111, _) => Ok(Self::NegD),
            (0b110011, true) => Ok(Self::NegA),
            (0b110011, false) => Ok(Self::NegM),
            (0b011111, _) => Ok(Self::DPlus1),
            (0b110111, true) => Ok(Self::APlus1),
            (0b110111, false) => Ok(Self::MPlus1),
            (0b001110, _) => Ok(Self::DMinus1),
            (0b110010, true) => Ok(Self::AMinus1),
            (0b110010, false) => Ok(Self::MMinus1),
            (0b000010, true) => Ok(Self::DPlusA),
            (0b000010, false) => Ok(Self::DPlusM),
            (0b010011, true) => Ok(Self::DMinusA),
            (0b010011, false) => Ok(Self::DMinusM),
            (0b000111, true) => Ok(Self::AMinusD),
            (0b000111, false) => Ok(Self::MMinusD),
            (0b000000, true) => Ok(Self::DAndA), // lol
            (0b000000, false) => Ok(Self::DAndM),
            (0b010101, true) => Ok(Self::DOrA),
            (0b010101, false) => Ok(Self::DOrM),
            // Unofficial
            (
                _c @ (0b111011 | 0b001000 | 0b011000 | 0b100000 | 0b100100 | 0b101000 | 0b101100
                | 0b101111 | 0b111000 | 0b111101),
                _,
            ) => Ok(Self::Zero),
            (
                _c @ (0b001001 | 0b011001 | 0b100001 | 0b100101 | 0b101001 | 0b101011 | 0b101101
                | 0b101110 | 0b111001 | 0b111100),
                _,
            ) => Ok(Self::NegOne),
            (_c @ (0b001010 | 0b011011 | 0b011101), _) => Ok(Self::D),
            (_c @ (0b100010 | 0b100111 | 0b110101), true) => Ok(Self::A),
            (_c @ (0b100010 | 0b100111 | 0b110101), false) => Ok(Self::M),
            (_c @ (0b001011 | 0b011100 | 0b011010), _) => Ok(Self::NotD),
            (_c @ (0b100011 | 0b100110 | 0b110100), true) => Ok(Self::NotA),
            (_c @ (0b100011 | 0b100110 | 0b110100), false) => Ok(Self::NotM),
            _ => Err(anyhow!("unsupported bit configuration")),
        }
    }
}

impl TryFrom<AsmFlags> for CBits {
    type Error = anyhow::Error;
    fn try_from(value: AsmFlags) -> std::result::Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl From<CBits> for AsmFlags {
    fn from(value: CBits) -> Self {
        Self { bits: value as i16 }
    }
}

impl std::fmt::Display for CBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CBits::Zero => "0",
            CBits::One => "1",
            CBits::NegOne => "-1",
            CBits::D => "D",
            CBits::A => "A",
            CBits::M => "M",
            CBits::NotD => "!D",
            CBits::NotA => "!A",
            CBits::NotM => "!M",
            CBits::NegD => "-D",
            CBits::NegA => "-A",
            CBits::NegM => "-M",
            CBits::DPlus1 => "D+1",
            CBits::APlus1 => "A+1",
            CBits::MPlus1 => "M+1",
            CBits::DMinus1 => "D-1",
            CBits::AMinus1 => "A-1",
            CBits::MMinus1 => "M-1",
            CBits::DPlusA => "D+A",
            CBits::DPlusM => "D+M",
            CBits::DMinusA => "D-A",
            CBits::DMinusM => "D-M",
            CBits::AMinusD => "A-D",
            CBits::MMinusD => "M-D",
            CBits::DAndA => "D&A",
            CBits::DAndM => "D&M",
            CBits::DOrA => "D|A",
            CBits::DOrM => "D|M",
        };
        write!(f, "{str}")
    }
}

pub enum InstructionType {
    A,
    C,
}

// wrapper struct
pub struct Instruction {
    inner: AsmFlags,
}

impl From<i16> for Instruction {
    fn from(value: i16) -> Self {
        Self {
            inner: AsmFlags { bits: value },
        }
    }
}

impl From<u16> for Instruction {
    #[allow(overflowing_literals)]
    fn from(value: u16) -> Self {
        Self {
            inner: AsmFlags { bits: value as i16 },
        }
    }
}

impl Binary for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.inner)
    }
}

impl Instruction {
    pub fn is_c_instr(&self) -> bool {
        self.inner.contains(AsmFlags::C)
    }
}

pub struct Assembler {
    pub labels: HashMap<String, u16>,
    pub var_counter: u16,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: HashMap::new(),
            var_counter: 15, // Starts at 15 so we can increment it pre insertion
        }
    }

    // Helper function to abstract over checking the static list first, then the labels unique to this assembly
    fn get_label(&self, label: &str) -> Option<&u16> {
        if let Some(i) = BUILTIN_LABELS.get(label) {
            Some(i)
        } else if let Some(i) = self.labels.get(label) {
            Some(i)
        } else {
            None
        }
    }

    fn parse_a_instruction(&mut self, input: &str) -> Result<Instruction> {
        match input.parse::<i16>() {
            Ok(n) if n >= 0 => Ok(Instruction::from(n)),
            // If the address given is not a valid positive signed 16-bit integer, interpret it as a variable
            // Could also return an error, But I think most written assembly would not have intentionally numericized labels
            _ => {
                if let Some(&addr) = self.get_label(input) {
                    Ok(Instruction::from(addr))
                } else {
                    self.var_counter += 1;
                    self.labels.insert(String::from(input), self.var_counter);
                    Ok(Instruction::from(self.var_counter))
                }
            }
        }
    }

    fn parse_c_instruction(&self, input: &str) -> Result<Instruction> {
        let mut inst = AsmFlags::default();

        // There will always be a computation field, so we set the bounds now
        let mut comp_start = 0;
        let mut comp_end = input.len();

        // DEST
        // All valid commands with a destination field include '='
        // Technically this current implementation allows including valid destinations in a non-standard order
        if let Some(i) = input.find('=') {
            // we know the start of the computation field comes immediately after the '='
            comp_start = i + 1;
            let dest = &input[..i];

            // Making sure that only valid destinations are used.
            if !dest.chars().all(|c| "AMD".contains(c)) {
                return Err(anyhow!(
                    "'{dest}' contains an invalid destination character."
                ));
            }

            // Allowing the destinations in any order might be too permissive, but it's fine for now
            inst.set(AsmFlags::DEST_A, dest.contains('A'));
            inst.set(AsmFlags::DEST_M, dest.contains('M'));
            inst.set(AsmFlags::DEST_D, dest.contains('D'));
        }

        // JUMP
        if let Some(i) = input.find(';') {
            comp_end = i;
            let jump = &input[i + 1..];
            match jump {
                "JGT" => inst |= AsmFlags::JGT,
                "JEQ" => inst |= AsmFlags::JEQ,
                "JGE" => inst |= AsmFlags::JGE,
                "JLT" => inst |= AsmFlags::JLT,
                "JNE" => inst |= AsmFlags::JNE,
                "JLE" => inst |= AsmFlags::JLE,
                "JMP" => inst |= AsmFlags::JMP,
                _ => panic!("Semicolon requires a valid jump command!"),
            }
        }

        // COMP
        // RIP glorious messy control flow
        // You were too clever, too permissive, and too unreadable.
        inst |= match &input[comp_start..comp_end] {
            "0" => AsmFlags::from(CBits::Zero),
            "1" => AsmFlags::from(CBits::One),
            "-1" => AsmFlags::from(CBits::NegOne),
            "D" => AsmFlags::from(CBits::D),
            "A" => AsmFlags::from(CBits::A),
            "M" => AsmFlags::from(CBits::M),
            "!D" => AsmFlags::from(CBits::NotD),
            "!A" => AsmFlags::from(CBits::NotA),
            "!M" => AsmFlags::from(CBits::NotM),
            // Making the executive decision to allow the real bitwise NOT operator
            // Especially because it's used instead of ! in the Jack standard
            "~D" => AsmFlags::from(CBits::NotD),
            "~A" => AsmFlags::from(CBits::NotA),
            "~M" => AsmFlags::from(CBits::NotM),
            // Back to your regularly scheduled standard
            "-D" => AsmFlags::from(CBits::NegD),
            "-A" => AsmFlags::from(CBits::NegA),
            "-M" => AsmFlags::from(CBits::NegM),
            "D+1" => AsmFlags::from(CBits::DPlus1),
            "A+1" => AsmFlags::from(CBits::APlus1),
            "M+1" => AsmFlags::from(CBits::MPlus1),
            "D-1" => AsmFlags::from(CBits::DMinus1),
            "A-1" => AsmFlags::from(CBits::AMinus1),
            "M-1" => AsmFlags::from(CBits::MMinus1),
            "D-A" => AsmFlags::from(CBits::DMinusA),
            "D-M" => AsmFlags::from(CBits::DMinusM),
            "A-D" => AsmFlags::from(CBits::AMinusD),
            "M-D" => AsmFlags::from(CBits::MMinusD),
            // Since all of these are semantically equivalent, as long as it's a perfect match we'll allow either order
            "D+M" | "M+D" => AsmFlags::from(CBits::DPlusM),
            "D+A" | "A+D" => AsmFlags::from(CBits::DPlusA),
            "D&A" | "A&D" => AsmFlags::from(CBits::DAndA),
            "D&M" | "M&D" => AsmFlags::from(CBits::DAndM),
            "D|A" | "A|D" => AsmFlags::from(CBits::DOrA),
            "D|M" | "M|D" => AsmFlags::from(CBits::DOrM),
            _ => return Err(anyhow!("invalid or unsupported computation field")),
        };

        Ok(Instruction { inner: inst })
    }

    pub fn translate(&mut self, input: &str) -> Result<Instruction> {
        // A or C instruction
        if let Some(i) = input.strip_prefix('@') {
            self.parse_a_instruction(i)
        } else {
            self.parse_c_instruction(input)
        }
    }

    pub fn assemble(&mut self, asm: &[String]) -> Result<Vec<Instruction>> {
        // first pass
        let mut line: u16 = 0;
        for com in asm {
            let mut chars = com.chars();
            if let (Some('('), Some(')')) = (chars.next(), chars.next_back()) {
                //println!("({com})");
                if self.get_label(chars.as_str()).is_none() {
                    self.labels.insert(chars.collect(), line);
                }
            } else {
                line += 1;
            }
        }
        Ok(asm
            .iter()
            .filter(|&c| !c.contains('('))
            .map(|c| self.translate(c).expect("valid asm only"))
            .collect())
    }
}

fn write_bin() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();
    let mut asm = vec![];
    let mut assembler = Assembler::new();
    if let Ok(f) = File::open(filename) {
        let reader = BufReader::new(f);
        for line in reader.lines().flatten() {
            let cmd = strip_line(&line);
            if !cmd.is_empty() {
                asm.push(cmd);
            }
        }
    }
    let bin = assembler.assemble(&asm).unwrap();
    for b in bin {
        println!("{b:016b}");
    }
}

fn strip_line(input: &str) -> String {
    input
        .find("//")
        .map(|i| &input[..i])
        .unwrap_or(input)
        .replace(' ', "")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn valid_c_bits() {
        let c_bits = CBits::try_from(AsmFlags::C_NO | AsmFlags::C_F | AsmFlags::C_ND).unwrap();
        assert_eq!(c_bits, CBits::DMinusA);
    }
}
