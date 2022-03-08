use crate::machines::IO;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum Register {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  Hl,
  Bc,
  De
}

impl Register {
  fn to_string(&self) -> &str {
    match self {
      Register::A => "A",
      Register::B => "B",
      Register::C => "C",
      Register::D => "D",
      Register::E => "E",
      Register::H => "H",
      Register::L => "L",
      Register::Hl => "HL",
      Register::Bc => "BC",
      Register::De => "DE"
    }
  }
}

#[derive(Debug)]
enum Op {
  Nop,
  Incr(Register),
  Decr(Register),
  Add(Register),
  Sub(Register),
  Ana(Register),
  Xra(Register),
  Ora(Register),
  Mov(Register, Register),
  Cmp(Register),
  Adc(Register),
  Sbb(Register),
  Lxi(Register, Register, u8, u8),
  LxiSp(u8, u8),
  Dad(Register),
  Mvi(Register, u8),
  Stax(Register),
  Inx(Register),
  Dcx(Register),
  Ldax(Register),
  Push(Register, Register),
  Pop(Register, Register),
  Rlc(),
  Rrc(),
  Ral(),
  Rar(),
  Shld(usize),
  Lhld(usize),
  Cma(),
  Sta(usize),
  InxSp(),
  Stc(),
  DadSp(),
  Lda(usize),
  DcxSp(),
  Cmc(),
  Rnz(),
  Jnz(usize),
  Jmp(usize),
  Cnz(usize),
  Adi(u8),
  Rz(),
  Ret(),
  Jz(usize),
  Cz(usize),
  Call(usize),
  Aci(u8),
  Rnc(),
  Jnc(usize),
  Cnc(usize),
  Sui(u8),
  Rc(),
  Jc(usize),
  Cc(usize),
  Sbi(u8),
  Rpo(),
  Jpo(usize),
  Xthl(),
  Cpo(usize),
  Ani(u8),
  Rpe(),
  Pchl(),
  Jpe(usize),
  Xchg(),
  Cpe(usize),
  Xri(u8),
  Rp(),
  Out(u8),
  PopPsw(),
  Jp(usize),
  Cp(usize),
  PushPsw(),
  Ori(u8),
  Rm(),
  Sphl(),
  Jm(usize),
  Ei(),
  Cm(usize),
  Cpi(u8),
  In(u8),
  Di(),
  Daa()
}

impl Op {
  fn get_size(&self) -> usize {
    match self {
      Op::Incr(_) 
      | Op::Nop 
      | Op::Decr(_) 
      | Op::Mov(_,_) 
      | Op::Add(_)
      | Op::Sub(_)
      | Op::Ana(_)
      | Op::Xra(_)
      | Op::Ora(_)
      | Op::Cmp(_)
      | Op::Adc(_)
      | Op::Sbb(_)
      | Op::Dad(_)
      | Op::Stax(_)
      | Op::Inx(_)
      | Op::Dcx(_)
      | Op::Ldax(_)
      | Op::Push(_, _)
      | Op::Pop(_, _)
      | Op::Rlc()
      | Op::Rrc()
      | Op::Ral()
      | Op::Rar()
      | Op::Cma()
      | Op::InxSp()
      | Op::Stc()
      | Op::DadSp()
      | Op::DcxSp()
      | Op::Cmc()
      | Op::Rnz()
      | Op::Rz()
      | Op::Ret()
      | Op::Rnc()
      | Op::Rc()
      | Op::Rpo()
      | Op::Xthl()
      | Op::Rpe()
      | Op::Pchl()
      | Op::Xchg()
      | Op::Rp()
      | Op::PopPsw()
      | Op::PushPsw()
      | Op::Rm()
      | Op::Sphl()
      | Op::Ei()
      | Op::Di()
      | Op::Daa() => 1,

      Op::Mvi(_, _)
      | Op::Ani(_)
      | Op::Xri(_) 
      | Op::Ori(_)
      | Op::Cpi(_)
      | Op::Aci(_)
      | Op::Sui(_)
      | Op::Sbi(_)
      | Op::In(_)
      | Op::Adi(_)
      | Op::Out(_) => 2,
      
      Op::Lxi(_, _, _, _) 
      | Op::LxiSp(_, _)
      | Op::Shld(_)
      | Op::Lhld(_)
      | Op::Sta(_)
      | Op::Lda(_)
      | Op::Jnz(_)
      | Op::Jmp(_)
      | Op::Cnz(_)
      | Op::Jz(_)
      | Op::Cz(_)
      | Op::Call(_)
      | Op::Jnc(_)
      | Op::Cnc(_)
      | Op::Jc(_)
      | Op::Cc(_)
      | Op::Jpo(_)
      | Op::Cpo(_)
      | Op::Jpe(_)
      | Op::Cpe(_)
      | Op::Jp(_)
      | Op::Cp(_)
      | Op::Jm(_)
      | Op::Cm(_)  => 3,
    }
  }
  fn print(&self) -> String {
    match self {
      Op::Incr(reg) => format!("INCR {}", reg.to_string()),
      Op::Decr(reg) => format!("DECR {}", reg.to_string()),
      Op::Add(reg) => format!("ADD {}", reg.to_string()),
      Op::Sub(reg) => format!("SUB {}", reg.to_string()),
      Op::Ana(reg) => format!("ANA {}", reg.to_string()),
      Op::Xra(reg) => format!("XRA {}", reg.to_string()),
      Op::Ora(reg) => format!("ORA {}", reg.to_string()),
      Op::Mov(dest, source) => format!("MOV {},{}", dest.to_string(), source.to_string()),
      Op::Cmp(reg) => format!("CMP {}", reg.to_string()),
      Op::Adc(reg) => format!("ADC {}", reg.to_string()),
      Op::Sbb(reg) => format!("SBB {}", reg.to_string()),
      Op::Lxi(reg1, reg2, _, _) => format!("LXI {}{}", reg1.to_string(), reg2.to_string()),
      Op::LxiSp(val1, val2) => format!("LXI SP {:02x}{:02x}", val2, val1),
      Op::Dad(reg1) => format!("DAD {}", reg1.to_string()),
      Op::Mvi(reg, val) => format!("MVI {},{}", reg.to_string(), val),
      Op::Stax(reg) => format!("STAX {}", reg.to_string()),
      Op::Inx(reg) => format!("INX {}", reg.to_string()),
      Op::Dcx(reg) => format!("DCX {}", reg.to_string()),
      Op::Ldax(reg) => format!("LDAX {}", reg.to_string()),
      Op::Push(reg, _) => format!("PUSH {}", reg.to_string()),
      Op::Pop(reg, _) => format!("POP {}", reg.to_string()),
      Op::Rlc() => format!("RLC"),
      Op::Rrc() => format!("RRC"),
      Op::Ral() => format!("RAL"),
      Op::Rar() => format!("RAR"),
      Op::Shld(_) => format!("SHLD"),
      Op::Lhld(_) => format!("LHLD"),
      Op::Cma() => format!("CMA"),
      Op::Sta(_) => format!("STA"),
      Op::InxSp() => format!("INX SP"),
      Op::Stc() => format!("STC"),
      Op::DadSp() => format!("DAD SP"),
      Op::Lda(val) => format!("LDA {:04x}", val),
      Op::DcxSp() => format!("DCX SP"),
      Op::Cmc() => format!("CMC"),
      Op::Rnz() => format!("RNZ"),
      Op::Jnz(val) => format!("JNZ {:04x}", val),
      Op::Jmp(val) => format!("JMP {:04x}", val),
      Op::Cnz(_) => format!("CNZ"),
      Op::Adi(_) => format!("ADI"),
      Op::Rz() => format!("RZ"),
      Op::Ret() => format!("RET"),
      Op::Jz(_) => format!("JZ"),
      Op::Cz(_) => format!("CZ"),
      Op::Call(val) => format!("CALL {:04x}", val),
      Op::Aci(_) => format!("ACI"),
      Op::Rnc() => format!("RNC"),
      Op::Jnc(_) => format!("JNC"),
      Op::Cnc(_) => format!("CNC"),
      Op::Sui(_) => format!("SUI"),
      Op::Rc() => format!("RC"),
      Op::Jc(_) => format!("JC"),
      Op::Cc(_) => format!("CC"),
      Op::Sbi(_) => format!("SBI"),
      Op::Rpo() => format!("RPO"),
      Op::Jpo(_) => format!("JPO"),
      Op::Xthl() => format!("XTHL"),
      Op::Cpo(_) => format!("CPO"),
      Op::Ani(_) => format!("ANI"),
      Op::Rpe() => format!("RPE"),
      Op::Pchl() => format!("PCHL"),
      Op::Jpe(_) => format!("JPE"),
      Op::Xchg() => format!("XCHG"),
      Op::Cpe(_) => format!("CPE"),
      Op::Xri(_) => format!("XRI"),
      Op::Rp() => format!("RP"),
      Op::Out(_) => format!("OUT"),
      Op::PopPsw() => format!("POP PSW"),
      Op::Jp(_) => format!("JP"),
      Op::Cp(_) => format!("CP"),
      Op::PushPsw() => format!("PUSH PSW"),
      Op::Ori(_) => format!("ORI"),
      Op::Rm() => format!("Rm"),
      Op::Sphl() => format!("Sphl"),
      Op::Jm(_) => format!("Jm"),
      Op::Ei() => format!("EI"),
      Op::Cm(_) => format!("CM"),
      Op::Cpi(_) => format!("CPI"),
      Op::In(_) => format!("IN"),
      Op::Di() => format!("DI"),
      Op::Daa() => format!("DAA"),
      Op::Nop => format!("NOP")
    }
  }
}

