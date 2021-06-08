use std::thread;

enum Op {
  NOP = 0x01
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

  pub fn run(&mut self) {
    let mut n = 0;
    while self.state.pc < self.program_size {
      n += 1;
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
        0x04 => {
          println!("INCR B");
          let (answer, overflowed) = self.state.b.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.b = answer;
        }
        0x05 => {
          println!("B {:x}", self.state.b);
          let mut b = self.state.b;
          b = b.wrapping_sub(1);
          self.state.b = b;
          self.set_flags(b);
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
        0x0c => {
          println!("INR C");
          let (answer, overflowed) = self.state.c.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
  
          self.state.c = answer as u8;
        }
        0x0d => {
          println!("DCR C");
          let (answer, overflowed) = self.state.c.overflowing_sub(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
  
          self.state.c = answer as u8;
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
        0x14 => {
          println!("INR D");
          let (answer, overflowed) = self.state.d.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.d = answer;
        }
        0x15 => {
          println!("DECR D");
          let (answer, overflowed) = self.state.d.overflowing_sub(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.d = answer;
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
        0x1c => {
          println!("INR E");
          let (answer, overflowed) = self.state.e.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.e = answer;
        }
        0x1d => {
          println!("DECR E");
          let (answer, overflowed) = self.state.e.overflowing_sub(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.e = answer;
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
        0x24 => {
          println!("INR H");
          let (answer, overflowed) = self.state.h.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.h = answer;
        }
        0x25 => {
          println!("DECR H");
          let (answer, overflowed) = self.state.h.overflowing_sub(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.h = answer;
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
        0x2c => {
          println!("INR L");
          let (answer, overflowed) = self.state.l.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.l = answer;
        }
        0x2d => {
          println!("DECR L");
          let (answer, overflowed) = self.state.l.overflowing_sub(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.l = answer;
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
        0x3c => {
          println!("INCR A");
          let (answer, overflowed) = self.state.a.overflowing_add(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x3d => {
          println!("DECR A");
          let (answer, overflowed) = self.state.a.overflowing_sub(1);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x3f => {
          println!("CMC");
          self.state.flags.cy = if self.state.flags.cy == 1 {
            0
          } else {
            1
          }
        }
        0x41 => {
          println!("MOV B,C");
          self.state.b = self.state.c;
        }
        0x42 => {
          println!("MOV B,D");
          self.state.b = self.state.d;
        }
        0x43 => {
          println!("MOV B,E");
          self.state.b = self.state.e;
        }
        0x44 => {
          println!("MOV B,H");
          self.state.b = self.state.h;
        }
        0x45 => {
          println!("MOV B,L");
          self.state.b = self.state.l;
        }
        0x46 => {
          println!("MOV B, M");
          self.state.b = self.get_memory_at_hl();
        }
        0x47 => {
          println!("MOV B, A");
          self.state.b = self.state.a;
        }
        0x48 => {
          println!("MOV C, B");
          self.state.c = self.state.b;
        }
        0x4a => {
          println!("MOV C,D");
          self.state.c = self.state.d;
        }
        0x4b => {
          println!("MOV C,E");
          self.state.c = self.state.e;
        }
        0x4c => {
          println!("MOV C,H");
          self.state.c = self.state.h;
        }
        0x4f => {
          println!("MOV C, A");
          self.state.c = self.state.a;
        }
        0x4d => {
          println!("MOV C,L");
          self.state.c = self.state.l;
        }
        0x50 => {
          println!("MOV D,B");
          self.state.d = self.state.b;
        }
        0x51 => {
          println!("MOV D, C");
          self.state.d = self.state.c;
        }
        0x53 => {
          println!("MOV D,E");
          self.state.d = self.state.e;
        }
        0x54 => {
          println!("MOV D,H");
          self.state.d = self.state.h;
        }
        0x55 => {
          println!("MOV D,L");
          self.state.d = self.state.l;
        }
        0x56 => {
          println!("MOV D,M");
          self.state.d = self.get_memory_at_hl();
        }
        0x57 => {
          println!("MOV D,A");
          self.state.d = self.state.a;
        }
        0x58 => {
          println!("MOV E,B");
          self.state.e = self.state.b;
        }
        0x59 => {
          println!("MOV C, E");
          self.state.e = self.state.c;
        }
        0x5a => {
          println!("MOV E, D");
          self.state.e = self.state.d;
        }
        0x5c => {
          println!("MOV E,H");
          self.state.e = self.state.h;
        }
        0x5d => {
          println!("MOV E,L");
          self.state.e = self.state.l;
        }
        0x5e => {
          println!("MOV E,M");
          self.state.e = self.get_memory_at_hl();
        }
        0x5f => {
          println!("MOV E,A");
          self.state.e = self.state.a;
        }
        0x60 => {
          println!("MOV H,B");
          self.state.h = self.state.b;
        }
        0x61 => {
          println!("MOV H,C");
          self.state.h = self.state.c;
        }
        0x62 => {
          println!("MOV H,D");
          self.state.h = self.state.d;
        }
        0x63 => {
          println!("MOV H, E");
          self.state.h = self.state.e;
        }
        0x65 => {
          println!("MOV H,L");
          self.state.h = self.state.l;
        }	
        0x66 => {
          println!("MOV H,M");
          self.state.h = self.get_memory_at_hl();
        }
        0x67 => {
          println!("MOV H,A");
          self.state.h = self.state.a;
        }
        0x68 => {
          println!("MOV L,B");
          self.state.l = self.state.b;
        }
        0x69 => {
          println!("MOV L,C");
          self.state.l = self.state.c;
        }
        0x6a => {
          println!("MOV L,D");
          self.state.l = self.state.d;
        }
        0x6b => {
          println!("MOV L,E");
          self.state.l = self.state.e;
        }
        0x6c => {
          println!("MOV L,H");
          self.state.l = self.state.h;
        }
        0x6e => {
          println!("MOV L, M");
          self.state.l = self.get_memory_at_hl();
        }
        0x6f => {
          println!("MOV L,A");
          self.state.l = self.state.a;
        }
        0x70 => {
          println!("MOV M, B");
          self.state.memory[self.get_hl() as usize] = self.state.b;
        }
        0x72 => {
          println!("MOV M, d");
          self.state.memory[self.get_hl() as usize] = self.state.d;
        }
        0x73 => {
          println!("MOV M, e");
          self.state.memory[self.get_hl() as usize] = self.state.e;
        }
        0x74 => {
          println!("MOV M, h");
          self.state.memory[self.get_hl() as usize] = self.state.h;
        }
        0x75 => {
          println!("MOV M, l");
          self.state.memory[self.get_hl() as usize] = self.state.l;
        }
        0x77 => {
          println!("MOV M,A");
          self.state.memory[self.get_hl() as usize] = self.state.a;
        }
        0x78 => {
          println!("MOV A,B");
          self.state.a = self.state.b;
        }
        0x79 => {
          println!("MOV A,C");
          self.state.a = self.state.c;
        }
        0x7a => {
          println!("MOV A,D");
          self.state.a = self.state.d;
        }
        0x7b => {
          println!("MOV A,E");
          self.state.a = self.state.e;
        }
        0x7c => {
          println!("MOV H,A");
          self.state.a = self.state.h;
        }
        0x7d => {
          println!("MOV A, L");
          self.state.a = self.state.l;
        }
        0x7e => {
          println!("MOV A,M");
          self.state.a = self.get_memory_at_hl();
        }
        0x80 => {
          println!("ADD B");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.b);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x81 => {
          println!("ADD C");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.c);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x82 => {
          println!("ADD D");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.d);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x83 => {
          println!("ADD E");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.e);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x84 => {
          println!("ADD H");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.h);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x85 => {
          println!("ADD L");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.l);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
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
        0x87 => {
          println!("ADD A");
          let (answer, overflowed) = self.state.a.overflowing_add(self.state.a);
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
        0x90 => {
          println!("SUB B");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.b);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x91 => {
          println!("SUB B");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.c);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x92 => {
          println!("SUB D");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.d);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x93 => {
          println!("SUB E");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.e);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x94 => {
          println!("SUB H");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.h);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
        }
        0x95 => {
          println!("SUB L");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.l);
          self.set_flags(answer);
          self.state.flags.cy = if overflowed {
            1
          } else {
            0
          };
          self.state.a = answer;
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
        0x97 => {
          println!("SUB A");
          let (answer, overflowed) = self.state.a.overflowing_sub(self.state.a);
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
        0xa1 => {
          println!("ANA C");
          let answer = self.state.a & self.state.c;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xa2 => {
          println!("ANA D");
          let answer = self.state.a & self.state.d;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xa3 => {
          println!("ANA e");
          let answer = self.state.a & self.state.e;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xa4 => {
          println!("ANA h");
          let answer = self.state.a & self.state.h;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xa5 => {
          println!("ANA L");
          let l = self.state.l;
          let answer = self.state.a & l;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
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
        0xa7 => {
          println!("ANA A");
          let a = self.state.a;
          let answer = self.state.a & a;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xa8 => {
          println!("XRA B");
          let answer = self.state.a ^ self.state.b;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xa9 => {
          println!("XRA C");
          let answer = self.state.a ^ self.state.c;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xaa => {
          println!("XRA d");
          let answer = self.state.a ^ self.state.d;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xab => {
          println!("XRA e");
          let answer = self.state.a ^ self.state.e;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xac => {
          println!("XRA h");
          let answer = self.state.a ^ self.state.h;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xad => {
          println!("XRA l");
          let answer = self.state.a ^ self.state.l;
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
        0xaf => {
          println!("XRA A");
          let a = self.state.a;
          let answer = self.state.a ^ a;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb0 => {
          println!("ORA B");
          let answer = self.state.a | self.state.b;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb1 => {
          println!("ORA c");
          let answer = self.state.a | self.state.c;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb2 => {
          println!("ORA D");
          let answer = self.state.a | self.state.d;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb3 => {
          println!("ORA E");
          let answer = self.state.a | self.state.e;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb4 => {
          println!("ORA h");
          let answer = self.state.a | self.state.h;
          self.set_flags(answer);
          self.state.flags.cy = if self.state.a < (answer as u8) {
            1
          } else {
            0
          };
          self.state.a = answer as u8;
        }
        0xb5 => {
          println!("ORA l");
          let answer = self.state.a | self.state.l;
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
        0xb7 => {
          println!("ORA A");
          let answer = self.state.a;
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


  fn print_debug_info(state: &State, n: i32) {
    print!("\x1B[2J\x1B[1;1H");
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
