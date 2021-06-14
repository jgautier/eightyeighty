use std::thread;

enum Register {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  Hl
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
      Register::Hl => "HL"
    }
  }
}

enum Op {
  Nop,
  Incr(Register),
  Decr(Register),
  Add(Register),
  Sub(Register),
  Ana(Register),
  Xra(Register),
  Ora(Register),
  Mov(Register, Register)
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
      | Op::Ora(_) => 1,
    }
  }
  fn print(&self) {
    match self {
      Op::Incr(reg) => println!("INCR {}", reg.to_string()),
      Op::Decr(reg) => println!("DECR {}", reg.to_string()),
      Op::Add(reg) => println!("ADD {}", reg.to_string()),
      Op::Sub(reg) => println!("SUB {}", reg.to_string()),
      Op::Ana(reg) => println!("ANA {}", reg.to_string()),
      Op::Xra(reg) => println!("XRA {}", reg.to_string()),
      Op::Ora(reg) => println!("ORA {}", reg.to_string()),
      Op::Mov(dest, source) => println!("MOV {},{}", dest.to_string(), source.to_string()),
      Op::Nop => println!("NOP")
    }
  }
}

struct Flags {
  z: u8,
  s: u8,
  p: u8,
  cy: u8,
  ac: u8,
  pad: [u8; 3]
}

struct State {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  sp: usize,
  pc: usize,
  memory: [u8; 64000],
  flags: Flags
}

impl State {
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
      }
    }
  }
}

pub struct Emulator {
  state: State,
  program_size: usize
}

