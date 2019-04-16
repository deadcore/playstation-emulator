use std::fmt::*;

/// Exception types (as stored in the 'CAUSE' register)
pub enum Exception {
    /// Interrupt Request
    Interrupt = 0x0,
    /// Address error on load
    LoadAddressError = 0x4,
    /// Address error on store
    StoreAddressError = 0x5,
    /// System call (caused by the SYSCALL opcode)
    SysCall = 0x8,
    /// Breakpoint (caused by the BREAK opcode)
    Break = 0x9,
    /// CPU encountered an unknown instruction
    IllegalInstruction = 0xa,
    /// Unsupported coprocessor operation
    CoprocessorError = 0xb,
    /// Arithmetic overflow
    Overflow = 0xc,
}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name = match self {
            Exception::Interrupt => "Interrupt",
            Exception::LoadAddressError => "LoadAddressError",
            Exception::StoreAddressError => "StoreAddressError",
            Exception::SysCall => "SysCall",
            Exception::Break => "Break",
            Exception::IllegalInstruction => "IllegalInstruction",
            Exception::CoprocessorError => "CoprocessorError",
            Exception::Overflow => "Overflow",
        };
        write!(f, "{}", name)
    }
}
