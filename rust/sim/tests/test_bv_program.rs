//a Imports
use hgl_sim::prelude::*;
use rand::rngs::ThreadRng;
use rand::RngCore;

//a Register and CPU
type Rn = usize;

//ti Cpu
#[derive(Debug)]
pub struct Cpu<D, const NB: usize, const NR: usize>
where
    D: BvSim,
{
    regs: [D; NR],
}

//ip Cpu
impl<D, const NB: usize, const NR: usize> std::default::Default for Cpu<D, NB, NR>
where
    D: BvSim,
{
    fn default() -> Self {
        let regs = [D::default(); NR];
        Self { regs }
    }
}

impl<D, const NB: usize, const NR: usize> Cpu<D, NB, NR>
where
    D: BvSim,
{
    //ci new
    fn new() -> Self {
        let mut regs = [D::default(); NR];
        regs[0].set_u64(0);
        regs[1].set_u64(1);
        regs[2] = regs[0] - regs[1];
        regs[3] = regs[0] + regs[1];
        for i in 4..NR {
            regs[i] = regs[i-1] + regs[i-1];
        }
        Self { regs }
    }

    //mi randomize_reg
    fn randomize_reg(&mut self, rd: usize, n: u64) {
        self.regs[rd].set_u64(n);
    }

    //mi do_op
    fn do_op(&mut self, op: &OpCode) {
        let rs1 = self.regs[op.rs1];
        let rs2 = self.regs[op.rs2];
        let rd = op.perform(&rs1, &rs2);
        self.regs[op.rd] = rd;
    }

    //mi execute_program
    fn execute_program(&mut self, p: &Program) {
        for o in p.ops.iter() {
            self.do_op(o);
        }
    }
}

//a OpCode, AluOp
//ti AluOp
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

//ip AluOp
impl AluOp {
    fn random(n: usize) -> (Self, usize) {
        let op_n = n % 60;
        let op = {
            if op_n < 2 {
                if op_n&1 == 0 {
                    Self::Not
                } else {
                    Self::Neg
                }
            } else if op_n<4 {
                if op_n&1 == 0 {
                    Self::And
                } else {
                    Self::Or
                }
            } else {
                match op_n%3 {
                    0 => Self::Add,
                    1 => Self::Sub,
                    _ => Self::Xor,
                }
            }
        };
        (op, n / 60)
    }
}

//ti OpCode
#[derive(Debug, Clone, Copy)]
struct OpCode {
    pub rd: Rn,
    pub rs1: Rn,
    pub rs2: Rn,
    pub op: AluOp,
}

//ip OpCode
impl OpCode {
    //ci new_random
    fn new_random<const NR:usize>(fixed_r:usize, mut n: usize) -> Self {
        let rd = fixed_r + n % (NR - fixed_r);
        n /= fixed_r;
        let rs1 = n % NR;
        n /= NR;
        let rs2 = n % NR;
        n /= NR;
        let (op, _) = AluOp::random(n);
        Self { rd, rs1, rs2, op }
    }

    //mi perform
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

//a Program
//ti Program
struct Program {
    ops: Vec<OpCode>,
}

//ip Program
impl Program {
    fn new() -> Self {
        let ops = vec![];
        Self { ops }
    }
    fn push(&mut self, op: OpCode) {
        self.ops.push(op);
    }
}

#[test]
fn construct_128() -> Result<(), String> {
    const NR:usize = 256;
    const NF:usize = 4;
    let mut rng = rand::thread_rng();

    let mut cpu: Cpu<Bv<128>, 128, NR> = Cpu::new();
    for i in 0..NR/2 {
        cpu.randomize_reg(i+NR/2, rng.next_u64() );
    }
    dbg!(&cpu);
    let mut p = Program::new();
    for i in 0..1000 {
        p.push(OpCode::new_random::<NR>(NF, i * 0x12372348f))
    }
    cpu.execute_program(&p);
    dbg!(&cpu);
    assert!(false);

    Ok(())
}