struct Flags {
  z: u8,
  s: u8,
  p: u8,
  cy: u8,
  ac: u8
}

pub struct State {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  pub sp: usize,
  pub pc: usize,
  pub memory: [u8; 64000],
  flags: Flags
}

impl State {
  fn set_register_16(&mut self, reg: &Register, value: u16) {
    let [val1, val2] = value.to_le_bytes();
    match reg {
      Register::Hl => {
        self.h = val2;
        self.l = val1;
      }
      Register::Bc => {
        self.b = val2;
        self.c = val1;
      }
      Register::De => {
        self.d = val2;
        self.e = val1;
      }
      _ => {
        panic!("Unsupported for register type {}", reg.to_string())
      }
    }
  }
  fn get_register_16(&self, reg: &Register) -> u16 {
    match reg {
      Register::Bc => {
        u16::from_le_bytes([self.c, self.b])
      }
      Register::De => {
        u16::from_le_bytes([self.e, self.d])
      }
      Register::Hl => {
        u16::from_le_bytes([self.l, self.h])
      }
      _ => {
        panic!("Unsupported for register type {}", reg.to_string())
      }
    }
  }
  fn set_register(&mut self, reg: &Register, value: u8) {
    match reg {
      Register::A => self.a = value,
      Register::B => self.b = value,
      Register::C => self.c = value,
      Register::D => self.d = value,
      Register::E => self.e = value,
      Register::H => self.h = value,
      Register::L => self.l = value,
      Register::Hl => {
        self.memory[u16::from_le_bytes([self.l, self.h]) as usize] = value
      },
      Register::Bc => {
        self.memory[u16::from_le_bytes([self.c, self.b]) as usize] = value
      }
      Register::De => {
        self.memory[u16::from_le_bytes([self.e, self.d]) as usize] = value
      }
    }
  }
  fn get_register(&self, reg: &Register) -> u8 {
    match reg {
      Register::A => self.a,
      Register::B => self.b,
      Register::C => self.c,
      Register::D => self.d,
      Register::E => self.e,
      Register::H => self.h,
      Register::L => self.l,
      Register::Hl => {
        self.memory[u16::from_le_bytes([self.l, self.h]) as usize]
      },
      Register::Bc => {
        self.memory[u16::from_le_bytes([self.c, self.b]) as usize]
      },
      Register::De => {
        self.memory[u16::from_le_bytes([self.e, self.d]) as usize]
      }
    }
  }
}

pub struct Cpu {
  pub state: State,
  program_size: usize,
  interrupts_enabled: bool,
  op_history: Vec<String>
}

