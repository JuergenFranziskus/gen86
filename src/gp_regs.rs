use crate::mem::Mem;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reg {
    pub name: RName,
    pub size: RSize,
}
impl Reg {
    pub fn with_name(mut self, name: RName) -> Reg {
        self.name = name;
        self
    }
    pub fn with_size(mut self, size: RSize) -> Reg {
        self.size = size;
        self
    }

    pub fn mem(self) -> Mem<'static> {
        Mem::new() + self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RName {
    A,
    B,
    C,
    D,
    DI,
    SI,
    BP,
    SP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl RName {
    pub fn with_size(self, size: RSize) -> Reg {
        Reg { name: self, size }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RSize {
    Byte,
    Word,
    DWord,
    QWord,
}
impl RSize {
    pub fn with_name(self, name: RName) -> Reg {
        Reg { name, size: self }
    }
}

pub const AL: Reg = Reg {
    name: RName::A,
    size: RSize::Byte,
};
pub const AX: Reg = Reg {
    name: RName::A,
    size: RSize::Word,
};
pub const EAX: Reg = Reg {
    name: RName::A,
    size: RSize::DWord,
};
pub const RAX: Reg = Reg {
    name: RName::A,
    size: RSize::QWord,
};

pub const BL: Reg = Reg {
    name: RName::B,
    size: RSize::Byte,
};
pub const BX: Reg = Reg {
    name: RName::B,
    size: RSize::Word,
};
pub const EBX: Reg = Reg {
    name: RName::B,
    size: RSize::DWord,
};
pub const RBX: Reg = Reg {
    name: RName::B,
    size: RSize::QWord,
};

pub const CL: Reg = Reg {
    name: RName::C,
    size: RSize::Byte,
};
pub const CX: Reg = Reg {
    name: RName::C,
    size: RSize::Word,
};
pub const ECX: Reg = Reg {
    name: RName::C,
    size: RSize::DWord,
};
pub const RCX: Reg = Reg {
    name: RName::C,
    size: RSize::QWord,
};

pub const DL: Reg = Reg {
    name: RName::D,
    size: RSize::Byte,
};
pub const DX: Reg = Reg {
    name: RName::D,
    size: RSize::Word,
};
pub const EDX: Reg = Reg {
    name: RName::D,
    size: RSize::DWord,
};
pub const RDX: Reg = Reg {
    name: RName::D,
    size: RSize::QWord,
};

pub const DIL: Reg = Reg {
    name: RName::DI,
    size: RSize::Byte,
};
pub const DI: Reg = Reg {
    name: RName::DI,
    size: RSize::Word,
};
pub const EDI: Reg = Reg {
    name: RName::DI,
    size: RSize::DWord,
};
pub const RDI: Reg = Reg {
    name: RName::DI,
    size: RSize::QWord,
};

pub const SIL: Reg = Reg {
    name: RName::SI,
    size: RSize::Byte,
};
pub const SI: Reg = Reg {
    name: RName::SI,
    size: RSize::Word,
};
pub const ESI: Reg = Reg {
    name: RName::SI,
    size: RSize::DWord,
};
pub const RSI: Reg = Reg {
    name: RName::SI,
    size: RSize::QWord,
};

pub const BPL: Reg = Reg {
    name: RName::BP,
    size: RSize::Byte,
};
pub const BP: Reg = Reg {
    name: RName::BP,
    size: RSize::Word,
};
pub const EBP: Reg = Reg {
    name: RName::BP,
    size: RSize::DWord,
};
pub const RBP: Reg = Reg {
    name: RName::BP,
    size: RSize::QWord,
};

pub const SPL: Reg = Reg {
    name: RName::SP,
    size: RSize::Byte,
};
pub const SP: Reg = Reg {
    name: RName::SP,
    size: RSize::Word,
};
pub const ESP: Reg = Reg {
    name: RName::SP,
    size: RSize::DWord,
};
pub const RSP: Reg = Reg {
    name: RName::SP,
    size: RSize::QWord,
};

pub const R8B: Reg = Reg {
    name: RName::R8,
    size: RSize::Byte,
};
pub const R8W: Reg = Reg {
    name: RName::R8,
    size: RSize::Word,
};
pub const R8L: Reg = Reg {
    name: RName::R8,
    size: RSize::DWord,
};
pub const R8: Reg = Reg {
    name: RName::R8,
    size: RSize::QWord,
};

pub const R9B: Reg = Reg {
    name: RName::R9,
    size: RSize::Byte,
};
pub const R9W: Reg = Reg {
    name: RName::R9,
    size: RSize::Word,
};
pub const R9L: Reg = Reg {
    name: RName::R9,
    size: RSize::DWord,
};
pub const R9: Reg = Reg {
    name: RName::R9,
    size: RSize::QWord,
};

pub const R10B: Reg = Reg {
    name: RName::R10,
    size: RSize::Byte,
};
pub const R10W: Reg = Reg {
    name: RName::R10,
    size: RSize::Word,
};
pub const R10L: Reg = Reg {
    name: RName::R10,
    size: RSize::DWord,
};
pub const R10: Reg = Reg {
    name: RName::R10,
    size: RSize::QWord,
};

pub const R11B: Reg = Reg {
    name: RName::R11,
    size: RSize::Byte,
};
pub const R11W: Reg = Reg {
    name: RName::R11,
    size: RSize::Word,
};
pub const R11L: Reg = Reg {
    name: RName::R11,
    size: RSize::DWord,
};
pub const R11: Reg = Reg {
    name: RName::R11,
    size: RSize::QWord,
};

pub const R12B: Reg = Reg {
    name: RName::R12,
    size: RSize::Byte,
};
pub const R12W: Reg = Reg {
    name: RName::R12,
    size: RSize::Word,
};
pub const R12L: Reg = Reg {
    name: RName::R12,
    size: RSize::DWord,
};
pub const R12: Reg = Reg {
    name: RName::R12,
    size: RSize::QWord,
};

pub const R13B: Reg = Reg {
    name: RName::R13,
    size: RSize::Byte,
};
pub const R13W: Reg = Reg {
    name: RName::R13,
    size: RSize::Word,
};
pub const R13L: Reg = Reg {
    name: RName::R13,
    size: RSize::DWord,
};
pub const R13: Reg = Reg {
    name: RName::R13,
    size: RSize::QWord,
};

pub const R14B: Reg = Reg {
    name: RName::R14,
    size: RSize::Byte,
};
pub const R14W: Reg = Reg {
    name: RName::R14,
    size: RSize::Word,
};
pub const R14L: Reg = Reg {
    name: RName::R14,
    size: RSize::DWord,
};
pub const R14: Reg = Reg {
    name: RName::R14,
    size: RSize::QWord,
};

pub const R15B: Reg = Reg {
    name: RName::R15,
    size: RSize::Byte,
};
pub const R15W: Reg = Reg {
    name: RName::R15,
    size: RSize::Word,
};
pub const R15L: Reg = Reg {
    name: RName::R15,
    size: RSize::DWord,
};
pub const R15: Reg = Reg {
    name: RName::R15,
    size: RSize::QWord,
};
