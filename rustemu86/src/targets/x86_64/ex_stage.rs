use x86_64::decoder::ExOpcode;
use x86_64::decoder::ExecuteInst;
use x86_64::decoder::ExecuteInstType;
use x86_64::isa::registers::Reg64Id;
use x86_64::isa::opcode::OperandSize;
use x86_64::CpuState;

pub enum WriteBack {
    Rip(u64),
    GeneralRegister(Reg64Id, u64),
    CpuState(CpuState),
    Store(u64, WriteBackData),
    Load(Reg64Id, u64),
    Return(u64),
}

pub enum WriteBackData {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),
}

pub fn execute(inst: &ExecuteInstType) -> Result<WriteBack, ()> {
    match inst.clone() {
        ExecuteInstType::ArithLogic(inst) => execute_arith_logic(inst),
        ExecuteInstType::Branch(inst) => execute_branch(inst),
        ExecuteInstType::LoadStore(inst) => execute_load_store(inst),
        ExecuteInstType::Privilege(inst) => execute_privilege(inst),
    }
}

fn execute_arith_logic(inst: ExecuteInst) -> Result<WriteBack, ()> {
    match inst.get_opcode() {
        ExOpcode::Inc => Ok(execute_inc(inst)),
        ExOpcode::Add => Ok(execute_add(inst)),
        ExOpcode::Mov => Ok(execute_mov(inst)),
        _ => Err(()),
    }
}

fn execute_inc(inst: ExecuteInst) -> WriteBack {
    let result = inst.get_op1() + 1;
    let dest = inst.get_dest();
    WriteBack::GeneralRegister(dest, result)
}

fn execute_add(inst: ExecuteInst) -> WriteBack {
    let result = inst.get_op1() + inst.get_op2();
    let dest = inst.get_dest();
    WriteBack::GeneralRegister(dest, result)
}

fn execute_mov(inst: ExecuteInst) -> WriteBack {
    let result = inst.get_op1();
    let dest = inst.get_dest();
    WriteBack::GeneralRegister(dest, result)
}

fn execute_branch(inst: ExecuteInst) -> Result<WriteBack, ()> {
    match inst.get_opcode() {
        ExOpcode::Jump => Ok(execute_jump(inst)),
        ExOpcode::Return => Ok(execute_return(inst)),
        _ => Err(()),
    }
}

fn execute_jump(inst: ExecuteInst) -> WriteBack {
    let result = inst.get_rip() + inst.get_op1();
    WriteBack::Rip(result)
}

fn execute_return(inst: ExecuteInst) -> WriteBack {
    let sp = inst.get_op1();
    WriteBack::Return(sp)
}

fn execute_load_store(inst: ExecuteInst) -> Result<WriteBack, ()> {
    match inst.get_opcode() {
        ExOpcode::Load => Ok(execute_load(inst)),
        ExOpcode::Store => Ok(execute_store(inst)),
        _ => Err(()),
    }
}

fn execute_load(inst: ExecuteInst) -> WriteBack {
    let addr = inst.get_op1();
    let dest = inst.get_dest();
    WriteBack::Load(dest, addr)
}

fn execute_store(inst: ExecuteInst) -> WriteBack {
    let addr = inst.get_op1();
    let data = inst.get_op2();
    let data = match inst.get_op_size() {
        OperandSize::Byte => WriteBackData::Byte(data as u8),
        OperandSize::Word => WriteBackData::Word(data as u16),
        OperandSize::DoubleWord => WriteBackData::DWord(data as u32),
        OperandSize::QuadWord => WriteBackData::QWord(data as u64),
    };
    WriteBack::Store(addr, data)
}

fn execute_privilege(inst: ExecuteInst) -> Result<WriteBack, ()> {
    match inst.get_opcode() {
        ExOpcode::Halt => Ok(WriteBack::CpuState(CpuState::Halt)),
        _ => Err(()),
    }
}
