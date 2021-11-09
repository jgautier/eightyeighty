pub trait IO {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, val: u8);
}

pub struct SpaceInvadersIO {
    // read ports
    port0: u8,
    port1: u8,
    port2: u8,
    
    shift_register: u16,
    shift_amount: u8
}

impl SpaceInvadersIO {
    pub fn new() -> Self {
        Self {
            shift_register: 0,
            shift_amount: 0,
            port0: 0b0111_0000,
            port1: 0b0001_0000,
            port2: 0b0000_0000
        }
    }
    
}

impl IO for SpaceInvadersIO {
    fn input(&self, port: u8) -> u8 {
        match port {
            1 => self.port1,
            2 => self.port2,
            3 => (self.shift_register >> (8 - self.shift_amount)) as u8,
            _ => panic!("unhandled input port {}", port)
        }
    }
    fn output(&mut self, port: u8, val: u8) {
        match port {
            2 => self.shift_amount = val & 0b111,
            4 => {
                let [_, val2] = u16::to_le_bytes(self.shift_register);
                self.shift_register = u16::from_le_bytes([val2, val])
            },
            3 | 5 | 6 => {},
            _ => panic!("cannot write to port {}", port)
        }

    }
}

#[cfg(test)]
mod test {
    use crate::machine::SpaceInvadersIO;
    #[test]
    fn test() {
        println!("Hello World")
    }
}