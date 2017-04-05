extern crate nes;
extern crate rand;

use std::time::{Instant, Duration};
use nes::opcodes::*;
use nes::cpu::CPU;


pub fn run() {
    let mut bench = InstructionBenchmark::new();
    for _ in 0..10000 {
        bench.run_random_instruction();
    }
    bench.timing_results.sort_by_key(|t| t.max);
    //timing_vector.sort_by_key(|&(_, d)| d);
    for t in bench.timing_results {
        println!("{:?}", t);
    }
}


#[derive(Copy, Clone, Debug)]
struct InstructionTiming {
    op_code: OpCode,
    duration: Duration,
    count: u32,
    min: Duration,
    max: Duration,
    mean: Duration,
}

impl InstructionTiming {
    fn new(op_code: OpCode) -> InstructionTiming {
        InstructionTiming {
            op_code: op_code,
            duration: Duration::new(0,0),
            count: 0,
            min: Duration::new(10,0),
            max: Duration::new(0,0),
            mean: Duration::new(0,0),
        }
    }
}

use std::ops;
use std::cmp::{min, max};
impl ops::AddAssign<Duration> for InstructionTiming {
    fn add_assign(&mut self, duration: Duration) {
        self.duration += duration;
        self.count += 1;
        self.min = min(self.min, duration);
        self.max = max(self.max, duration);
        self.mean = self.duration / self.count;
    }
}

struct InstructionBenchmark {
    op_codes: OpCodes,
    cpu: CPU,
    timing_results: Vec<InstructionTiming>,
}

impl InstructionBenchmark {
    fn new() -> InstructionBenchmark {
        InstructionBenchmark {
            op_codes: OpCodes::new(),
            cpu: CPU::new(0x2000),
            timing_results: OP_CODES.into_iter().map(|&code| InstructionTiming::new(code)).collect(),
        }
    }

    fn run_random_instruction(&mut self) {
        let op_code_index = (rand::random::<u8>() % 151) as usize;
        let op_code = OP_CODES[op_code_index];
        let start_address = self.cpu.program_counter();

        let mut memory = external_memory!(
            start_address => op_code,
            start_address.wrapping_add(1) => rand::random::<u8>(),
            start_address.wrapping_add(2) => rand::random::<u8>(),
            start_address.wrapping_add(3) => rand::random::<u8>(),
            start_address.wrapping_add(4) => rand::random::<u8>(),
            start_address.wrapping_add(5) => rand::random::<u8>()
        );

        let start = Instant::now();
        self.op_codes.execute_instruction(&mut self.cpu, &mut memory);
        let elapsed = start.elapsed();
        self.timing_results[op_code_index] += elapsed;
    }
}



