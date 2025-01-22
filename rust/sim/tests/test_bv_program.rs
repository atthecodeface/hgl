//a Imports
use hgl_sim::prelude::*;
use rand::RngCore;

//a Register and CPU
type Rn = usize;

//ti Cpu
#[derive(Debug)]
pub struct Cpu<D, const NB: usize, const NR: usize>
where
    D: SimBv,
{
    regs: [D; NR],
    op_mask: Option<D>,
}

//ip Cpu
impl<D, const NB: usize, const NR: usize> std::default::Default for Cpu<D, NB, NR>
where
    D: SimBv,
{
    fn default() -> Self {
        let regs = [D::default(); NR];
        Self { regs, op_mask:None }
    }
}

impl<D, const NB: usize, const NR: usize> Cpu<D, NB, NR>
where
    D: SimBv,
{
    //ci new
    fn new(bound_to_bits:bool) -> Self {
        let mut regs = [D::default(); NR];
        regs[0].set_u64(0);
        regs[1].set_u64(1);
        regs[2] = regs[0] - regs[1];
        regs[3] = regs[0] + regs[1];
        for i in 4..NR {
            regs[i] = regs[i - 1] + regs[i - 1];
        }
        let mut op_mask = None;
        if bound_to_bits {
            if NB == 128 {
                op_mask = Some(regs[2]);
            } else {
                op_mask = Some((regs[1] << NB)-regs[1]);
            }
            for i in 0..NR {
                regs[i] &= op_mask.unwrap();
            }
        }
        Self { regs, op_mask }
    }

    //mi randomize_reg
    fn randomize_reg<F: FnMut() -> u64>(&mut self, rd: usize, f: &mut F) {
        self.regs[rd] = D::randomize(f);
    }

    //mi do_op
    fn do_op(&mut self, op: &OpCode) {
        let rs1 = self.regs[op.rs1];
        let rs2 = self.regs[op.rs2];
        eprintln!("{} = {rs1:?} {:?} {rs2:?}", op.rd, op.op);
        let rd = op.perform(&rs1, &rs2, &self.op_mask);
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
    Shl(usize),
    Shr(usize),
}

//ip AluOp
impl AluOp {
    fn random(n: usize, num_bits: usize) -> (Self, usize) {
        let op_n = n % 60;
        let mut n = n / 60;
        let op = {
            if op_n < 6 {
                match op_n {
                    0 => Self::Not,
                    1 => Self::Neg,
                    2 => Self::And,
                    3 => Self::Or,
                    _ => {
                        let by = n % (num_bits + 1);
                        n = n / (num_bits + 1);
                        if op_n == 4 {
                            Self::Shl(by)
                        } else {
                            Self::Shr(by)
                        }
                    }
                }
            } else {
                match op_n % 3 {
                    0 => Self::Add,
                    1 => Self::Sub,
                    _ => Self::Xor,
                }
            }
        };
        (op, n)
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
    fn new_random<const NR: usize>(num_bits: usize, fixed_r: usize, mut n: usize) -> Self {
        let rd = fixed_r + n % (NR - fixed_r);
        n /= fixed_r;
        let rs1 = n % NR;
        n /= NR;
        let rs2 = n % NR;
        n /= NR;
        let (op, _) = AluOp::random(n, num_bits);
        Self { rd, rs1, rs2, op }
    }

    //mi perform
    fn perform<D: SimBv>(&self, rs1: &D, rs2: &D, mask:&Option<D>) -> D {
        use AluOp::*;
        let r = match self.op {
            Add => *rs1 + *rs2,
            Sub => *rs1 - *rs2,
            Xor => *rs1 ^ *rs2,
            And => *rs1 & *rs2,
            Or => *rs1 | *rs2,
            Not => !*rs1,
            Neg => rs1.signed_neg(),
            Shl(n) => *rs1 << n,
            Shr(n) => *rs1 >> n,
            // _ => *rs1,
        };
        if let Some(mask) = mask {
            r & mask
        } else {
            r
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

fn test_cpu_with_u128<const NB: usize>() -> Result<(), String>
where BvN<NB>: IsBv
{
    const NR: usize = 256;
    const NF: usize = 4;
    let mut rng = rand::thread_rng();

    let mut cpu_bv: Cpu<Bv<NB>, NB, NR> = Cpu::new(false);
    let mut cpu_u128: Cpu<std::num::Wrapping<u128>, NB, NR> = Cpu::new(true);
    for i in 0..NR / 2 {
        cpu_bv.randomize_reg(i + NR / 2, &mut || rng.next_u64());
    }
    for i in 0..NR {
        cpu_u128.regs[i].as_u8s_mut()[0..(NB+7)/8].copy_from_slice(cpu_bv.regs[i].as_u8s());
    }

    let mut p = Program::new();
    for i in 0..10_000 {
        p.push(OpCode::new_random::<NR>(NB, NF, i * 0x12372348f))
    }

    cpu_bv.execute_program(&p);
    cpu_u128.execute_program(&p);

    for i in 0..NR {
        let e = cpu_u128.regs[i].as_u8s();
        let v = cpu_bv.regs[i].as_u8s();
        for j in 0..v.len() {
            assert_eq!(e[j],v[j], "Mismatch in byte {j} of reg {i}, left is cpu_u128");
        }
    }
    Ok(())
}

#[test]
fn construct_128() -> Result<(), String> {
    test_cpu_with_u128::<12>()?;
    test_cpu_with_u128::<32>()?;
    test_cpu_with_u128::<48>()?;
    test_cpu_with_u128::<64>()?;
    test_cpu_with_u128::<65>()?;
    test_cpu_with_u128::<97>()?;
    test_cpu_with_u128::<120>()?;
    test_cpu_with_u128::<127>()?;
    test_cpu_with_u128::<128>()?;
    Ok(())
}
