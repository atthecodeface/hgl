use hgl_sim::prelude::*;

const NUM_REGS: usize = 16;

type Rn = usize;

#[derive(Debug, Clone, Copy)]
enum AluOp {
    Add,
    Sub,
    Xor,
    And,
    Or,
    Not,
    Neg,
}

impl AluOp {
    fn random(mut n: usize) -> Self {
        match n % 7 {
            0 => Self::Add,
            1 => Self::Sub,
            2 => Self::Xor,
            3 => Self::And,
            4 => Self::Or,
            5 => Self::Not,
            6 => Self::Neg,
            _ => Self::Add,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OpCode {
    pub rd: Rn,
    pub rs1: Rn,
    pub rs2: Rn,
    pub op: AluOp,
}

impl OpCode {
    fn new_random(mut n: usize) -> Self {
        let rd = NUM_REGS/4 + (n % (NUM_REGS-NUM_REGS/4));
        n /= NUM_REGS;
        let rs1 = n % NUM_REGS;
        n /= NUM_REGS;
        let rs2 = n % NUM_REGS;
        n /= NUM_REGS;
        let op = AluOp::random(n);
        Self { rd, rs1, rs2, op }
    }
    fn perform<D: BvSim>(&self, rs1: &D, rs2: &D) -> D {
        use AluOp::*;
        eprintln!("{rs1:?} {:?} {rs2:?}", self.op);
        match self.op {
            Add => *rs1 + *rs2,
            Sub => *rs1 - *rs2,
            Xor => *rs1 ^ *rs2,
            And => *rs1 & *rs2,
            Or => *rs1 | *rs2,
            Not => !*rs1,
            Neg => -*rs1,
            // _ => *rs1,
        }
    }
}

struct Program {
    ops: Vec<OpCode>,
}

impl Program {
    fn new() -> Self {
        let ops = vec![];
        Self { ops }
    }
    fn push(&mut self, op: OpCode) {
        self.ops.push(op);
    }
}

#[derive(Debug, Default)]
pub struct Cpu<D, const NB: usize>
where
    D: BvSim,
{
    regs: [D; NUM_REGS],
}

impl<D, const NB: usize> Cpu<D, NB>
where
    D: BvSim,
{
    fn new() -> Self {
        let mut regs = <[D; NUM_REGS]>::default();
        for i in 0..NUM_REGS {
            regs[i].set_u64(i as u64);
        }
        Self { regs }
    }
    fn randomize_reg(&mut self, rd:usize, n:usize) {
        self.regs[rd].set_u64((n.wrapping_mul(0xd128fd3)) as u64);
    }
    fn do_op(&mut self, op: &OpCode) {
        let rs1 = self.regs[op.rs1];
        let rs2 = self.regs[op.rs2];
        let rd = op.perform(&rs1, &rs2);
        self.regs[op.rd] = rd;
    }
    fn execute_program(&mut self, p:&Program) {
        for o in p.ops.iter() {
            self.do_op(o);
        }
    }
}

#[test]
fn construct_128() -> Result<(), String> {
    let mut cpu: Cpu<Bv<128>, 128> = Cpu::new();
    for i in 4..NUM_REGS {
        cpu.randomize_reg(i,(i+12384).wrapping_mul(0x233123d45));
    }
    dbg!(&cpu);
    let mut p = Program::new();
    for i in 0..1000 {
        p.push(OpCode::new_random(i*0x12372348f))
    }
    cpu.execute_program(&p); 
    dbg!(&cpu);
   assert!(false);

    Ok(())
}
