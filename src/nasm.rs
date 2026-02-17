use std::io::{self, Write};

use crate::gp_regs::*;
use crate::mem::{Mem, Scale};
use crate::operand::{OSize, Operand};
use crate::writer::Condition;
use crate::writer::X86Writer;

pub struct NasmWriter<O> {
    out: O,
}
impl<O: Write> NasmWriter<O> {
    pub fn new(out: O) -> Self {
        Self { out }
    }

    fn emit_triop<'a, 'b, 'c>(
        &mut self,
        name: &str,
        rd: impl Into<Operand<'a>>,
        rs0: impl Into<Operand<'b>>,
        rs1: impl Into<Operand<'c>>,
    ) -> io::Result<()> {
        let rd = rd.into();
        let rs0 = rs0.into();
        let rs1 = rs1.into();

        write!(self.out, "    {name} ")?;
        self.print_operand(&rd)?;
        write!(self.out, ", ")?;
        self.print_operand(&rs0)?;
        write!(self.out, ", ")?;
        self.print_operand(&rs1)?;
        writeln!(self.out)?;

        Ok(())
    }
    fn emit_binop<'a, 'b>(
        &mut self,
        name: &str,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()> {
        let rd = rd.into();
        let rs = rs.into();

        write!(self.out, "    {name} ")?;
        self.print_operand(&rd)?;
        write!(self.out, ", ")?;
        self.print_operand(&rs)?;
        writeln!(self.out)?;

        Ok(())
    }
    fn emit_binop_cc<'a, 'b>(
        &mut self,
        name: &str,
        cc: Condition,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> io::Result<()> {
        let rd = rd.into();
        let rs = rs.into();

        write!(self.out, "    {name}")?;
        self.print_cc(cc)?;
        write!(self.out, " ")?;
        self.print_operand(&rd)?;
        write!(self.out, ", ")?;
        self.print_operand(&rs)?;
        writeln!(self.out)?;

        Ok(())
    }
    fn emit_unop<'a>(&mut self, name: &str, r: impl Into<Operand<'a>>) -> io::Result<()> {
        let r = r.into();

        write!(self.out, "    {name} ")?;
        self.print_operand(&r)?;
        writeln!(self.out)?;

        Ok(())
    }
    fn emit_unop_cc<'a>(
        &mut self,
        name: &str,
        cc: Condition,
        r: impl Into<Operand<'a>>,
    ) -> io::Result<()> {
        let r = r.into();

        write!(self.out, "    {name}")?;
        self.print_cc(cc)?;
        write!(self.out, " ")?;
        self.print_operand(&r)?;
        writeln!(self.out)?;

        Ok(())
    }
    fn emit_nulop(&mut self, name: &str) -> io::Result<()> {
        writeln!(self.out, "    {name}")?;
        Ok(())
    }

    fn print_cc(&mut self, cc: Condition) -> io::Result<()> {
        let name = match cc {
            Condition::A => "a",
            Condition::AE => "ae",
            Condition::B => "b",
            Condition::BE => "be",
            Condition::C => "c",
            Condition::CXZ => "cxz",
            Condition::ECXZ => "ecxz",
            Condition::RCXZ => "rcxz",
            Condition::E => "e",
            Condition::G => "g",
            Condition::GE => "ge",
            Condition::L => "l",
            Condition::LE => "le",
            Condition::NA => "na",
            Condition::NAE => "nae",
            Condition::NB => "nb",
            Condition::NBE => "nbe",
            Condition::NC => "nc",
            Condition::NE => "ne",
            Condition::NG => "ng",
            Condition::NGE => "nge",
            Condition::NL => "nl",
            Condition::NLE => "nle",
            Condition::NO => "no",
            Condition::NP => "np",
            Condition::NS => "ns",
            Condition::NZ => "nz",
            Condition::O => "o",
            Condition::P => "p",
            Condition::PE => "pe",
            Condition::PO => "po",
            Condition::S => "s",
            Condition::Z => "z",
        };
        write!(self.out, "{name}")?;

        Ok(())
    }
    fn print_operand(&mut self, op: &Operand) -> io::Result<()> {
        match op {
            &Operand::Reg(reg) => self.print_reg(reg),
            Operand::Mem(mem) => self.print_mem(mem),
            &Operand::Integer(value) => write!(self.out, "{value}"),
            &Operand::Label(label) => write!(self.out, "{label}"),
        }
    }

    fn print_mem(&mut self, mem: &Mem) -> io::Result<()> {
        let mut needs_plus = false;

        if let Some(size) = mem.size {
            self.print_size(size)?;
        }

        write!(self.out, "[")?;
        if let Some(label) = mem.label {
            needs_plus = true;
            write!(self.out, "{label}")?;
        }

        if let Some(base) = mem.base {
            if needs_plus {
                write!(self.out, " + ")?;
            }
            needs_plus = true;
            self.print_reg(base)?;
        }

        if let Some((index, scale)) = mem.index {
            if needs_plus {
                write!(self.out, " + ")?;
            }
            needs_plus = true;

            self.print_reg(index)?;
            match scale {
                Scale::One => (),
                Scale::Two => write!(self.out, " * 2")?,
                Scale::Four => write!(self.out, " * 4")?,
                Scale::Eight => write!(self.out, " * 8")?,
            }
        }

        if mem.offset != 0 {
            let is_neg = mem.offset < 0;
            if needs_plus && is_neg {
                write!(self.out, " - {}", mem.offset.unsigned_abs())?;
            } else if needs_plus {
                write!(self.out, " + {}", mem.offset)?;
            } else {
                write!(self.out, "{}", mem.offset)?;
            }
        }

        write!(self.out, "]")?;

        Ok(())
    }
    fn print_reg(&mut self, reg: Reg) -> io::Result<()> {
        let name = match reg {
            AL => "al",
            AX => "ax",
            EAX => "eax",
            RAX => "rax",
            BL => "bl",
            BX => "bx",
            EBX => "ebx",
            RBX => "rbx",
            CL => "cl",
            CX => "cx",
            ECX => "ecx",
            RCX => "rcx",
            DL => "dl",
            DX => "dx",
            EDX => "edx",
            RDX => "rdx",
            DIL => "dil",
            DI => "di",
            EDI => "edi",
            RDI => "rdi",
            SIL => "sil",
            SI => "si",
            ESI => "esi",
            RSI => "rsi",
            BPL => "bpl",
            BP => "bp",
            EBP => "ebp",
            RBP => "rbp",
            SPL => "spl",
            SP => "sp",
            ESP => "esp",
            RSP => "rsp",
            R8B => "r8b",
            R8W => "r8w",
            R8D => "r8d",
            R8 => "r8",
            R9B => "r9b",
            R9W => "r9w",
            R9D => "r9d",
            R9 =>  "r9",
            R10B => "r10b",
            R10W => "r10w",
            R10D => "r10d",
            R10 =>  "r10",
            R11B => "r11b",
            R11W => "r11w",
            R11D => "r11d",
            R11 =>  "r11",
            R12B => "r12b",
            R12W => "r12w",
            R12D => "r12d",
            R12 =>  "r12",
            R13B => "r13b",
            R13W => "r13w",
            R13D => "r13d",
            R13 =>  "r13",
            R14B => "r14b",
            R14W => "r14w",
            R14D => "r14d",
            R14 =>  "r14",
            R15B => "r15b",
            R15W => "r15w",
            R15D => "r15d",
            R15 =>  "r15",
        };

        write!(self.out, "{name}")?;
        Ok(())
    }
    fn print_size(&mut self, size: OSize) -> io::Result<()> {
        let name = match size {
            OSize::Byte => "byte",
            OSize::Word => "word",
            OSize::DWord => "dword",
            OSize::QWord => "qword",
        };

        write!(self.out, "{name}")?;
        Ok(())
    }
}
impl<O: Write> X86Writer for NasmWriter<O> {
    fn emit_preamble(&mut self) -> std::io::Result<()> {
        writeln!(self.out, "default rel")?;

        Ok(())
    }

