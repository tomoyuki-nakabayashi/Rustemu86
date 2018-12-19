use bitfield::bitfield;

pub enum InstrFormat {
    Base(BaseInstrFormat),
    Compressed(CompressedInstrFormt),
}

pub enum BaseInstrFormat {
    R_FORMAT(RTypeInstrFormat),
    I_FORMAT,
    S_FORMAT,
    B_FORMAT,
    U_FORMAT,
    J_FORMAT,
}

pub enum CompressedInstrFormat {
    CR_FORMAT,
    CI_FORMAT,
    CSS_FORMAT,
    CIW_FORMAT,
    CL_FORMAT,
    CS_FORMAT,
    CB_FORMAT,
    CJ_FORMAT,
}

bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct RTypeInstrFormat(u32);
    opcode: 6, 0;
    rd: 7, 11;
    funct3: 12, 14;
    rs1: 15, 19;
    rs2: 20, 24;
    funct7: 25, 31;
}