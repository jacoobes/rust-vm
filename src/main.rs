use core::panic;



#[derive(Debug)]
enum Opcode {
    Halt,
    Load { dst: usize, val: u32 },
    Add  { r1: usize, r2: usize, dest: usize},
    Sub  { r1: usize, r2: usize, dest: usize},
}
impl From<u32> for Opcode {
    fn from(instruction: u32) -> Self {
        let reg: usize = ((instruction  &  0xF0000000) >> 28).try_into().unwrap();
        let o1: usize  = ((instruction  &  0x0F000000) >> 24).try_into().unwrap();
        let o2: usize  = ((instruction  &  0x00F00000) >> 20).try_into().unwrap();
        let o3: usize =  ((instruction  &  0x000F0000) >> 16).try_into().unwrap();
        let imm = instruction  &  0x0000FFFF;

        match reg {
            0 => Opcode::Halt,
            1 => Opcode::Load { dst: o1, val: imm },
            2 => Opcode::Add  { r1: o1, r2: o2, dest: o3 },
            3 => Opcode::Sub { r1: o1, r2: o2, dest: o3 },
            _ => panic!("Unknown instruction")
        }
    }
}


fn main() {

    let prog: [u32;4] = [ 0x10000064, 0x110000C8, 0x20120000, 0x0];
    let mut regs: [u32;10] = [0;10];


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