impl Emulator {
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
        ac: 0,
        pad: [0, 0, 0]
      }
    };
    let program_size = bytes.len();
    for (i, b) in bytes.into_iter().enumerate() {
      state.memory[i] = b;
    }
    Emulator {
      state,
      program_size
    }
  }

  fn read_next_op(&self) -> Result<Op, u8> {
    let byte = self.state.memory[self.state.pc];
    match byte {
      // Increment Ops
      0x04 => Ok(Op::Incr(Register::B)),
      0x3c => Ok(Op::Incr(Register::A)),
      0x0c => Ok(Op::Incr(Register::C)),
      0x14 => Ok(Op::Incr(Register::D)),
      0x1c => Ok(Op::Incr(Register::E)),
      0x24 => Ok(Op::Incr(Register::H)),
      0x2c => Ok(Op::Incr(Register::L)),
      // Decrement Ops
      0x15 => Ok(Op::Decr(Register::D)),
      0x1d => Ok(Op::Decr(Register::E)),
      0x25 => Ok(Op::Decr(Register::H)),
      0x2d => Ok(Op::Decr(Register::L)),
      0x3d => Ok(Op::Decr(Register::A)),
      0x0d => Ok(Op::Decr(Register::C)),
      0x05 => Ok(Op::Decr(Register::B)),
      // Add Ops
      0x80 => Ok(Op::Add(Register::B)),
      0x81 => Ok(Op::Add(Register::C)),
      0x82 => Ok(Op::Add(Register::D)),
      0x83 => Ok(Op::Add(Register::E)),
      0x84 => Ok(Op::Add(Register::H)),
      0x85 => Ok(Op::Add(Register::L)),
      0x87 => Ok(Op::Add(Register::A)),
      // Sub Ops
      0x90 => Ok(Op::Sub(Register::B)),
      0x91 => Ok(Op::Sub(Register::C)),
      0x92 => Ok(Op::Sub(Register::D)),
      0x93 => Ok(Op::Sub(Register::E)),
      0x94 => Ok(Op::Sub(Register::H)),
      0x95 => Ok(Op::Sub(Register::L)),
      0x97 => Ok(Op::Sub(Register::A)),
      // Bitwise & Ops
      0xa0 => Ok(Op::Ana(Register::B)),
      0xa1 => Ok(Op::Ana(Register::C)),
      0xa2 => Ok(Op::Ana(Register::D)),
      0xa3 => Ok(Op::Ana(Register::E)),
      0xa4 => Ok(Op::Ana(Register::H)),
      0xa5 => Ok(Op::Ana(Register::L)),
      0xa7 => Ok(Op::Ana(Register::A)),
      // Bitwise XOR Ops
      0xa8 => Ok(Op::Xra(Register::B)),
      0xa9 => Ok(Op::Xra(Register::C)),
      0xaa => Ok(Op::Xra(Register::D)),
      0xab => Ok(Op::Xra(Register::E)),
      0xac => Ok(Op::Xra(Register::H)),
      0xad => Ok(Op::Xra(Register::L)),
      0xaf => Ok(Op::Xra(Register::A)),
      // Bitwise OR Ops
      0xb0 => Ok(Op::Ora(Register::B)),
      0xb1 => Ok(Op::Ora(Register::C)),
      0xb2 => Ok(Op::Ora(Register::D)),
      0xb3 => Ok(Op::Ora(Register::E)),
      0xb4 => Ok(Op::Ora(Register::H)),
      0xb5 => Ok(Op::Ora(Register::L)),
      0xb7 => Ok(Op::Ora(Register::A)),
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
      _ => Err(byte)
    }
  }

  fn execute_op(&mut self, op_code: Op) {
    op_code.print();
    match &op_code {
      Op::Nop => {}
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
      }
      Op::Mov(dest, source) => {
        self.state.set_register(dest, self.state.get_register(source))
      }
    };
    self.state.pc += &op_code.get_size();
  }

  pub fn run(&mut self) {
    let mut n = 0;
    while self.state.pc < self.program_size {
      n += 1;
      let op_code = self.read_next_op();
      match op_code {
        Ok(op) => {
          self.execute_op(op);
          print_debug_info(&self.state, n);
          continue
        },
        Err(_) => println!("Unhandled op falling back")
      };
      let op = self.get_current_op();
      match op {
        0x00 => {
          println!("NOP");
        }
        0x01 => {
          println!("LXI B,D16");
          self.state.b = self.state.memory[self.state.pc + 1];
          self.state.c = self.state.memory[self.state.pc];
          self.state.pc += 2;
        }
        0x02 => {
          println!("STAX B");
          self.state.memory[self.get_bc() as usize] = self.state.a;
        }
        0x03 => {
          println!("INX B");
          let (answer, _) = self.get_bc().overflowing_add(1);
          let [c, b] = answer.to_le_bytes();
          self.state.b = b;
          self.state.c = c;
        }
        0x06 => {
          println!("MVI B {:x}", self.state.memory[self.state.pc]);
          self.state.b = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x07 => {
          println!("RLC");
          let leftmost = self.state.a >> 7;
          self.state.flags.cy = leftmost;
          self.state.a = (self.state.a << 1) | leftmost;
        }
        0x09 => {
          println!("DAD B");
          let hl = self.get_hl() as u32;
          let bc = self.get_bc() as u32;
          let answer = hl + bc;
          self.state.flags.cy = if answer > u16::MAX as u32 {
            1
          } else {
            0
          };
          let [l, h, _, _] = answer.to_le_bytes();
          self.state.l = l;
          self.state.h = h;
        }
        0x0a => {
          println!("LDAX B");
          self.state.a = self.get_memory_at_bc();
        }
        0x0b => {
          println!("DCX B");
          let [c, b] = (self.get_bc() - 1).to_le_bytes();
          self.state.b = b;
          self.state.c = c;
        }
        0x0e => {
          println!("MVI, C,D8 {:x}", self.state.memory[self.state.pc]);
          self.state.c = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x0f => {
          println!("RRC");
          let rightmost = self.state.a & 1;
          self.state.flags.cy = if rightmost == 1 {
            1
          } else {
            0
          };
          self.state.a = (self.state.a >> 1) | (rightmost << 7);
        }
        0x11 => {
          println!("LXI D,D16 {:x}, {:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          self.state.d = self.state.memory[self.state.pc + 1];
          self.state.e = self.state.memory[self.state.pc];
          self.state.pc += 2;
        }
        0x12 => {
          println!("STAX D");
          self.state.memory[self.get_de() as usize] = self.state.a;
        }
        0x13 => {
          println!("INCX DE");
          let [e, d] = (u16::from_le_bytes([self.state.e, self.state.d]) + 1).to_le_bytes();
          self.state.d = d;
          self.state.e = e;
        }
        0x16 => {
          println!("MVI D D8");
          self.state.d = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x17 => {
          println!("RAL");
          let leftmost = self.state.a >> 7;
          self.state.a = (self.state.a << 1) | self.state.flags.cy;
          self.state.flags.cy = leftmost;
        }
        0x19 => {
          println!("DAD D");
          let hl = self.get_hl();
          let de = self.get_de();
          let answer = hl + de;
          let [l, h] = answer.to_le_bytes();
          self.state.l = l;
          self.state.h = h;
          //TODO set carry
        }
        0x1a => {
          println!("LD A {:x}, {:x}", self.state.d, self.state.e);
          self.state.a = self.get_memory_at_de();
        }
        0x1b => {
          println!("DCX D");
          let [e, d] = (self.get_de() - 1).to_le_bytes();
          self.state.d = d;
          self.state.e = e;
        }
        0x1e => {
          println!("MVI E,D8");
          self.state.e = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x1f => {
          println!("RAR");
          let rightmost = self.state.a & 1;
          self.state.a = (self.state.a >> 1) | (self.state.flags.cy << 7);
          self.state.flags.cy = rightmost;
        }
        0x21 => {
          println!("LXI H,D16 {:x}, {:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          self.state.h = self.state.memory[self.state.pc + 1];
          self.state.l = self.state.memory[self.state.pc];
          self.state.pc += 2;
        }
        0x22 => {
          println!("SHLD ADR");
          let address = self.get_next_2_bytes_as_usize();
          self.state.memory[address] = self.state.l;
          self.state.memory[address + 1] = self.state.h;
        }
        0x23 => {
          println!("INCX HL");
          let [l, h] = (self.get_hl() + 1).to_le_bytes();
          self.state.h = h;
          self.state.l = l;
        }
        0x26 => {
          println!("MVI H,D8 {:x}", self.state.memory[self.state.pc]);
          self.state.h = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x29 => {
          println!("DAD H");
          let hl = self.get_hl() as u32;
          let answer = hl * 2;
          self.state.flags.cy = if answer > u16::MAX as u32 {
            1
          } else {
            0
          };
          let [l, h, _, _] = answer.to_le_bytes();
          self.state.h = h;
          self.state.l = l;
        }
        0x2a => {
          println!("LHLD adr");
          let address = self.get_next_2_bytes_as_usize();
          self.state.l = self.state.memory[address];
          self.state.h = self.state.memory[address + 1];
        }
        0x2b => {
          println!("DCX H");
          let [l, h] = (self.get_hl() - 1).to_le_bytes();
          self.state.h = h;
          self.state.l = l;
        }
        0x2e => {
          println!("MVI L, D8");
          self.state.l = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x2f => {
          println!("CMA");
          self.state.a = !self.state.a;
        }
        0x31 => {
          println!("LXI SP, D16");
          self.state.sp = self.get_next_2_bytes_as_usize();
        }
        0x32 => {
          println!("STA  {:x} {:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          self.state.memory[self.get_next_2_bytes_as_usize()] = self.state.a;
        }
        0x33 => {
          println!("INX SP");
          self.state.sp += 1;
        }
        0x34 => {
          println!("INCR M");
          let (answer, _) = self.get_memory_at_hl().overflowing_add(1);
          self.state.memory[self.get_hl() as usize] = answer;
          self.set_flags(answer);
        }
        0x35 => {
          println!("DCR M");
          let (answer, _) = self.get_memory_at_hl().overflowing_sub(1);
          self.state.memory[self.get_hl() as usize] = answer;
          self.set_flags(answer);
        }
        0x36 => {
          println!("MVI, M,D8");
          self.state.memory[self.get_hl() as usize] = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x37 => {
          println!("STC");
          self.state.flags.cy = 1;
        }
        0x39 => {
          println!("DAD SP");
          let [l, h] = (self.get_hl() + (self.state.sp as u16)).to_le_bytes();
          self.state.h = h;
          self.state.l = l;
        }
        0x3a => {
          println!("LDA {:x}{:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          self.state.a = self.state.memory[self.get_next_2_bytes_as_usize()];
        }
        0x3b => {
          println!("DCX SP");
          self.state.sp -= 1;
        }
        0x3e => {
          println!("MVI  A,D8");
          self.state.a = self.state.memory[self.state.pc];
          self.state.pc += 1;
        }
        0x3f => {
          println!("CMC");
          self.state.flags.cy = if self.state.flags.cy == 1 {
            0
          } else {
            1
          }
        }
        0x86 => {
          println!("ADD M");
          let (answer, overflowed) = self.state.a.overflowing_add(self.get_memory_at_hl());
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x88 => {
          println!("ADC B");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.b);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x89 => {
          println!("ADC c");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.c);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x8a => {
          println!("ADC D");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.d);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x8b => {
          println!("ADC e");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.e);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x8c => {
          println!("ADC h");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.h);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x8d => {
          println!("ADC L");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.l);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x8e => {
          println!("ADC B");
          let (answer, overflowed) = self.state.a.overflowing_add(self.get_memory_at_hl());
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x8f => {
          println!("ADC A");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.a);
          let (answer2, overflowed2) = answer.overflowing_add(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x96 => {
          println!("SUB M");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.get_memory_at_hl());
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x98 => {
          println!("SBB B");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.b);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x99 => {
          println!("SBB C");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.c);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x9a => {
          println!("SBB D");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.d);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x9b => {
          println!("SBB e");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.e);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x9c => {
          println!("SBB H");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.h);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x9d => {
          println!("SBB L");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.l);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x9e => {
          println!("SBB m");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.get_memory_at_hl());
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0x9f => {
          println!("SBB A");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.a);
          let (answer2, overflowed2) = answer.overflowing_sub(self.state.flags.cy);
          self.set_flags(answer2);
          self.state.flags.cy = if overflowed || overflowed2 {
            1
          } else {
            0
          };
          self.state.a = answer2;
        }
        0xa6 => {
          println!("ANA M");
          let answer = self.state.a & self.get_memory_at_hl();
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xae => {
          println!("XRA M");
          let answer = self.state.a ^ self.get_memory_at_hl();
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb6 => {
          println!("ORA M");
          let answer = self.state.a | self.get_memory_at_hl();
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb8 => {
          println!("CMP B");
          let answer = self.state.a - self.state.b;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xb9 => {
          println!("CMP C");
          let answer = self.state.a - self.state.c;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xba => {
          println!("CMP d");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.d);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
        }
        0xbb => {
          println!("CMP e");
          let answer = self.state.a - self.state.e;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xbc => {
          println!("CMP h");
          let answer = self.state.a - self.state.h;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xbd => {
          println!("CMP l");
          let answer = self.state.a - self.state.l;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xbe => {
          println!("CMP e");
          let answer = self.state.a - self.get_memory_at_hl();
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xbf => {
          println!("CMP a");
          let answer = 0;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
        }
        0xc0 => {
          println!("{:04x}", self.state.pc);
          println!("RNZ");
          if self.state.flags.z == 0 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xc1 => {
          println!("POP B");
          self.state.c = self.state.memory[self.state.sp];
          self.state.b = self.state.memory[self.state.sp + 1];
          self.state.sp += 2;
        }
        0xc2 => {
          println!("JNZ {:x}{:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          if self.state.flags.z == 0 {
            self.state.pc = self.get_next_2_bytes_as_usize();
          } else {
            self.state.pc += 2;
          }
        }
        0xc3 => {
          println!("JMP  {:x} {:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          self.state.pc = self.get_next_2_bytes_as_usize();
        }
        0xc4 => {
          println!("CNZ addr");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.z == 0 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xc5 => {
          println!("PUSH B");
          self.state.memory[self.state.sp - 2] = self.state.c;
          self.state.memory[self.state.sp - 1] = self.state.b;
          self.state.sp -= 2;
        }
        0xc6 => {
          println!("ADI {:x}", self.state.memory[self.state.pc]);
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.memory[self.state.pc]);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
  
          self.state.a = answer as u8;
          self.state.pc += 1;
        }
        0xc8 => {
          println!("{:04x}", self.state.pc);
          println!("RZ");
          if self.state.flags.z == 1 {
            println!("{:04x}", u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize);
            //self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            let low = self.state.memory[self.state.sp.wrapping_add(1)];
            let high = self.state.memory[self.state.sp];
            self.state.pc = (((high as u16) << 8) + low as u16) as usize;
            self.state.sp += 2; 
          }
        }
        0xc9 => {
          println!("RET");
          self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
          self.state.sp += 2; 
        }
        0xca => {
          println!("JZ");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.z == 1 {
            self.state.pc = address;
          }
        }
        0xcc => {
          println!("CZ adr");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.z == 1 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xcd => {
          println!("CALL {:x} {:x}", self.state.memory[self.state.pc + 1], self.state.memory[self.state.pc]);
          let address = self.get_next_2_bytes_as_usize();
          if address == 5 && self.state.c == 9 {
              let mut offset = (self.get_de() + 3) as usize;
              while self.state.memory[offset] != 36 {
                print!("{}", self.state.memory[offset] as char);
                offset += 1;
              }
              println!();
              std::process::exit(0);
          }
          let return_address = ((self.state.pc) as u16).to_le_bytes();
          self.state.memory[self.state.sp - 1] = return_address[0];
          self.state.memory[self.state.sp - 2] = return_address[1];
          self.state.sp -= 2;
          self.state.pc = address;
        }
        0xce => {
          println!("ACI D8");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.memory[self.state.pc] + self.state.flags.cy);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
          self.state.pc += 1;
        }
        0xd0 => {
          print!("RNC");
          if self.state.flags.cy == 0 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xd1 => {
          println!("POP D");
          self.state.e = self.state.memory[self.state.sp];
          self.state.d = self.state.memory[self.state.sp + 1];
          self.state.sp += 2;
        }
        0xd2 => {
          println!("JNC");
          let address = self.get_next_2_bytes_as_usize();
          self.state.pc = address;
          if self.state.flags.cy == 0 {
            self.state.pc = address;
          }
        }
        0xd3 => {
          println!("SPECIAL");
          // TODO ???
          self.state.pc += 1;
        }
        0xd4 => {
          println!("CNC adr");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.cy == 0 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xd5 => {
          println!("PUSH D");
          self.state.memory[self.state.sp - 2] = self.state.e;
          self.state.memory[self.state.sp - 1] = self.state.d;
          self.state.sp -= 2;
        }
        0xd6 => {
          println!("SUI D8");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.memory[self.state.pc]);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
          self.state.pc += 1;
        }
        0xd8 => {
          println!("RC");
          if self.state.flags.cy == 1 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xda => {
          println!("JC");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.cy == 1 {
            self.state.pc = address;
          }
        }
        0xdc => {
          println!("CC ADDR");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.cy == 1 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xde => {
          println!("SBI D8");
          let (mut answer, overflowed) = self.state.a.overflowing_sub(self.state.memory[self.state.pc]);
          answer -= self.state.flags.cy;
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
          self.state.pc += 1;
        }
        0xe0 => {
          println!("RPO");
          if self.state.flags.p == 0 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xe1 => {
          println!("POP H");
          self.state.l = self.state.memory[self.state.sp];
          self.state.h = self.state.memory[self.state.sp + 1];
          self.state.sp += 2;
        }
        0xe2 => {
          println!("JPO adr");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.p == 0 {
            self.state.pc = address;
          }
        }
        0xe3 => {
          println!("XTHL");
          std::mem::swap(&mut self.state.l, &mut self.state.memory[self.state.sp]);
          std::mem::swap(&mut self.state.h, &mut self.state.memory[self.state.sp + 1]);
        }
        0xe4 => {
          println!("CPO ADDR");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.p == 0 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xe5 => {
          println!("PUSH H");
          self.state.memory[self.state.sp - 2] = self.state.l;
          self.state.memory[self.state.sp - 1] = self.state.h;
          self.state.sp -=2;
        }
        0xe6 => {
          println!("ANI D8");
          let answer = self.state.a & self.state.memory[self.state.pc];
          self.set_flags(answer);  
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
          self.state.pc += 1;
        }
        0xe8 => {
          println!("RPE");
          if self.state.flags.p == 1 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xe9 => {
          println!("PCHL");
          self.state.pc = self.get_hl() as usize;
        }
        0xea => {
          println!("JPE");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.p == 1 {
            self.state.pc = address;
          }
        }
        0xeb => {
          println!("XCHG");
          let tmp = self.state.h;
          self.state.h = self.state.d;
          self.state.d = tmp;
          let tmp = self.state.l;
          self.state.l = self.state.e;
          self.state.e = tmp;
        }
        0xec => {
          let address = self.get_next_2_bytes_as_usize();
          println!("CPE adr {:x}", address);
          if self.state.flags.p == 1 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xee => {
          println!("XRI D8");
          let answer = self.state.a ^ self.state.memory[self.state.pc];
          self.set_flags(answer);  
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
          self.state.pc += 1;
        }
        0xf0 => {
          println!("RP");
          if self.state.flags.s == 0 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xf1 => {
          println!("POP PSW");
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
        }
        0xf2 => {
          println!("JP");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.s == 0 {
            self.state.pc = address;
          }
        }
        0xf4 => {
          println!("CP adr");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.s == 0 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xf5 => {
          println!("PUSH PSW");
          let mut flags = 0;
          flags |= self.state.flags.cy;
          flags |= 1 << 1;
          flags |= self.state.flags.p << 2;
          flags |= self.state.flags.ac << 4;
          flags |= self.state.flags.z << 6;
          flags |= self.state.flags.s << 7;
          self.state.memory[self.state.sp - 2] = flags;
          self.state.memory[self.state.sp - 1] = self.state.a;
          self.state.sp -= 2;
        }
        0xf6 => {
          println!("ORI D8");
          let answer = self.state.a | self.state.memory[self.state.pc];
          self.set_flags(answer);  
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
          self.state.pc += 1;
        }
        0xf8 => {
          println!("RM");
          if self.state.flags.s == 1 {
            self.state.pc = u16::from_le_bytes([self.state.memory[self.state.sp + 1], self.state.memory[self.state.sp]]) as usize;
            self.state.sp += 2; 
          }
        }
        0xf9 => {
          println!("SPHL");
          self.state.sp = self.get_hl() as usize;
        }
        0xfa => {
          println!("JM");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.s == 1 {
            self.state.pc = address;
          }
        }
        0xfb => {
          println!("EI");
          // TODO special??
        }
        0xfc => {
          println!("CM ADDR");
          let address = self.get_next_2_bytes_as_usize();
          if self.state.flags.s == 1 {
            let return_address = ((self.state.pc) as u16).to_le_bytes();
            self.state.memory[self.state.sp - 1] = return_address[0];
            self.state.memory[self.state.sp - 2] = return_address[1];
            self.state.sp -= 2;
            self.state.pc = address;
          }
        }
        0xfe => {
          println!("CPI D8");
          let (answer, _) = self.state.a.overflowing_sub(self.state.memory[self.state.pc]);
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < self.state.memory[self.state.pc] {
            1
          } else {
            0
          };
  
          self.state.pc += 1;
        }
        _ => {
          panic!("unhandled op {:x}", op);
        } 
      }
      print_debug_info(&self.state, n);
    }
  }

  fn get_next_2_bytes_as_usize(&mut self) -> usize {
    let val = u16::from_le_bytes([self.state.memory[self.state.pc], self.state.memory[self.state.pc + 1]]) as usize;
    self.state.pc += 2;
    val
  }

  fn get_memory_at_hl(&self) -> u8 {
    self.state.memory[self.get_hl() as usize]
  }
  fn get_memory_at_de(&self) -> u8 {
    self.state.memory[self.get_de() as usize]
  }

  fn get_memory_at_bc(&self) -> u8 {
    self.state.memory[self.get_bc() as usize]
  }

  fn get_de(&self) -> u16 {
    u16::from_le_bytes([self.state.e, self.state.d])
  }

  fn get_hl(&self) -> u16 {
    u16::from_le_bytes([self.state.l, self.state.h])
  }

  fn get_bc(&self) -> u16 {
    u16::from_le_bytes([self.state.c, self.state.b])
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
  fn get_current_op(&mut self) -> u8 {
    let op = self.state.memory[self.state.pc];
    println!("{:x} op", op);
    self.state.pc += 1;
    op
  }
}

pub fn parity(b: u8) -> u8 {
  if b.count_ones() % 2 == 0 {
    1
  } else {
    0
  }
}

fn print_debug_info(state: &State, n: i32) {
  println!("n {}", n);
  println!("{0: <2} | {1: <2} | {2: <2} | {3: <2} | {4: <2} | {5: <2} | {6: <2} | {7: <4} | {8: <4} | {9: <5}",
            "a", "b", "c", "d", "e", "h", "l", "pc", "sp", "flags");
  print!("{:02x} | {:02x} | {:02x} | {:02x} | {:02x} | {:02x} | {:02x} | {:04x} | {:04x} | {}",
            state.a, state.b, state.c, state.d, state.e, state.h, state.l, state.pc, state.sp, "");
  
  if state.flags.z == 1 {
    print!("z");
  } else {
    print!(".");
  }

  if state.flags.s == 1 {
    print!("s");
  } else {
    print!(".");
  }

  if state.flags.p == 1 {
    print!("p");
  } else {
    print!(".");
  }


  if state.flags.cy == 1 {
    print!("c");
  } else {
    print!(".");
  }

  println!();
}

#[cfg(test)]
mod test {
  use std::fs;
  use crate::emulator::Emulator;
  #[test]
  fn cpudiag() {
    let result = fs::read("cpudiag.bin");
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
        println!("{:x}", bytes[0x0309]);
        let mut emu = Emulator::new(bytes);
        emu.run();
    } else {
        println!("Error reading file {:?}", result);
    }
  }
}
