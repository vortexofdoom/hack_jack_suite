pub mod translator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VmCommand<'a> {
    // Arithmetic
    Add,
    Sub,
    Neg,
    Compare(Comparison),
    And,
    Or,
    Not,
    //mem access
    Push(MemSegment, i16),
    Pop(MemSegment, i16),
    // Branching
    Label(&'a str),
    Goto(&'a str),
    IfGoto(&'a str),
    // Function
    Function(&'a str, i16),
    Call(&'a str, i16),
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparison {
    Eq,
    GT,
    LT,
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::GT => write!(f, "gt"),
            Self::LT => write!(f, "lt"),
        }
    }
}

impl std::fmt::Display for MemSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Argument => write!(f, "argument"),
            Self::This => write!(f, "this"),
            Self::That => write!(f, "that"),
            Self::Constant => write!(f, "constant"),
            Self::Static => write!(f, "static"),
            Self::Pointer => write!(f, "pointer"),
            Self::Temp => write!(f, "temp"),
        }
    }
}

impl std::fmt::Display for VmCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmCommand::Add => write!(f, "add"),
            VmCommand::Sub => write!(f, "sub"),
            VmCommand::Neg => write!(f, "neg"),
            VmCommand::Compare(cmp) => write!(f, "{cmp}"),
            VmCommand::And => write!(f, "and"),
            VmCommand::Or => write!(f, "or"),
            VmCommand::Not => write!(f, "not"),
            VmCommand::Push(seg, arg) => write!(f, "push {seg} {arg}"),
            VmCommand::Pop(seg, arg) => write!(f, "pop {seg} {arg}"),
            VmCommand::Label(label) => write!(f, "label {label}"),
            VmCommand::Goto(label) => write!(f, "goto {label}"),
            VmCommand::IfGoto(label) => write!(f, "if-goto {label}"),
            VmCommand::Function(func, n) => write!(f, "function {func} {n}"),
            VmCommand::Call(func, n) => write!(f, "call {func} {n}"),
            VmCommand::Return => write!(f, "return"),
        }
    }
}
