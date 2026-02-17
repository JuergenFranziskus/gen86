use crate::{
    gp_regs::{RSize, Reg},
    operand::OSize,
};
use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mem<'a> {
    pub base: Option<Reg>,
    pub index: Option<(Reg, Scale)>,
    pub offset: i64,
    pub label: Option<&'a str>,
    pub size: Option<OSize>,
}
impl Mem<'static> {
    pub fn new() -> Self {
        Self {
            base: None,
            index: None,
            offset: 0,
            label: None,
            size: None,
        }
    }
}
impl<'a> Mem<'a> {
    pub fn based(mut self, base: Reg) -> Self {
        self.base = Some(base);
        self
    }
    pub fn indexed(self, index: Reg) -> Self {
        self.indexed_scaled(index, Scale::One)
    }
    pub fn indexed_scaled(mut self, index: Reg, scale: Scale) -> Self {
        self.index = Some((index, scale));
        self
    }
    pub fn labeled(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}
impl<'a> Add<Reg> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: Reg) -> Self::Output {
        self.based(rhs)
    }
}
impl<'a> Add<(Reg, Scale)> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: (Reg, Scale)) -> Self::Output {
        self.indexed_scaled(rhs.0, rhs.1)
    }
}
impl<'a> Add<i64> for Mem<'a> {
    type Output = Self;
    fn add(mut self, rhs: i64) -> Self::Output {
        self.offset = self.offset.wrapping_add(rhs);
        self
    }
}
impl<'a> Add<i32> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        self + rhs as i64
    }
}
impl<'a> Add<i16> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: i16) -> Self::Output {
        self + rhs as i64
    }
}
impl<'a> Add<i8> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: i8) -> Self::Output {
        self + rhs as i64
    }
}
impl<'a> Add<u64> for Mem<'a> {
    type Output = Self;
    fn add(mut self, rhs: u64) -> Self::Output {
        self.offset = self.offset.wrapping_add_unsigned(rhs);
        self
    }
}
impl<'a> Add<u32> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        self + rhs as u64
    }
}
impl<'a> Add<u16> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        self + rhs as u64
    }
}
impl<'a> Add<u8> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        self + rhs as u64
    }
}
impl<'a> Add<&'a str> for Mem<'a> {
    type Output = Self;
    fn add(mut self, rhs: &'a str) -> Self::Output {
        self.label = Some(rhs);
        self
    }
}
impl<'a> Add<&'a String> for Mem<'a> {
    type Output = Self;
    fn add(mut self, rhs: &'a String) -> Self::Output {
        self.label = Some(rhs);
        self
    }
}
impl<'a> Add<OSize> for Mem<'a> {
    type Output = Self;
    fn add(mut self, rhs: OSize) -> Self::Output {
        self.size = Some(rhs);
        self
    }
}
impl<'a> Add<RSize> for Mem<'a> {
    type Output = Self;
    fn add(self, rhs: RSize) -> Self::Output {
        self + Into::<OSize>::into(rhs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Scale {
    One,
    Two,
    Four,
    Eight,
}