const OP_CODES: [OpCode; 151] = [
    ADC_IMMEDIATE,
    ADC_ZERO_PAGE,
    ADC_ZERO_PAGE_X,
    ADC_ABSOLUTE,
    ADC_ABSOLUTE_X,
    ADC_ABSOLUTE_Y,
    ADC_INDIRECT_X,
    ADC_INDIRECT_Y,
    AND_IMMEDIATE,
    AND_ZERO_PAGE,
    AND_ZERO_PAGE_X,
    AND_ABSOLUTE,
    AND_ABSOLUTE_X,
    AND_ABSOLUTE_Y,
    AND_INDIRECT_X,
    AND_INDIRECT_Y,
    ASL_ACCUMULATOR,
    ASL_ZERO_PAGE,
    ASL_ZERO_PAGE_X,
    ASL_ABSOLUTE,
    ASL_ABSOLUTE_X,
    BIT_ZERO_PAGE,
    BIT_ABSOLUTE,
    BRANCH_PLUS,
    BRANCH_MINUS,
    BRANCH_OVERFLOW_SET,
    BRANCH_OVERFLOW_CLEAR,
    BRANCH_CARRY_SET,
    BRANCH_CARRY_CLEAR,
    BRANCH_NOT_EQUAL,
    BRANCH_EQUAL,
    BRK         ,
    CMP_IMMEDIATE,
    CMP_ZERO_PAGE,
    CMP_ZERO_PAGE_X,
    CMP_ABSOLUTE,
    CMP_ABSOLUTE_X,
    CMP_ABSOLUTE_Y,
    CMP_INDIRECT_X,
    CMP_INDIRECT_Y,
    CPX_IMMEDIATE,
    CPX_ZERO_PAGE,
    CPX_ABSOLUTE,
    CPY_IMMEDIATE,
    CPY_ZERO_PAGE,
    CPY_ABSOLUTE,
    DEC_ZERO_PAGE,
    DEC_ZERO_PAGE_X,
    DEC_ABSOLUTE,
    DEC_ABSOLUTE_X,
    EOR_IMMEDIATE,
    EOR_ZERO_PAGE,
    EOR_ZERO_PAGE_X,
    EOR_ABSOLUTE,
    EOR_ABSOLUTE_X,
    EOR_ABSOLUTE_Y,
    EOR_INDIRECT_X,
    EOR_INDIRECT_Y,
    CLC            ,
    SEC            ,
    CLI            ,
    SEI            ,
    CLV            ,
    CLD            ,
    SED            ,
    INC_ZERO_PAGE,
    INC_ZERO_PAGE_X,
    INC_ABSOLUTE,
    INC_ABSOLUTE_X,
    JMP_ABSOLUTE,
    JMP_INDIRECT,
    JSR_ABSOLUTE,
    LDA_IMMEDIATE,
    LDA_ZERO_PAGE,
    LDA_ZERO_PAGE_X,
    LDA_ABSOLUTE,
    LDA_ABSOLUTE_X,
    LDA_ABSOLUTE_Y,
    LDA_INDIRECT_X,
    LDA_INDIRECT_Y,
    LDX_IMMEDIATE,
    LDX_ZERO_PAGE,
    LDX_ZERO_PAGE_Y,
    LDX_ABSOLUTE,
    LDX_ABSOLUTE_Y,
    LDY_IMMEDIATE,
    LDY_ZERO_PAGE,
    LDY_ZERO_PAGE_X,
    LDY_ABSOLUTE,
    LDY_ABSOLUTE_X,
    LSR_ACCUMULATOR,
    LSR_ZERO_PAGE,
    LSR_ZERO_PAGE_X,
    LSR_ABSOLUTE,
    LSR_ABSOLUTE_X,
    NOP_IMPLIED,
    ORA_IMMEDIATE,
    ORA_ZERO_PAGE,
    ORA_ZERO_PAGE_X,
    ORA_ABSOLUTE,
    ORA_ABSOLUTE_X,
    ORA_ABSOLUTE_Y,
    ORA_INDIRECT_X,
    ORA_INDIRECT_Y,
    TAX            ,
    TXA            ,
    DEX            ,
    INX            ,
    TAY            ,
    TYA            ,
    DEY            ,
    INY            ,
    ROL_ACCUMULATOR,
    ROL_ZERO_PAGE,
    ROL_ZERO_PAGE_X,
    ROL_ABSOLUTE,
    ROL_ABSOLUTE_X,
    ROR_ACCUMULATOR,
    ROR_ZERO_PAGE,
    ROR_ZERO_PAGE_X,
    ROR_ABSOLUTE,
    ROR_ABSOLUTE_X,
    RTI        ,
    RTS        ,
    SBC_IMMEDIATE,
    SBC_ZERO_PAGE,
    SBC_ZERO_PAGE_X,
    SBC_ABSOLUTE,
    SBC_ABSOLUTE_X,
    SBC_ABSOLUTE_Y,
    SBC_INDIRECT_X,
    SBC_INDIRECT_Y,
    STA_ZERO_PAGE,
    STA_ZERO_PAGE_X,
    STA_ABSOLUTE,
    STA_ABSOLUTE_X,
    STA_ABSOLUTE_Y,
    STA_INDIRECT_X,
    STA_INDIRECT_Y,
    TXS            ,
    TSX            ,
    PHA            ,
    PLA            ,
    PHP            ,
    PLP            ,
    STX_ZERO_PAGE,
    STX_ZERO_PAGE_Y,
    STX_ABSOLUTE,
    STY_ZERO_PAGE,
    STY_ZERO_PAGE_X,
    STY_ABSOLUTE,
];