    fn label(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.out, "{label}:")?;

        Ok(())
    }

    fn global(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.out, "global {label}")?;

        Ok(())
    }

    fn text(&mut self) -> std::io::Result<()> {
        writeln!(self.out, "section .text")?;

        Ok(())
    }

    fn rodata(&mut self) -> io::Result<()> {
        writeln!(self.out, "section .rodata")?;
        
        Ok(())
    }

    fn blank(&mut self) -> io::Result<()> {
        writeln!(self.out)?;

        Ok(())
    }
    fn comment(&mut self, comment: &str) -> io::Result<()> {
        writeln!(self.out, "  ; {comment}")?;

        Ok(())
    }

    fn db(&mut self, label: &str, bytess: &[&[u8]]) -> io::Result<()> {
        write!(self.out, "{label} db ")?;
        for (i, &bytes) in bytess.iter().enumerate() {
            let last = i == bytess.len() - 1;
            if is_ascii_printable(bytes) {
                let str = str::from_utf8(bytes).unwrap();
                write!(self.out, "\"{str}\"")?;
                if !last {
                    write!(self.out, ", ")?;
                }
            }
            else {   
                for (i, &byte) in bytes.iter().enumerate() {
                    let last = last && i == bytes.len() - 1;
                    write!(self.out, "{byte}")?;
                    if !last {
                        write!(self.out, ", ")?;
                    }
                }
            }
        }

        writeln!(self.out)?;

        Ok(())
    }
    fn equ(&mut self, label: &str, value: i64) -> io::Result<()> {
        writeln!(self.out, "{label} equ {value}")?;

        Ok(())
    }

    fn adc<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("adc", rd, rs)
    }

    fn add<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("add", rd, rs)
    }

    fn and<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("and", rd, rs)
    }

    fn call<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("call", rs)
    }

    fn cdq(&mut self) -> std::io::Result<()> {
        self.emit_nulop("cdq")
    }

    fn clc(&mut self) -> std::io::Result<()> {
        self.emit_nulop("clc")
    }

    fn cld(&mut self) -> std::io::Result<()> {
        self.emit_nulop("cld")
    }

    fn cli(&mut self) -> std::io::Result<()> {
        self.emit_nulop("cli")
    }

    fn cmp<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("cmp", rd, rs)
    }

    fn cmov<'a, 'b>(
        &mut self,
        cc: Condition,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop_cc("cmov", cc, rd, rs)
    }

    fn cqo(&mut self) -> std::io::Result<()> {
        self.emit_nulop("cqo")
    }

    fn cwd(&mut self) -> std::io::Result<()> {
        self.emit_nulop("cwd")
    }

    fn dec<'a>(&mut self, rd: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("dec", rd)
    }

    fn div<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("div", rs)
    }

    fn idiv<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("idiv", rs)
    }

    fn imul1<'a, 'b>(&mut self, rd: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("imul", rd)
    }

    fn imul2<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("imul", rd, rs)
    }

    fn imul3<'a, 'b, 'c>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs0: impl Into<Operand<'b>>,
        rs1: impl Into<Operand<'c>>,
    ) -> std::io::Result<()> {
        self.emit_triop("imul", rd, rs0, rs1)
    }

    fn inc<'a>(&mut self, rd: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("inc", rd)
    }

    fn jcc<'a>(&mut self, cc: Condition, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop_cc("j", cc, rs)
    }

    fn jmp<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("jmp", rs)
    }

    fn lea<'a>(&mut self, rd: Reg, rs: Mem<'a>) -> std::io::Result<()> {
        self.emit_binop("lea", rd, rs)
    }

    fn mov<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("mov", rd, rs)
    }

    fn movsx<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("movsx", rd, rs)
    }

    fn movzx<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("movzx", rd, rs)
    }

    fn mul<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("mul", rs)
    }

    fn neg<'a>(&mut self, rd: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("neg", rd)
    }

    fn not<'a>(&mut self, rd: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("not", rd)
    }

    fn nop(&mut self) -> std::io::Result<()> {
        self.emit_nulop("nop")
    }

    fn nop1<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("nop", rs)
    }

    fn or<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("or", rd, rs)
    }

    fn pop<'a>(&mut self, rd: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("pop", rd)
    }

    fn push<'a>(&mut self, rs: impl Into<Operand<'a>>) -> std::io::Result<()> {
        self.emit_unop("push", rs)
    }

    fn rcl<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("rcl", rd, rs)
    }

    fn rcr<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("rcr", rd, rs)
    }

    fn ret(&mut self) -> std::io::Result<()> {
        self.emit_nulop("ret")
    }

    fn rol<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("rol", rd, rs)
    }

    fn ror<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("ror", rd, rs)
    }

    fn sal<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("sal", rd, rs)
    }

    fn sar<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("sar", rd, rs)
    }

    fn sbb<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("sbb", rd, rs)
    }

    fn setcc<'a>(&mut self, cc: Condition, dst: impl Into<Operand<'a>>) -> io::Result<()> {
        self.emit_unop_cc("set", cc, dst)
    }

    fn shl<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("shl", rd, rs)
    }

    fn shr<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("shr", rd, rs)
    }

    fn stc(&mut self) -> std::io::Result<()> {
        self.emit_nulop("stc")
    }

    fn std(&mut self) -> std::io::Result<()> {
        self.emit_nulop("std")
    }

    fn sti(&mut self) -> std::io::Result<()> {
        self.emit_nulop("sti")
    }

    fn sub<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("sub", rd, rs)
    }

    fn syscall(&mut self) -> std::io::Result<()> {
        self.emit_nulop("syscall")
    }

    fn test<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("test", rd, rs)
    }

    fn xor<'a, 'b>(
        &mut self,
        rd: impl Into<Operand<'a>>,
        rs: impl Into<Operand<'b>>,
    ) -> std::io::Result<()> {
        self.emit_binop("xor", rd, rs)
    }
}


fn is_ascii_printable(bytes: &[u8]) -> bool {
    for &byte in bytes {
        if byte > 127 || byte < 32 || byte == b'"' { return false };
    }

    true
}
