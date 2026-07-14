use core::panic;



#[derive(Debug)]
enum Opcode {
    Halt,
    Load { dst: usize, val: u32 },
    Add  { r1: usize, r2: usize, dest: usize},
    Sub  { r1: usize, r2: usize, dest: usize},
}
impl From<u16> for Opcode {
    fn from(instruction: u16) -> Self {
        let reg = (instruction  &  0xF000) >> 12;
        let o1  = (instruction  &  0xF00) >> 8;
        let o2  = (instruction  &  0xF0) >> 4;
        let o3  = instruction   &  0xF;
        let imm = (instruction  &  0xFF) as u32;

        match reg {
            0 => Opcode::Halt,
            1 => Opcode::Load { dst: o1.into(), val: imm },
            2 => Opcode::Add  { r1: o1.into(), r2: o2.into(), dest: o3.into() },
            3 => Opcode::Add  { r1: o1.into(), r2: o2.into(), dest: o3.into() },
            4 => Opcode::Sub  { r1: o1.into(), r2: o2.into(), dest: o3.into() },
            _ => panic!("Unknown instruction")
        }
    }
}


fn main() {

    let prog: [u16;4] = [ 0x1064, 0x11C8, 0x2012, 0x0000 ];
    let mut regs: [u32;4] = [0,0,0,0];


    let mut ip = 0;
    let mut done = false;
    while !done {
        let instruction = Opcode::from(prog[ip]);
        println!("{:?}", instruction);

        match instruction {
            Opcode::Halt => { 
                done = true;
                ip+=1;
            },
            Opcode::Load { dst, val } => {
                regs[dst]=val;
                ip+=1;
            },
            Opcode::Add { r1, r2, dest } => {
                let lhs = regs[r1];
                let rhs = regs[r2];
                regs[dest]=lhs+rhs;
                ip+=1;
            },
            Opcode::Sub { r1, r2, dest } => { 
                let lhs = regs[r1];
                let rhs = regs[r2];
                regs[dest]=lhs-rhs;
                ip+=1;
            }, 
        }

        println!("{:?}", regs);
    }
}
