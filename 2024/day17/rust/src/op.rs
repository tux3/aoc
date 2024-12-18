pub enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instr {
    fn from(value: u8) -> Self {
        match value {
            0 => Instr::Adv,
            1 => Instr::Bxl,
            2 => Instr::Bst,
            3 => Instr::Jnz,
            4 => Instr::Bxc,
            5 => Instr::Out,
            6 => Instr::Bdv,
            7 => Instr::Cdv,
            _ => panic!("Invalid instruction opcode {value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub code: Vec<u8>,
    pub regs: Vec<usize>,
    pub ip: usize,
    pub out: Vec<u8>,
}

impl State {
    pub fn fetch(&self) -> Option<(Instr, u8)> {
        (self.ip + 1 < self.code.len())
            .then(|| (Instr::from(self.code[self.ip]), self.code[self.ip + 1]))
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.out.clear();
    }

    fn combo_op(&self, op: u8) -> usize {
        match op {
            0..=3 => op as usize,
            4..=6 => self.regs[op as usize - 4],
            _ => panic!("Unexpected combo op"),
        }
    }

    pub fn exec(&mut self, instr: Instr, op: u8) {
        self.ip += 2;
        match instr {
            Instr::Adv => self.regs[0] /= 2usize.pow(self.combo_op(op) as u32),
            Instr::Bxl => self.regs[1] ^= op as usize,
            Instr::Bst => self.regs[1] = self.combo_op(op) & 0b111,
            Instr::Jnz => {
                if self.regs[0] != 0 {
                    self.ip = op as usize;
                }
            }
            Instr::Bxc => self.regs[1] ^= self.regs[2],
            Instr::Out => self.out.push(self.combo_op(op) as u8 & 0b111),
            Instr::Bdv => self.regs[1] = self.regs[0] / 2usize.pow(self.combo_op(op) as u32),
            Instr::Cdv => self.regs[2] = self.regs[0] / 2usize.pow(self.combo_op(op) as u32),
        }
    }
}
