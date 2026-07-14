use core::panic;



#[derive(Debug)]
enum Opcode {
    Halt,
    Load { dst: u16, val: u16 },
    Add  { r1: u16, r2: u16, dest: u16},
}
impl From<u16> for Opcode {
    fn from(instruction: u16) -> Self {
        let reg = (instruction  &  0xF000) >> 12;
        let o1 =  (instruction  &  0xF00) >> 8;
        let o2 =  (instruction  &  0xF0) >> 4;
        let o3  = instruction  & 0xF;
        let imm = instruction   &  0xFF;

        match reg {
            0 => Opcode::Halt,
            1 => Opcode::Load { dst: o1, val: imm },
            2 => Opcode::Add { r1: o1, r2: o2, dest: o3 },
            _ => panic!("Unknown instruction")
        }
    }
}


fn main() {

    let prog: [u16;4] = [ 0x1064, 0x11C8, 0x2201, 0x0000 ];

    let mut ip = 0;
    while ip < prog.len() {
        let instruction = Opcode::from(prog[ip]);
        
        println!(r#"{:?}"#, instruction);
        ip+=1;
    }


}