impl Cpu {
  pub fn new(bytes: Vec<u8>) -> Self {
    let mut state = State {
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
      sp: 0,
      pc: 0,
      memory: [0; 64000],
      flags: Flags {
        z: 0,
        s: 0,
        p: 0,
        cy: 0,
        ac: 0
      }
    };
    let program_size = bytes.len();
    for (i, b) in bytes.into_iter().enumerate() {
      state.memory[i] = b;
    }
    Cpu {
      state,
      program_size,
      interrupts_enabled: false,
      op_history: Vec::new()
    }
  }

  fn read_next_op(&self) -> Result<Op, String> {
    let byte = self.state.memory[self.state.pc];
    let byte2 = if self.state.pc + 1 < 64000 {
      self.state.memory[self.state.pc + 1]
    } else {
      0
    };
    let byte3 = if self.state.pc + 2 < 64000 {
      self.state.memory[self.state.pc + 2]
    } else {
      0
    };
    let bytes_as_usize = u16::from_le_bytes([byte2, byte3]) as usize;
    match byte {
      0x00 => Ok(Op::Nop),
      // LXI Ops
      0x01 => Ok(Op::Lxi(Register::B, Register::C, byte2, byte3)),
      0x11 => Ok(Op::Lxi(Register::D, Register::E, byte2, byte3)),
      0x21 => Ok(Op::Lxi(Register::H, Register::L, byte2, byte3)),
      0x31 => Ok(Op::LxiSp(byte2, byte3)),
      // Increment Ops
      0x04 => Ok(Op::Incr(Register::B)),
      0x3c => Ok(Op::Incr(Register::A)),
      0x0c => Ok(Op::Incr(Register::C)),
      0x14 => Ok(Op::Incr(Register::D)),
      0x1c => Ok(Op::Incr(Register::E)),
      0x24 => Ok(Op::Incr(Register::H)),
      0x2c => Ok(Op::Incr(Register::L)),
      0x34 => Ok(Op::Incr(Register::Hl)),
      // Decrement Ops
      0x15 => Ok(Op::Decr(Register::D)),
      0x1d => Ok(Op::Decr(Register::E)),
      0x25 => Ok(Op::Decr(Register::H)),
      0x2d => Ok(Op::Decr(Register::L)),
      0x3d => Ok(Op::Decr(Register::A)),
      0x0d => Ok(Op::Decr(Register::C)),
      0x05 => Ok(Op::Decr(Register::B)),
      0x35 => Ok(Op::Decr(Register::Hl)),
      // Add Ops
      0x80 => Ok(Op::Add(Register::B)),
      0x81 => Ok(Op::Add(Register::C)),
      0x82 => Ok(Op::Add(Register::D)),
      0x83 => Ok(Op::Add(Register::E)),
      0x84 => Ok(Op::Add(Register::H)),
      0x85 => Ok(Op::Add(Register::L)),
      0x86 => Ok(Op::Add(Register::Hl)),
      0x87 => Ok(Op::Add(Register::A)),
      // Sub Ops
      0x90 => Ok(Op::Sub(Register::B)),
      0x91 => Ok(Op::Sub(Register::C)),
      0x92 => Ok(Op::Sub(Register::D)),
      0x93 => Ok(Op::Sub(Register::E)),
      0x94 => Ok(Op::Sub(Register::H)),
      0x95 => Ok(Op::Sub(Register::L)),
      0x96 => Ok(Op::Sub(Register::Hl)),
      0x97 => Ok(Op::Sub(Register::A)),
      // Bitwise & Ops
      0xa0 => Ok(Op::Ana(Register::B)),
      0xa1 => Ok(Op::Ana(Register::C)),
      0xa2 => Ok(Op::Ana(Register::D)),
      0xa3 => Ok(Op::Ana(Register::E)),
      0xa4 => Ok(Op::Ana(Register::H)),
      0xa5 => Ok(Op::Ana(Register::L)),
      0xa6 => Ok(Op::Ana(Register::Hl)),
      0xa7 => Ok(Op::Ana(Register::A)),
      // Bitwise XOR Ops
      0xa8 => Ok(Op::Xra(Register::B)),
      0xa9 => Ok(Op::Xra(Register::C)),
      0xaa => Ok(Op::Xra(Register::D)),
      0xab => Ok(Op::Xra(Register::E)),
      0xac => Ok(Op::Xra(Register::H)),
      0xad => Ok(Op::Xra(Register::L)),
      0xae => Ok(Op::Xra(Register::Hl)),
      0xaf => Ok(Op::Xra(Register::A)),
      // Bitwise OR Ops
      0xb0 => Ok(Op::Ora(Register::B)),
      0xb1 => Ok(Op::Ora(Register::C)),
      0xb2 => Ok(Op::Ora(Register::D)),
      0xb3 => Ok(Op::Ora(Register::E)),
      0xb4 => Ok(Op::Ora(Register::H)),
      0xb5 => Ok(Op::Ora(Register::L)),
      0xb6 => Ok(Op::Ora(Register::Hl)),
      0xb7 => Ok(Op::Ora(Register::A)),
      // Cmp Ops
      0xb8 => Ok(Op::Cmp(Register::B)),
      0xb9 => Ok(Op::Cmp(Register::C)),
      0xba => Ok(Op::Cmp(Register::D)),
      0xbb => Ok(Op::Cmp(Register::E)),
      0xbc => Ok(Op::Cmp(Register::H)),
      0xbd => Ok(Op::Cmp(Register::L)),
      0xbe => Ok(Op::Cmp(Register::Hl)),
      0xbf => Ok(Op::Cmp(Register::A)),
      // Adc Ops
      0x88 => Ok(Op::Adc(Register::B)),
      0x89 => Ok(Op::Adc(Register::C)),
      0x8a => Ok(Op::Adc(Register::D)),
      0x8b => Ok(Op::Adc(Register::E)),
      0x8c => Ok(Op::Adc(Register::H)),
      0x8d => Ok(Op::Adc(Register::L)),
      0x8e => Ok(Op::Adc(Register::Hl)),
      0x8f => Ok(Op::Adc(Register::A)),
      // Sbb Ops
      0x98 => Ok(Op::Sbb(Register::B)),
      0x99 => Ok(Op::Sbb(Register::C)),
      0x9a => Ok(Op::Sbb(Register::D)),
      0x9b => Ok(Op::Sbb(Register::E)),
      0x9c => Ok(Op::Sbb(Register::H)),
      0x9d => Ok(Op::Sbb(Register::L)),
      0x9e => Ok(Op::Sbb(Register::Hl)),
      0x9f => Ok(Op::Sbb(Register::A)),
      // Mov Ops
      0x40 => Ok(Op::Mov(Register::B, Register::B)),
      0x41 => Ok(Op::Mov(Register::B, Register::C)),
      0x42 => Ok(Op::Mov(Register::B, Register::D)),
      0x43 => Ok(Op::Mov(Register::B, Register::E)),
      0x44 => Ok(Op::Mov(Register::B, Register::H)),
      0x45 => Ok(Op::Mov(Register::B, Register::L)),
      0x47 => Ok(Op::Mov(Register::B, Register::A)),
      0x48 => Ok(Op::Mov(Register::C, Register::B)),
      0x49 => Ok(Op::Mov(Register::C, Register::C)),
      0x4a => Ok(Op::Mov(Register::C, Register::D)),
      0x4b => Ok(Op::Mov(Register::C, Register::E)),
      0x4c => Ok(Op::Mov(Register::C, Register::H)),
      0x4d => Ok(Op::Mov(Register::C, Register::L)),
      0x4f => Ok(Op::Mov(Register::C, Register::A)),
      0x50 => Ok(Op::Mov(Register::D, Register::B)),
      0x51 => Ok(Op::Mov(Register::D, Register::C)),
      0x52 => Ok(Op::Mov(Register::D, Register::D)),
      0x53 => Ok(Op::Mov(Register::D, Register::E)),
      0x54 => Ok(Op::Mov(Register::D, Register::H)),
      0x55 => Ok(Op::Mov(Register::D, Register::L)),
      0x57 => Ok(Op::Mov(Register::D, Register::A)),
      0x58 => Ok(Op::Mov(Register::E, Register::B)),
      0x59 => Ok(Op::Mov(Register::E, Register::C)),
      0x5a => Ok(Op::Mov(Register::E, Register::D)),
      0x5b => Ok(Op::Mov(Register::E, Register::E)),
      0x5c => Ok(Op::Mov(Register::E, Register::H)),
      0x5d => Ok(Op::Mov(Register::E, Register::L)),
      0x5f => Ok(Op::Mov(Register::E, Register::A)),
      0x60 => Ok(Op::Mov(Register::H, Register::B)),
      0x61 => Ok(Op::Mov(Register::H, Register::C)),
      0x62 => Ok(Op::Mov(Register::H, Register::D)),
      0x63 => Ok(Op::Mov(Register::H, Register::E)),
      0x64 => Ok(Op::Mov(Register::H, Register::H)),
      0x65 => Ok(Op::Mov(Register::H, Register::L)),
      0x67 => Ok(Op::Mov(Register::H, Register::A)),
      0x68 => Ok(Op::Mov(Register::L, Register::B)),
      0x69 => Ok(Op::Mov(Register::L, Register::C)),
      0x6a => Ok(Op::Mov(Register::L, Register::D)),
      0x6b => Ok(Op::Mov(Register::L, Register::E)),
      0x6c => Ok(Op::Mov(Register::L, Register::H)),
      0x6f => Ok(Op::Mov(Register::L, Register::A)),
      0x78 => Ok(Op::Mov(Register::A, Register::B)),
      0x79 => Ok(Op::Mov(Register::A, Register::C)),
      0x7a => Ok(Op::Mov(Register::A, Register::D)),
      0x7b => Ok(Op::Mov(Register::A, Register::E)),
      0x7c => Ok(Op::Mov(Register::A, Register::H)),
      0x7d => Ok(Op::Mov(Register::A, Register::L)),
      0x46 => Ok(Op::Mov(Register::B, Register::Hl)),
      0x4e => Ok(Op::Mov(Register::C, Register::Hl)),
      0x56 => Ok(Op::Mov(Register::D, Register::Hl)),
      0x5e => Ok(Op::Mov(Register::E, Register::Hl)),
      0x66 => Ok(Op::Mov(Register::H, Register::Hl)),
      0x6e => Ok(Op::Mov(Register::L, Register::Hl)),
      0x70 => Ok(Op::Mov(Register::Hl, Register::B)),
      0x71 => Ok(Op::Mov(Register::Hl, Register::C)),
      0x72 => Ok(Op::Mov(Register::Hl, Register::D)),
      0x73 => Ok(Op::Mov(Register::Hl, Register::E)),
      0x74 => Ok(Op::Mov(Register::Hl, Register::H)),
      0x75 => Ok(Op::Mov(Register::Hl, Register::L)),
      0x77 => Ok(Op::Mov(Register::Hl, Register::A)),
      0x7e => Ok(Op::Mov(Register::A, Register::Hl)),
      // DAD Ops
      0x09 => Ok(Op::Dad(Register::Bc)),
      0x19 => Ok(Op::Dad(Register::De)),
      0x29 => Ok(Op::Dad(Register::Hl)),
      // MVI Ops
      0x06 => Ok(Op::Mvi(Register::B, byte2)),
      0x0e => Ok(Op::Mvi(Register::C, byte2)),
      0x16 => Ok(Op::Mvi(Register::D, byte2)),
      0x1e => Ok(Op::Mvi(Register::E, byte2)),
      0x26 => Ok(Op::Mvi(Register::H, byte2)),
      0x2e => Ok(Op::Mvi(Register::L, byte2)),
      0x36 => Ok(Op::Mvi(Register::Hl, byte2)),
      0x3e => Ok(Op::Mvi(Register::A, byte2)),
      // STAX ops
      0x02 => Ok(Op::Stax(Register::Bc)),
      0x12 => Ok(Op::Stax(Register::De)),
      // INX Ops
      0x03 => Ok(Op::Inx(Register::Bc)),
      0x13 => Ok(Op::Inx(Register::De)),
      0x23 => Ok(Op::Inx(Register::Hl)),
      // DCX Ops
      0x0b => Ok(Op::Dcx(Register::Bc)),
      0x1b => Ok(Op::Dcx(Register::De)),
      0x2b => Ok(Op::Dcx(Register::Hl)),
      // LDAX Ops
      0x0a => Ok(Op::Ldax(Register::Bc)),
      0x1a => Ok(Op::Ldax(Register::De)),
      // PUSH Ops
      0xc5 => Ok(Op::Push(Register::B, Register::C)),
      0xd5 => Ok(Op::Push(Register::D, Register::E)),
      0xe5 => Ok(Op::Push(Register::H, Register::L)),
      // POP Ops
      0xc1 => Ok(Op::Pop(Register::B, Register::C)),
      0xd1 => Ok(Op::Pop(Register::D, Register::E)),
      0xe1 => Ok(Op::Pop(Register::H, Register::L)),
      // RLC
      0x07 => Ok(Op::Rlc()),
      // RRC
      0x0f => Ok(Op::Rrc()),
      // RAL
      0x17 => Ok(Op::Ral()),
      // RAR
      0x1f => Ok(Op::Rar()),
      // SHLD/LHLD
      0x22 => Ok(Op::Shld(bytes_as_usize)),
      0x2a => Ok(Op::Lhld(bytes_as_usize)),
      // CMA
      0x2f => Ok(Op::Cma()),
      // STA
      0x32 => Ok(Op::Sta(bytes_as_usize)),
      // INX SP
      0x33 => Ok(Op::InxSp()),
      // STC
      0x37 => Ok(Op::Stc()),
      // DAD SP
      0x39 => Ok(Op::DadSp()),
      // LDA
      0x3a => Ok(Op::Lda(bytes_as_usize)),
      // DCX SP
      0x3b => Ok(Op::DcxSp()),
      // CMC
      0x3f => Ok(Op::Cmc()),
      // RNZ
      0xc0 => Ok(Op::Rnz()),
      // JNZ
      0xc2 => Ok(Op::Jnz(bytes_as_usize)),
      // JMP
      0xc3 => Ok(Op::Jmp(bytes_as_usize)),
      // CNZ
      0xc4 => Ok(Op::Cnz(bytes_as_usize)),
      // ADI
      0xc6 => Ok(Op::Adi(byte2)),
      // RZ
      0xc8 => Ok(Op::Rz()),
      // RET
      0xc9 => Ok(Op::Ret()),
      // JZ
      0xca => Ok(Op::Jz(bytes_as_usize)),
      // CZ
      0xcc => Ok(Op::Cz(bytes_as_usize)),
      // CALL
      0xcd => Ok(Op::Call(bytes_as_usize)),
      0xce => Ok(Op::Aci(byte2)),
      0xd0 => Ok(Op::Rnc()),
      0xd2 => Ok(Op::Jnc(bytes_as_usize)),
      // CNC
      0xd4 => Ok(Op::Cnc(bytes_as_usize)),
      // SUI
      0xd6 => Ok(Op::Sui(byte2)),
      // RC
      0xd8 => Ok(Op::Rc()),
      // JC
      0xda => Ok(Op::Jc(bytes_as_usize)),
      // CC
      0xdc => Ok(Op::Cc(bytes_as_usize)),
      // SBI
      0xde => Ok(Op::Sbi(byte2)),
      // RPO
      0xe0 => Ok(Op::Rpo()),
      // JPO
      0xe2 => Ok(Op::Jpo(bytes_as_usize)),
      // XTHML
      0xe3 => Ok(Op::Xthl()),
      // CPO
      0xe4 => Ok(Op::Cpo(bytes_as_usize)),
      // ANI
      0xe6 => Ok(Op::Ani(byte2)),
      // RPE
      0xe8 => Ok(Op::Rpe()),
      // PCHL
      0xe9 => Ok(Op::Pchl()),
      // JPE
      0xea => Ok(Op::Jpe(bytes_as_usize)),
      // XCHG
      0xeb => Ok(Op::Xchg()),
      // CPE
      0xec => Ok(Op::Cpe(bytes_as_usize)),
      // XRI
      0xee => Ok(Op::Xri(byte2)),
      // RP
      0xf0 => Ok(Op::Rp()),
      // OUT
      0xd3 => Ok(Op::Out(byte2)),
      // POP PSW
      0xf1 => Ok(Op::PopPsw()),
      // JP 
      0xf2 => Ok(Op::Jp(bytes_as_usize)),
      // CP
      0xf4 => Ok(Op::Cp(bytes_as_usize)),
      // PUSH PSW
      0xf5 => Ok(Op::PushPsw()),
      // ORI
      0xf6 => Ok(Op::Ori(byte2)),
      // RM
      0xf8 => Ok(Op::Rm()),
      // SPHL
      0xf9 => Ok(Op::Sphl()),
      // JM
      0xfa => Ok(Op::Jm(bytes_as_usize)),
      // EI
      0xfb => Ok(Op::Ei()),
      // CM
      0xfc => Ok(Op::Cm(bytes_as_usize)),
      // CPI
      0xfe => Ok(Op::Cpi(byte2)),
      // IN
      0xdb => Ok(Op::In(byte2)),
      // DI
      0xf3 => Ok(Op::Di()),
      // DAA
      0x27 => Ok(Op::Daa()),
      _ => Err(format!("{:04x}", byte))
    }
  }

