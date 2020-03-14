use riscv::register::{sstatus::Sstatus, scause::Scause};

#[derive(Debug)]
#[repr (C)]             //保证结构体的元素按照定义的顺序存储
pub struct InterruptFrame
{
    pub x: [usize;32],      //32个通用寄存器
    pub sstatus: Sstatus,   //系统状态
    pub stval: usize,
    pub scause: Scause,     //中断或异常的原因
    pub sepc: usize,        //发生中断时的位置 / PC
}

impl InterruptFrame
{
    pub fn increase_sepc(self: &mut Self)
    {
        self.sepc += 4;//PC+4即下一条指令
    }
}