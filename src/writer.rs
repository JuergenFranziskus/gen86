use std::io;

use crate::{gp_regs::Reg, mem::Mem, operand::Operand};

pub trait X86Writer {
    fn emit_preamble(&mut self) -> io::Result<()>;
    fn label(&mut self, label: &str) -> io::Result<()>;
    fn global(&mut self, label: &str) -> io::Result<()>;
    fn text(&mut self) -> io::Result<()>;

    fn adc<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn add<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn and<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn call<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn cdq(&mut self) -> io::Result<()>;
    fn clc(&mut self) -> io::Result<()>;
    fn cld(&mut self) -> io::Result<()>;
    fn cli(&mut self) -> io::Result<()>;
    fn cmp<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn cmov<'a, 'b>(
        &mut self,
        cc: Condition,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn cqo(&mut self) -> io::Result<()>;
    fn cwd(&mut self) -> io::Result<()>;
    fn dec<'a>(&mut self, rd: impl Into<Operand<'a>>) -> io::Result<()>;
    fn div<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn idiv<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn imul1<'a, 'b>(&mut self, rd: impl Into<Operand<'a>>) -> io::Result<()>;
    fn imul2<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn imul3<'a, 'b, 'c>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs0: impl Into<Operand<'b>>,
        rs1: impl Into<Operand<'c>>,
    ) -> io::Result<()>;
    fn inc<'a>(&mut self, rd: impl Into<Operand<'a>>) -> io::Result<()>;
    fn jcc<'a>(&mut self, cc: Condition, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn jmp<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn lea<'a>(&mut self, rd: Reg, rs: Mem<'a>) -> io::Result<()>;
    fn mov<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn movsx<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn movzx<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn mul<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn neg<'a>(&mut self, rd: impl Into<Operand<'a>>) -> io::Result<()>;
    fn not<'a>(&mut self, rd: impl Into<Operand<'a>>) -> io::Result<()>;
    fn nop(&mut self) -> io::Result<()>;
    fn nop1<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn or<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn pop<'a>(&mut self, rd: impl Into<Operand<'a>>) -> io::Result<()>;
    fn push<'a>(&mut self, rs: impl Into<Operand<'a>>) -> io::Result<()>;
    fn rcl<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn rcr<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn ret(&mut self) -> io::Result<()>;
    fn rol<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn ror<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn sal<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn sar<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn sbb<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn shl<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn shr<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn stc(&mut self) -> io::Result<()>;
    fn std(&mut self) -> io::Result<()>;
    fn sti(&mut self) -> io::Result<()>;
    fn sub<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn syscall(&mut self) -> io::Result<()>;
    fn test<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
    fn xor<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    A,
    AE,
    B,
    BE,
    C,
    CXZ,
    ECXZ,
    RCXZ,
    E,
    G,
    GE,
    L,
    LE,
    NA,
    NAE,
    NB,
    NBE,
    NC,
    NE,
    NG,
    NGE,
    NL,
    NLE,
    NO,
    NP,
    NS,
    NZ,
    O,
    P,
    PE,
    PO,
    S,
    Z,
}