  fn execute_op(&mut self, op_code: Op, io: &RefCell<dyn IO>) -> u8 {
    self.op_history.push(op_code.print());
    self.state.pc += &op_code.get_size();
    match &op_code {
      Op::Nop => {
        4
      }
      Op::Incr(reg) => {
        let val = self.state.get_register(&reg);
        let (answer, overflowed) = val.overflowing_add(1);
        self.set_flags(answer);
        self.state.set_register(&reg, answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        5
      }
      Op::Decr(reg) => {
        let val = self.state.get_register(&reg);
        let (answer, overflowed) = val.overflowing_sub(1);
        self.set_flags(answer);
        self.state.set_register(&reg, answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        match reg {
          Register::Hl => 10,
          _ => 5
        }
      }
      Op::Add(reg) => {
        let (answer, overflowed) = self.state.a.overflowing_add(self.state.get_register(reg));
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        self.state.a = answer;
        4
      }
      Op::Sub(reg) => {
        let (answer, overflowed) = self.state.a.overflowing_sub(self.state.get_register(reg));
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        self.state.a = answer;
        4
      }
      Op::Ana(reg) => {
        let answer = self.state.a & self.state.get_register(reg);
        self.set_flags(answer);
        self.state.flags.cy = if self.state.a < answer {
          1
        } else {
          0
        };
        self.state.a = answer;
        4
      }
      Op::Xra(reg) => {
        let answer = self.state.a ^ self.state.get_register(reg);
        self.set_flags(answer);
        self.state.flags.cy = if self.state.a < answer {
          1
        } else {
          0
        };
        self.state.a = answer;
        4
      }
      Op::Ora(reg) => {
        let answer = self.state.a | self.state.get_register(reg);
        self.set_flags(answer);
        self.state.flags.cy = if self.state.a < answer {
          1
        } else {
          0
        };
        self.state.a = answer;
        match reg {
          Register::Hl => 7,
          _ => 4
        }
      }
      Op::Cmp(reg) => {
        let (answer, overflowed) = self.state.a.overflowing_sub(self.state.get_register(reg));
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        4
      }
      Op::Adc(reg) => {
        let (answer, overflowed) = self.state.a.overflowing_add(self.state.get_register(reg));
        let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
        self.set_flags(answer2);
        self.state.flags.cy = if overflowed || overflowed2 {
          1
        } else {
          0
        };
        self.state.a = answer2;
        4
      }
      Op::Sbb(reg) => {
        let (answer, overflowed) = self.state.a.overflowing_sub(self.state.get_register(reg));
        let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
        self.set_flags(answer2);
        self.state.flags.cy = if overflowed || overflowed2 {
          1
        } else {
          0
        };
        self.state.a = answer2;
        4
      }
      Op::Lxi(reg1, reg2, val1, val2) => {
        self.state.set_register(reg1, *val2);
        self.state.set_register(reg2, *val1);
        10
      }
      Op::LxiSp(val1, val2) => {
        self.state.sp = u16::from_le_bytes([*val1, *val2]) as usize;
        10
      }
      Op::Mov(dest, source) => {        
        match dest {
          Register::Hl => {
            self.state.set_register(dest, self.state.get_register(source));
//            self.state.set_register(source, self.state.get_register(dest));
            7
          }
          _ => {
            self.state.set_register(dest, self.state.get_register(source));
            match source {
              Register::Hl => 7,
              _ => 5
            }
          }
        }
      }
      Op::Dad(reg1) => {
        let val = self.state.get_register_16(reg1) as u32;
        let hl = self.state.get_register_16(&Register::Hl) as u32;
        let answer = hl + val;
        self.state.flags.cy = if answer > u16::MAX as u32 {
          1
        } else {
          0
        };
        let [l, h, _, _] = answer.to_le_bytes();
        self.state.l = l;
        self.state.h = h;
        10
      }
      Op::Mvi(reg, val) => {
        self.state.set_register(reg, *val);
        match reg {
          Register::Hl => 10,
          _ => 7
        }          
      }
      Op::Stax(reg) => {
        self.state.set_register(reg, self.state.a);
        7
      }
      Op::Inx(reg) => {
        self.state.set_register_16(reg, self.state.get_register_16(reg).overflowing_add(1).0);
        5
      }
      Op::Dcx(reg) => {
        self.state.set_register_16(reg, self.state.get_register_16(reg).overflowing_sub(1).0);
        5
      },
      Op::Ldax(reg) => {
        self.state.a = self.state.get_register(reg);
        7
      }
      Op::Push(reg1, reg2) => {
        self.state.memory[self.state.sp - 1] = self.state.get_register(reg1);
        self.state.memory[self.state.sp - 2] = self.state.get_register(reg2);
        self.state.sp -= 2;
        11
      }
      Op::Pop(reg1, reg2) => {
        self.state.set_register(reg1, self.state.memory[self.state.sp + 1]);
        self.state.set_register(reg2, self.state.memory[self.state.sp]);
        self.state.sp += 2;
        10
      }
      Op::Rlc() => {
        let leftmost = self.state.a >> 7;
        self.state.flags.cy = leftmost;
        self.state.a = (self.state.a << 1) | leftmost;
        4
      }
      Op::Rrc() => {
        let rightmost = self.state.a & 1;
        self.state.flags.cy = if rightmost == 1 {
          1
        } else {
          0
        };
        self.state.a =  (rightmost << 7) | (self.state.a >> 1);
        4
      }
      Op::Ral() => {
        let leftmost = self.state.a >> 7;
        self.state.a = (self.state.a << 1) | self.state.flags.cy;
        self.state.flags.cy = leftmost;
        4
      }
      Op::Rar() => {
        let rightmost = self.state.a & 1;
        self.state.a = (self.state.a >> 1) | (self.state.flags.cy << 7);
        self.state.flags.cy = rightmost;
        4
      }
      Op::Shld(address) => {
        self.state.memory[*address] = self.state.l;
        self.state.memory[address + 1] = self.state.h;
        16
      }
      Op::Lhld(address) => {
        self.state.l = self.state.memory[*address];
        self.state.h = self.state.memory[address + 1];
        16
      }
      Op::Cma() => {
        self.state.a = !self.state.a;
        4
      }
      Op::Sta(address) => {
        self.state.memory[*address] = self.state.a;
        13
      }
      Op::InxSp() => {
        if self.state.sp == 0xffff {
          self.state.sp = 0;
        } else {
          self.state.sp += 1;
        }
        5
      }
      Op::Stc() => {
        self.state.flags.cy = 1;
        4
      }
      Op::DadSp() => {
        let hl = self.state.get_register_16(&Register::Hl);
        let sp = u16::from_le_bytes([(self.state.sp - 1) as u8, self.state.sp as u8]); 
        let [l, h] = (hl + sp).to_le_bytes();
        self.state.h = h;
        self.state.l = l;
        10
      }
      Op::Lda(address) => {
        self.state.a = self.state.memory[*address];
        13
      }
      Op::DcxSp() => {
        if self.state.sp == 0 {
          self.state.sp = 0xffff;
        } else {
          self.state.sp -= 1;
        }
        5
      }
      Op::Cmc() => {
        self.state.flags.cy = if self.state.flags.cy == 1 {
          0
        } else {
          1
        };
        4
      }
      Op::Rnz() => {
        if self.state.flags.z == 0 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2;
          11 
        } else {
          5
        }
      }
      Op::Jnz(val) => {
        if self.state.flags.z == 0 {
          self.state.pc = *val;
        }
        10
      }
      Op::Jmp(val) => {
        self.state.pc = *val;
        10
      }
      Op::Cnz(val) => {
        if self.state.flags.z == 0 {
          let return_address = ((self.state.pc) as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Adi(val) => {
        let (answer, overflowed) = self.state.a.overflowing_add(*val);
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };

        self.state.a = answer as u8;
        7
      }
      Op::Rz() => {
        if self.state.flags.z == 1 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
            self.state.sp += 2;
          11
        } else {
          5
        }
      }
      Op::Ret() => {
        self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
        self.state.sp += 2;
        10
      }
      Op::Jz(val) => {
        if self.state.flags.z == 1 {
          self.state.pc = *val;
        }
        10
      }
      Op::Cz(val) => {
        if self.state.flags.z == 1 {
          let return_address = ((self.state.pc) as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Call(val) => {
        let address = *val;
        if address == 5 && self.state.c == 9 {
            let mut offset = (self.state.get_register_16(&Register::De) + 3) as usize;
            while self.state.memory[offset] != 36 {
              print!("{}", self.state.memory[offset] as char);
              offset += 1;
            }
            println!();
            std::process::exit(0);
        }
        let return_address = ((self.state.pc) as u16).to_le_bytes();
        self.state.memory[self.state.sp - 1] = return_address[1];
        self.state.memory[self.state.sp - 2] = return_address[0];
        self.state.sp -= 2;
        self.state.pc = address;
        17
      }
      Op::Aci(val) => {
        let (answer, overflowed) = self.state.a.overflowing_add(*val + self.state.flags.cy);
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        self.state.a = answer;
        7
      },
      Op::Rnc() => {
        if self.state.flags.cy == 0 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2; 
          11
        } else {
          5
        }
      }
      Op::Jnc(val) => {
        if self.state.flags.cy == 0 {
          self.state.pc = *val;
        }
        10
      }
      Op::Cnc(val) => {
        if self.state.flags.cy == 0 {
          let return_address = ((self.state.pc) as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Sui(val) => {
        let (answer, overflowed) = self.state.a.overflowing_sub(*val);
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        self.state.a = answer;
        7
      }
      Op::Rc() => {
        if self.state.flags.cy == 1 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2;
          11
        } else {
          5
        }
      }
      Op::Jc(val) => {
        if self.state.flags.cy == 1 {
          self.state.pc = *val;
        }
        10
      }
      Op::Cc(val) => {
        if self.state.flags.cy == 1 {
          let return_address = (self.state.pc as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Sbi(val) => {
        let (mut answer, overflowed) = self.state.a.overflowing_sub(*val);
        answer -= self.state.flags.cy;
        self.set_flags(answer);
        self.state.flags.cy = if overflowed {
          1
        } else {
          0
        };
        self.state.a = answer;
        7
      }
      Op::Rpo() => {
        if self.state.flags.p == 0 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2; 
          11
        } else {
          5
        }
      }
      Op::Jpo(val) => {
        if self.state.flags.p == 0 {
          self.state.pc = *val;
        }
        10
      }
      Op::Xthl() => {
        std::mem::swap(&mut self.state.l, &mut self.state.memory[self.state.sp]);
        std::mem::swap(&mut self.state.h, &mut self.state.memory[self.state.sp + 1]);
        18
      }
      Op::Cpo(val) => {
        if self.state.flags.p == 0 {
          let return_address = (self.state.pc as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Ani(val) => {
        let answer = self.state.a & *val;
        self.set_flags(answer);  
        self.state.flags.cy = if self.state.a < (answer as u8) {
          1
        } else {
          0
        };
        self.state.a = answer as u8;
        7
      }
      Op::Rpe() => {
        if self.state.flags.p == 1 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2; 
          11
        } else {
          5
        }
      }
      Op::Pchl() => {
        self.state.pc = self.state.get_register_16(&Register::Hl) as usize;
        5
      }
      Op::Jpe(val) => {
        if self.state.flags.p == 1 {
          self.state.pc = *val;
        }
        10
      }
      Op::Xchg() => {
        let tmp = self.state.h;
        self.state.h = self.state.d;
        self.state.d = tmp;
        let tmp = self.state.l;
        self.state.l = self.state.e;
        self.state.e = tmp;
        5
      }
      Op::Cpe(val) => {
        if self.state.flags.p == 1 {
          let return_address = (self.state.pc as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Xri(val) => {
        let answer = self.state.a ^ *val;
        self.set_flags(answer);  
        self.state.flags.cy = if self.state.a < (answer as u8) {
          1
        } else {
          0
        };
        self.state.a = answer as u8;
        7
      }
      Op::Rp() => {
        if self.state.flags.s == 0 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2; 
          11
        } else {
          5
        }
      }
      Op::Out(port) => {
        io.borrow_mut().output(*port, self.state.get_register(&Register::A));
        10
      }
      Op::PopPsw() => {
        let flags = self.state.memory[self.state.sp];
        self.state.a = self.state.memory[self.state.sp + 1];
        self.state.flags.cy = flags & 1;
        self.state.flags.p = if (flags & (1 << 2)) > 0 {
          1
        } else {
          0
        };

        self.state.flags.ac = if (flags & (1 << 4)) > 0 {
          1
        } else {
          0
        };

        self.state.flags.z = if (flags & (1 << 6)) > 0 {
          1
        } else {
          0
        };

        self.state.flags.s = if (flags & (1 << 7)) > 0 {
          1
        } else {
          0
        };
        self.state.sp += 2;
        10
      }
      Op::Jp(val) => {
        if self.state.flags.s == 0 {
          self.state.pc = *val;
        }
        10
      }
      Op::Cp(val) => {
        if self.state.flags.s == 0 {
          let return_address = (self.state.pc  as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::PushPsw() => {
        let flags = (self.state.a as u16) << 8 |
        (self.state.flags.s << 7) as u16 |
        (self.state.flags.z << 6) as u16 |
        (self.state.flags.p << 2) as u16 |
        1 << 1 as u16 |
        self.state.flags.cy as u16;
        self.state.memory[self.state.sp - 2] = flags as u8;
        self.state.memory[self.state.sp - 1] = self.state.a;
        self.state.sp -= 2;
        11
      }
      Op::Ori(val) => {
        let answer = self.state.a | *val;
        self.set_flags(answer);  
        self.state.flags.cy = if self.state.a < (answer as u8) {
          1
        } else {
          0
        };
        self.state.a = answer as u8;
        7
      }
      Op::Rm() => {
        if self.state.flags.s == 1 {
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp], self.state.memory[self.state.sp + 1]]) as usize;
          self.state.sp += 2; 
          11
        } else {
          5
        }
      }
      Op::Sphl() => {
        self.state.sp = self.state.get_register_16(&Register::Hl) as usize;
        5
      }
      Op::Jm(val) => {
        if self.state.flags.s == 1 {
          self.state.pc = *val;
        }
        10
      }
      Op::Ei() => {
        self.interrupts_enabled = true;
        4
      }
      Op::Cm(val) => {
        if self.state.flags.s == 1 {
          let return_address = (self.state.pc as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[1];
          self.state.memory[self.state.sp - 2] = return_address[0];
          self.state.sp -= 2;
          self.state.pc = *val;
          17
        } else {
          11
        }
      }
      Op::Cpi(val) => {
        let (answer, _) = self.state.a.overflowing_sub(*val);
        self.set_flags(answer);
        self.state.flags.cy = if self.state.a < *val {
          1
        } else {
          0
        };
        7
      }
      Op::In(port) => {
        self.state.set_register(&Register::A, io.borrow().input(*port));
        10
      }
      Op::Di() => {
        self.interrupts_enabled = false;
        4
      },
      Op::Daa() => {
        let mut a: u16 = self.state.get_register(&Register::A) as u16;
        // least significant bits
        let lsb = a & 0x0f;
        if lsb > 9 || self.state.flags.ac == 1 {
            a += 6;
            self.state.flags.ac = if (lsb + 6) > 0xf {
              1
            } else {
              0
            }
        };
        // most significant bits
        let mut msb = (a & 0xf0) >> 4;
        if (msb > 9) || self.state.flags.cy == 1 { 
          msb += 6; 
        }
        a = (msb << 4) | (a & 0xf);
        self.state.flags.ac = if (msb + 6) > 0xf {
          1
        } else {
          0
        };
        self.set_flags(a as u8);
        self.state.a = a as u8;

        4
      }
    }
  }

  pub fn run(&mut self, io: &RefCell<dyn IO>) {
    let mut n = 0;
    while self.state.pc < self.program_size {
      n += 1;
      let op_code = self.read_next_op();
      match op_code {
        Ok(op) => {
          self.execute_op(op, io);
        },
        Err(op) => {
          panic!("unhandled op {}", op);
        }
      };
    }
  }

  pub fn execute_next_op(&mut self, io: &RefCell<dyn IO>) -> Result<u8, &str> {
      let op_code = self.read_next_op();
      match op_code {
        Ok(op) => {
          let cycles = self.execute_op(op, io);
          let result = Ok(cycles);
          result
        },
        Err(op) => {
          Err("Error executing op {}")
        }
      }
  }

  pub fn interrupt(&mut self, interrupt_num: u8) -> bool {
    if self.interrupts_enabled {   
      let return_address = ((self.state.pc) as u16).to_le_bytes();
      self.state.memory[self.state.sp - 1] = return_address[1];
      self.state.memory[self.state.sp - 2] = return_address[0];
      self.state.sp -= 2;
      self.state.pc = (interrupt_num as usize) << 3;
      true
    } else {
      false
    }
  }

  fn set_flags(&mut self, val: u8) {
    self.state.flags.z = if val == 0 {
      1
    } else {
      0
    };

    self.state.flags.s = if val > 128 {
      1
    } else {
      0
    };

    self.state.flags.p = parity(val as u8);
  }
}

pub fn parity(b: u8) -> u8 {
  if b.count_zeros() % 2 == 0 {
    1
  } else {
    0
  }
}

fn print_debug_info(state: &State, op: &Op, counter: u64) -> String {
  //print!("\x1B[2J\x1B[1;1H");
  let mut log_str = format!("{:04x}: {:11} a:{:02x} b:{:02x} c:{:02x} d:{:02x} e:{:02x} lh:{:02x}{:02x} pc:{:04x} sp:{:04x} spvalue:{:04x} | {} {}",
            state.pc, op.print(), state.a, state.b, state.c, state.d, state.e, state.h, state.l, state.pc, state.sp, (state.memory[state.sp] as u16) << 8 | state.memory[state.sp + 1] as u16, "", counter);
  
  if state.flags.z == 1 {
    log_str += "z";
  } else {
    log_str += ".";
  }

  if state.flags.s == 1 {
    log_str += "s";
  } else {
    log_str += ".";
  }

  if state.flags.p == 1 {
    log_str += "p";
  } else {
    log_str += ".";
  }


  if state.flags.cy == 1 {
    log_str += "c";
  } else {
    log_str += ".";
  }

  log_str
}

#[cfg(test)]
mod test {
  use std::fs;
  use crate::cpu::Cpu;
  use crate::machines::spaceinvaders::SpaceInvadersIO;
  use crate::machines::Speaker;
  use std::cell::RefCell;
  struct TestSpeaker {}
  impl Speaker for TestSpeaker {
    fn play_wav_file(&mut self, _: &str) {
      // no op
    }
    fn start_wav_file(&mut self, _: &str) {
      // no op
    }
    fn stop_wav_file(&mut self, _: &str) {
     // no op
    }
  }
  #[test]
  fn cpudiag() {
    let result = fs::read("resources/cpudiag.bin");
    if let Ok(mut bytes) = result {
        for _ in 0..256 {
          bytes.insert(0, 0);
        }
        bytes[0] = 0xc3;
        bytes[1] = 0;
        bytes[2] = 0x01;
      
        bytes[368] = 0x7;

        bytes[0x59c] = 0xc3;
        bytes[0x59d] = 0xc2;
        bytes[0x59e] = 0x05;
        let space_invaders_io = &RefCell::new(SpaceInvadersIO::new(Box::new(TestSpeaker{})));
        let mut cpu = Cpu::new(bytes);
        cpu.run(space_invaders_io);
    } else {
        println!("Error reading file {:?}", result);
    }
  }
}
