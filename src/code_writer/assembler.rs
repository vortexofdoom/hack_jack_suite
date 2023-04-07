use std::collections::HashMap;
use std::fmt::Binary;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitOrAssign;
use once_cell::sync::Lazy;
use bitflags::bitflags;
use anyhow::{anyhow, Result};

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
        const C_NO        = 1 << 6;
        const C_F         = 1 << 7;
        const C_NA        = 1 << 8;
        const C_ZA        = 1 << 9;
        const C_ND        = 1 << 10;
        const C_ZD        = 1 << 11;
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
        
        // Relevant c-bit combinations for the assembly language. Comments are how they literally resolve into ALU output
        const C_BITS    = Self::C_NO.bits() | Self::C_F.bits() | Self::C_NA.bits() | Self::C_ZA.bits() | Self::C_ND.bits() | Self::C_ZD.bits();
        
        // Probably gonna use the enum solution
        // 0 + 0      => 0
        //const CTRL_0         = Self::F.bits() | Self::ZA.bits() | Self::ZD.bits();   
        
        // !(-1 + -1) => 1
        
        // // -1 + 0     => -1
        // const CTRL_NEG_1     = Self::CTRL_0.bits() | Self::ND.bits();
        
        // // D & -1     => D
        // const CTRL_D         = Self::NA.bits() | Self::ZA.bits();
        
        // // -1 & AM    => A
        // const CTRL_A        = Self::ND.bits() | Self::ZD.bits();
        
        // // !(D & -1)  => !D
        // const CTRL_NOT_D     = Self::CTRL_D.bits() | Self::NO.bits();
        
        // // !(-1 & A)  => !A
        // const CTRL_NOT_A     = Self::CTRL_A.bits() | Self::NO.bits();
        
        // // !(D + -1)  => -D
        // const CTRL_NEG_D     = Self::CTRL_NOT_D.bits() | Self::F.bits();
        
        // // !(-1 + A)  => -A
        // const CTRL_NEG_A     = Self::CTRL_NOT_A.bits() | Self::F.bits();
        
        // // !(!D + -1) => D + 1
        // const CTRL_D_PLUS_1  = Self::C_BITS.bits() & !Self::ZD.bits();
        
        // // !(-1 + !A) => A + 1
        // const CTRL_A_PLUS_1  = Self::C_BITS.bits() & !Self::ZA.bits();
        
        // // (D + -1)     => (D - 1)
        // const CTRL_D_MINUS_1 = Self::F.bits() | Self::NA.bits() | Self::ZA.bits();
        
        // // (-1 + A)     => (A - 1)
        // const CTRL_A_MINUS_1 = Self::F.bits() | Self::ND.bits() | Self::ZD.bits();
        
        // // !(!D + A)  => (D - A)
        // const CTRL_D_SUB_A   = Self::ND.bits() | Self::F.bits() | Self::NO.bits();
        
        // // !(D + !A)  => (A - D)
        // const CTRL_A_SUB_D   = Self::NO.bits() | Self::F.bits() | Self::NA.bits();
        
        // // !(!D & !A) => (D | A)
        // const CTRL_D_OR_A    = Self::NO.bits() | Self::NA.bits() | Self::ND.bits();

        // // 
        // const CMP_M         = Self::CMP_A.bits() | Self::ADDR.bits();
        // const CMP_NOT_M     = Self::CMP_NOT_A.bits() | Self::ADDR.bits();
        // const CMP_NEG_M     = Self::CMP_NEG_A.bits() | Self::ADDR.bits();
        // const CMP_M_PLUS_1  = Self::CMP_A_PLUS_1.bits() | Self::ADDR.bits();
        // const CMP_M_MINUS_1 = Self::CMP_
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

        format!("{dest}{}{jump}", Ctrl::try_from(self).expect("All valid bit configurations mapped"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum Ctrl {
    Zero    = 0b__101010 << 6,
    One     = 0b__111111 << 6,
    NegOne  = 0b__111010 << 6,
    D       = 0b__001100 << 6,
    A       = 0b__110000 << 6,
    M       = 0b1_110000 << 6,
    NotD    = 0b__001101 << 6,
    NotA    = 0b__110001 << 6,
    NotM    = 0b1_110001 << 6,
    NegD    = 0b__001111 << 6,
    NegA    = 0b__110011 << 6,
    NegM    = 0b1_110011 << 6,
    DPlus1  = 0b__011111 << 6,
    APlus1  = 0b__110111 << 6,
    MPlus1  = 0b1_110111 << 6,
    DMinus1 = 0b__001110 << 6,
    AMinus1 = 0b__110010 << 6, 
    MMinus1 = 0b1_110010 << 6,
    DPlusA  = 0b__000010 << 6,
    DPlusM  = 0b1_000010 << 6,
    DMinusA = 0b__010011 << 6,
    DMinusM = 0b1_010011 << 6,
    AMinusD = 0b__000111 << 6,
    MMinusD = 0b1_000111 << 6,
    DAndA   = 0b__000000 << 6, // lol
    DAndM   = 0b1_000000 << 6,
    DOrA    = 0b__010101 << 6,
    DOrM    = 0b1_010101 << 6,
}

// might be able to just add all the variants eventually and implement From<i16> instead
impl TryFrom<&AsmFlags> for Ctrl {
    type Error = ();
    fn try_from(value: &AsmFlags) -> std::result::Result<Self, Self::Error> {
        let bits  = (value.bits >> 6) & 0b1111111;
        let bit_7 = bits & 0 << 7 == bits;
        match (bits & 0 << 7, bits >> 6 == 0) {
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
            (_c @ (0b111011 | 0b001000 | 0b011000 | 0b100000 | 0b100100 
            | 0b101000 | 0b101100 | 0b101111 | 0b111000 | 0b111101), _) => Ok(Self::Zero),
            (_c @ (0b001001 | 0b011001 | 0b100001 | 0b100101 | 0b101001
            | 0b101011 | 0b101101 | 0b101110 | 0b111001 | 0b111100), _) => Ok(Self::NegOne),
            (_c @ (0b001010 | 0b011011 | 0b011101), _) => Ok(Self::D),
            (_c @ (0b100010 | 0b100111 | 0b110101), true) => Ok(Self::A), 
            (_c @ (0b100010 | 0b100111 | 0b110101), false) => Ok(Self::M),
            (_c @ (0b001011 | 0b011100 | 0b011010), _) => Ok(Self::NotD),
            (_c @ (0b100011 | 0b100110 | 0b110100), true) => Ok(Self::NotA),
            (_c @ (0b100011 | 0b100110 | 0b110100), false) => Ok(Self::NotM),
            _ => todo!("implement unofficial instructions"),
        }
    }
}

impl From<Ctrl> for AsmFlags {
    fn from(value: Ctrl) -> Self {
        Self { bits: value as i16 }
    }
}

impl BitOrAssign<Ctrl> for AsmFlags {
    fn bitor_assign(&mut self, rhs: Ctrl) {
        *self |= Self::from(rhs)
    }

}

impl std::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Ctrl::Zero => "0",
            Ctrl::One => "1",
            Ctrl::NegOne => "-1",
            Ctrl::D => "D",
            Ctrl::A => "A",
            Ctrl::M => "M",
            Ctrl::NotD => "!D",
            Ctrl::NotA => "!A",
            Ctrl::NotM => "!M",
            Ctrl::NegD => "-D",
            Ctrl::NegA => "-A",
            Ctrl::NegM => "-M",
            Ctrl::DPlus1 => "D+1",
            Ctrl::APlus1 => "A+1",
            Ctrl::MPlus1 => "M+1",
            Ctrl::DMinus1 => "D-1",
            Ctrl::AMinus1 => "A-1",
            Ctrl::MMinus1 => "M-1",
            Ctrl::DPlusA => "D+A",
            Ctrl::DPlusM => "D+M",
            Ctrl::DMinusA => "D-A",
            Ctrl::DMinusM => "D-M",
            Ctrl::AMinusD => "A-D",
            Ctrl::MMinusD => "M-D",
            Ctrl::DAndA => "D&A",
            Ctrl::DAndM => "D&M",
            Ctrl::DOrA => "D|A",
            Ctrl::DOrM => "D|M",
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
            inner: AsmFlags { bits: value } 
        }
    }
}

