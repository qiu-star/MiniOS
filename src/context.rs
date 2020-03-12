use riscv::register::{sstatus::Sstatus, scause::Scause};

#[derive(Debug)]
#[repr (C)]             //保证结构体的元素按照定义的顺序存储
pub struct InterruptFrame
{
    x: [usize;32],      //32个通用寄存器
    sstatus: Sstatus,   //系统状态
    stval: usize,       //中断跳转地址
    scause: Scause,     //中断或异常的原因
    sepc: usize,        //发生中断时的位置 / PC
}

impl InterruptFrame
{
    pub fn increase_sepc(self: &mut Self)
    {
        self.sepc += 4;//PC+4即下一条指令
    }
}