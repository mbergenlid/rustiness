
pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;

pub const BRANCH_PLUS: u8           = 0x10;
pub const BRANCH_MINUS: u8          = 0x30;
pub const BRANCH_OVERFLOW_SET: u8   = 0x70;
pub const BRANCH_OVERFLOW_CLEAR: u8 = 0x50;
pub const BRANCH_CARRY_SET: u8      = 0xB0;
pub const BRANCH_CARRY_CLEAR: u8    = 0x90;
pub const BRANCH_NOT_EQUAL: u8      = 0xD0;
pub const BRANCH_EQUAL: u8          = 0xF0;