impl From<u16> for Instruction {
    #[allow(overflowing_literals)]
    fn from(value: u16) -> Self {
        Self { 
            inner: AsmFlags { bits: value as i16 } 
        }
    }
}

impl Binary for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.inner)
    }
}

impl Instruction {
    
}

pub struct Assembler {
    pub labels: HashMap<String, u16>,
    pub var_counter: u16,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: HashMap::new(),
            var_counter: 15,    // Starts at 15 so we can increment it pre insertion
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

    // fn insert_label(&mut self, label: &str, addr: u16) -> Option<u16> {
    //     self.labels.insert(String::from(label), addr)
    // }

    // fn insert_var(&mut self, label: &str) -> Option<u16> {
    //     let result = self.labels.insert(String::from(label), self.var_counter);
    //     self.var_counter += 1;
    //     result
    // }

    fn parse_a_instruction(&mut self, input: &str) -> Result<Instruction> {
        match input.parse::<u16>() {
            Ok(n) if n < i16::MAX as u16 => Ok(Instruction::from(n)),
            // If the address given is not a valid positive signed 16-bit integer, interpret it as a variable
            // Could also return an error
            _ => {
                if let Some(&addr) = self.labels.get(&input[1..]) {
                    Ok(Instruction::from(addr))
                } else {
                    self.var_counter += 1;
                    self.labels.insert(String::from(&input[1..]), self.var_counter);
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

        // All valid commands with a destination field include '='
        // Technically this current implementation allows including valid destinations in a non-standard order.
        if let Some(i) = input.find("=") {
            // we know the start of the computation field comes immediately after the '='
            comp_start = i + 1;
            let dest = &input[..i];

            // Making sure that only valid destinations are used.
            if !dest.chars().all(|c| "AMD".contains(c)) {
                return Err(anyhow!("'{dest}' contains an invalid destination character."));
            }

            // Allowing the destinations in any order might be too permissive, but it's fine for now
            inst.set(AsmFlags::DEST_D, dest.contains('D'));
            inst.set(AsmFlags::DEST_A, dest.contains('A'));
            inst.set(AsmFlags::DEST_M, dest.contains('M'));
        }
        // JUMP
        if let Some(i) = input.find(";") {
            comp_end = i;
            let jump = &input[i+1..];
            match jump {
                "JGT" => inst |= AsmFlags::JGT,
                "JEQ" => inst |= AsmFlags::JEQ,
                "JGE" => inst |= AsmFlags::JGE,
                "JLT" => inst |= AsmFlags::JLT,
                "JNE" => inst |= AsmFlags::JNE,
                "JLE" => inst |= AsmFlags::JLE,
                "JMP" => inst |= AsmFlags::JMP,
                _ => panic!("Semicolon requires a valid jump command!")
            }
        }

        // COMP
        // RIP glorious messy control flow
        // You were too clever, too permissive, and too unreadable.
        inst |= match &input[comp_start..comp_end] {
            "0" => Ctrl::Zero,
            "1" => Ctrl::One,
            "-1" => Ctrl::NegOne,
            "D" => Ctrl::D,
            "A" => Ctrl::A,
            "M" => Ctrl::M,
            "!D" => Ctrl::NotD,
            "!A" => Ctrl::NotA,
            "!M" => Ctrl::NotM,
            "-D" => Ctrl::NegD,
            "-A" => Ctrl::NegA,
            "-M" => Ctrl::NegM,
            "D+1" => Ctrl::DPlus1,            
            "A+1" => Ctrl::APlus1,            
            "M+1" => Ctrl::MPlus1,
            "D-1" => Ctrl::DMinus1,
            "A-1" => Ctrl::AMinus1,
            "M-1" => Ctrl::MMinus1,
            "D+A" | "A+D" => Ctrl::DPlusA,
            "D+M" | "M+D" => Ctrl::DPlusM,
            "D-A" => Ctrl::DMinusA,
            "D-M" => Ctrl::DMinusM,
            "A-D" => Ctrl::AMinusD,
            "M-D" => Ctrl::MMinusD,
            "D&A" | "A&D" => Ctrl::DAndA,
            "D&M" | "M&D" => Ctrl::DAndM,
            "D|A" | "A|D" => Ctrl::DOrA,
            "D|M" | "M|D" => Ctrl::DOrM,
            _ => return Err(anyhow!("invalid or unsupported computation field"))
        };

        // if comp == "0" {                    // 0
        //     inst |= Ctrl::Zero;
        // } else if comp == "1" { 
        //     inst |= Ctrl::One;
        // } else if comp == "-1" {            // -1
        //     inst |= Ctrl::NegOne;
        // } else if comp.len() <= 2 {
        //     if comp.contains('D') {         // D
        //         c_bits = 0b001100;
        //     } else {                        // A / M
        //         c_bits = 0b110000;
        //     }
        //     if comp.contains("!") {         // !D / !A / !M
        //         c_bits |= 0b000001;
        //     } else if comp.contains("-") {  // -D / -A / -M
        //         c_bits |= 0b000011;
        //     }
        // } else if comp.contains("+1") {
        //     c_bits = 0b000111;
        //     if comp.contains("D") {         // D+1
        //         c_bits |= 0b011000;
        //     } else {                        // A+1 / M+1
        //         c_bits |= 0b110000;
        //     }
        // } else if comp.contains("D+") || comp.contains("+D") {     // D+A / D+M
        //     c_bits = 0b000010;
        // } else if comp == "D-1" {           // D-1
        //     c_bits = 0b001110;
        // } else if comp.contains("-1") {     // A-1 / M-1
        //     c_bits = 0b110010;
        // } else if comp.contains("D-") {     // D-A / D-M
        //     c_bits = 0b010011;
        // } else if comp.contains("-") {      // A-D / M-D
        //     c_bits = 0b000111;
        // } else if comp.contains("&") {      // D&A / D&M
        //     c_bits = 0b000000;
        // } else if comp.contains("|") {      // D|A / D|M
        //     c_bits = 0b010101;
        // }
        // inst |= c_bits << 6;
        Ok(Instruction { inner: inst })
    }

    pub fn translate(&mut self, input: &str) -> Result<Instruction> {
        let mut inst = AsmFlags::default();
        // A or C instruction
        if input.starts_with('@') {
            self.parse_a_instruction(&input[1..])
        } else {
            self.parse_c_instruction(input)
        }
    }

    pub fn assemble(&mut self, asm: &[String]) -> Result<Vec<Instruction>> {
        // first pass
        let mut line: u16 = 0;
        for com in asm {
            if let (Some('('), Some(')')) = (com.chars().nth(0), com.chars().nth_back(0)) {
                //println!("{com}");
                self.labels.insert(String::from(&com[1..com.len() - 1]), line);
            } else {
                line += 1;
            }
        }
        Ok(asm
            .iter()
            .filter(|&c| !c.contains("("))
            .map(|c| self.translate(&c).expect("valid asm only"))
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
        for line in reader.lines() {
            if let Ok(s) = line {
                let cmd = strip_line(&s);
                if !cmd.is_empty() {
                    asm.push(cmd);
                }
            }
        }
    }
    let bin = assembler.assemble(&asm).unwrap();
    for b in bin {
        println!("{}", format!("{b:016b}"));
    }
}

fn strip_line(input: &str) -> String {
    input
        .find("//")
        .map(|i| &input[..i])
        .unwrap_or(input)
        .replace(" ", "")
}


 