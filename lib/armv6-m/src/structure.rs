/* === Memory === */
/// Memory is addressed with 32 bits addresses
pub const MEMORY_MAX_ADDRESSABLE_ADDRESS: u32 = u32::MAX; // aka 4 294 967 295

/// Max ROM memory
pub const MAX_ROM: u32 = 256 * 1024;

/// ROM physical address in memory
pub const ROM_PHYSICAL_ADDRESS: u32 = 0x00000000;

/// Max RAM memory
pub const MAX_RAM: u32 = 16 * 1024;

/// RAM physical address in memory
pub const RAM_PHYSICAL_ADDRESS: u32 = 0x20000000;

/* === Registers === */

pub const REGISTER_COUNT: u32 = 22;

/// Application Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Apsr(u32);

/// Interrupt Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Ipsr(u32);

/// Execution Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Epsr(u32);

/// List of the registers:
/// - General purpose registers R0-R12.
/// - Two Stack Pointer registers, `SP_main` and `SP_process`. These are banked versions of SP, also described as R13.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-ARM-core-registers>
/// - The Link Register, `LR` also described as R14.
/// - The Program Counter, `PC`, sometimes described as R15.
/// - Status registers for flags, execution state bits, and the current exception number.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
/// - A mask register, `PRIMASK`, used to manage the prioritization scheme for exceptions and interrupts.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-mask-register--PRIMASK>
/// - A control register, `CONTROL` that identifies the current stack.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-CONTROL-register>
///
/// Source: <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers>
#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    /// Stack Pointer register, banked version of SP (with SpProcess, described as R13), also described as MSP
    SP_main,
    /// Stack Pointer register, banked version of SP (with SpMain, described as R13), also described as PSP
    SP_process,
    /// The Link Register, also described as R14
    LR,
    /// The Program Counter, also described as R15
    PC,
    /// A mask register, used to manage the prioritization scheme for exceptions and interrupts
    PRIMASK,
    /// A control register, that identifies the current stack
    CONTROL,
    /// Application Program Status Register
    APSR,
    /// Interrupt Program Status Register
    IPSR,
    /// Execution Program Status Register
    EPSR,
}

impl Register {
    pub fn index(&self) -> usize {
        *self as usize
    }
}


/* === Condition flags === */

/// Source: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Conditional-execution>
pub enum ConditionFlag {
    /// Equal
    EQ = 0b0000,
    /// Not equal
    NE = 0b0001,
    /// Carry set
    CS = 0b0010,
    /// Carry clear
    CC = 0b0011,
    /// Minus, negative
    MI = 0b0100,
    /// Plus, positive or zero
    PL = 0b0101,
    /// Overflow
    VS = 0b0110,
    /// No overflow
    VC = 0b0111,
    /// Unsigned higher
    HI = 0b1000,
    /// Unsigned lower or same
    LS = 0b1001,
    /// Signed greater than or equal
    GE = 0b1010,
    /// Signed less than
    LT = 0b1011,
    /// Signed greater than
    GT = 0b1100,
    /// Signed less than or equal
    LE = 0b1101,
    /// Always (unconditional), also called None
    AL = 0b1110,
}

/* === Instruction set === */
pub enum Instruction {
    Bits32(Instruction32),
    Bits16(Instruction16),
}

/// Source: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions>
#[allow(nonstandard_style)]
pub enum Instruction16 {
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADC--register->
    ADC,
    ADD,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADR>
    ADR,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/AND--register->
    AND,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ASR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ASR--register->
    ASR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/B>
    B,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BIC--register->
    BIC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BKPT>
    BKPT,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BLX--register->
    BLX,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BX>
    BX,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMN--register->
    CMN,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMP--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMP--register->
    CMP,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CPS>
    CPS,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CPY>
    CPY,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/EOR--register->
    EOR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDM--LDMIA--LDMFD>
    LDM,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDM--LDMIA--LDMFD>
    LDMIA,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDM--LDMIA--LDMFD>
    LDMFD,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDR--immediate->
    /// Literal: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDR--literal->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDR--register->
    LDR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRB--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRB--register->
    LDRB,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRH--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRH--register->
    LDRH,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRSB--register->
    LDRSB,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRSH--register->
    LDRSH,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSL--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSL--register->
    LSL,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSR--register->
    LSR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MOV--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MOV--register->
    /// Shifted Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MOV--shifted-register->
    MOV,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MUL>
    MUL,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MVN--register->
    MVN,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/NEG>
    NEG,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/NOP>
    NOP,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ORR--register->
    ORR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/POP>
    POP,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/PUSH>
    PUSH,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/REV>
    REV,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/REV16>
    REV16,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/REVSH>
    REVSH,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ROR--register->
    ROR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/RSB--immediate->
    RSB,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SBC--register->
    SBC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SEV>
    SEV,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STM--STMIA--STMEA>
    STM,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STM--STMIA--STMEA>
    STMIA,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STM--STMIA--STMEA>
    STMEA,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STR--register->
    STR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRB--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRB--register->
    STRB,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRH--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRH--register->
    STRH,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SUB--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SUB--register->
    /// SP minux immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SUB--SP-minus-immediate->
    SUB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SVC>
    SVC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SXTB>
    SXTB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SXTH>
    SXTH,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/TST--register->
    TST,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/UDF>
    UDF,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/UXTB>
    UXTB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/UXTH>
    UXTH,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/WFE>
    WFE,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/WFI>
    WFI,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/YIELD>
    YIELD,
}

#[allow(nonstandard_style)]
pub enum Instruction32 {
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BL>
    BL,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/DMB>
    DMB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/DSB>
    DSB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ISB>
    ISB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MRS>
    MRS,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MSR--register->
    MSR,
}
