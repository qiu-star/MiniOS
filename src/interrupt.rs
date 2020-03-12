use crate::context::InterruptFrame;
use riscv::register::{stvec, sscratch};
global_asm!(include_str!("asm/interrupt.asm"));

pub fn init_interrupt()
{
    extern {
        fn __savecontext();
    }
    //保存CPU现场
    unsafe {
        sscratch::write(0);//给中断asm初始化（这块不是很明白（难道不应该区分内核态中断和用户态中断进行赋值吗
        stvec::write(__savecontext as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
fn handle_interrupt(context: &mut InterruptFrame)
{
    // println!("context {:#x?}", context);
    // panic!("interrupt");
    println!("interrupt!");
    //中断返回后CPU恢复到发生中断指令时的pc，如果不手动使CPU执行下一条指令，那么会一直发生中断
    context.increase_sepc();
}