use crate::{
    gp_regs::{RSize, Reg},
    mem::Mem,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operand<'a> {
    Reg(Reg),
    Mem(Mem<'a>),
    Integer(i64),
    Label(&'a str),
}
impl Operand<'_> {
    pub fn size(&self) -> Option<OSize> {
        match self {
            Self::Reg(r) => Some(r.size.into()),
            Self::Mem(mem) => mem.size,
            Self::Integer(_) => None,
            Self::Label(_) => None,
        }
    }
}
impl From<Reg> for Operand<'static> {
    fn from(value: Reg) -> Self {
        Self::Reg(value)
    }
}
impl<'a> From<Mem<'a>> for Operand<'a> {
    fn from(value: Mem<'a>) -> Self {
        Self::Mem(value)
    }
}
impl From<i64> for Operand<'static> {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<i32> for Operand<'static> {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<i16> for Operand<'static> {
    fn from(value: i16) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<i8> for Operand<'static> {
    fn from(value: i8) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u64> for Operand<'static> {
    fn from(value: u64) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u32> for Operand<'static> {
    fn from(value: u32) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u16> for Operand<'static> {
    fn from(value: u16) -> Self {
        Self::Integer(value as i64)
    }
}
impl From<u8> for Operand<'static> {
    fn from(value: u8) -> Self {
        Self::Integer(value as i64)
    }
}
impl<'a> From<&'a str> for Operand<'a> {
    fn from(value: &'a str) -> Self {
        Self::Label(value)
    }
}
impl<'a> From<&'a String> for Operand<'a> {
    fn from(value: &'a String) -> Self {
        Self::Label(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OSize {
    Byte,
    Word,
    DWord,
    QWord,
}
impl From<RSize> for OSize {
    fn from(value: RSize) -> Self {
        match value {
            RSize::Byte => Self::Byte,
            RSize::Word => Self::Word,
            RSize::DWord => Self::DWord,
            RSize::QWord => Self::QWord,
        }
    }
}
