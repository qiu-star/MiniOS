use crate::context::InterruptFrame;
use riscv::register::{stvec, sscratch, sstatus};
use riscv::register::scause::{Trap, Interrupt, Exception};
use crate::clock_interrupt::{set_next_clockinterrupt, TICK};
global_asm!(include_str!("asm/trap.asm"));

pub fn init_interrupt()
{
    extern {
        fn __changecontext();
    }
    //保存CPU现场
    unsafe {
        sscratch::write(0);//给中断asm初始化（这块不是很明白（难道不应该区分内核态中断和用户态中断进行赋值吗
        sstatus::set_sie();//sie寄存器控制了所有内核态的中断，需要将其SSIE位设置为1,内核态才能接受软件中断
        stvec::write(__changecontext as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
fn handle_trap(context: &mut InterruptFrame)
{

    match context.scause.cause()
    {
        Trap::Exception(Exception::Breakpoint) => handle_breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimer) => handle_super_timer(),
        _ => panic!("unknown trap :{:?}!", context.scause.cause()),
    }
    //中断返回后CPU恢复到发生中断指令时的pc，如果不手动使CPU执行下一条指令，那么会一直发生中断
    // context.increase_sepc();
}

fn handle_super_timer()
{
    //设置下一次时钟中断发生的时间
    set_next_clockinterrupt();
    //将TICK加1
    unsafe {
        TICK += 1;
        if TICK % 100 == 0
        {
            println!("100 TICKS!");
        }
    }
    //时间中断直接返回CPU发生中断时的pc，故不需要执行context.increase_sepc()
}

fn handle_breakpoint() -> !
{
    panic!("BREAKEPOINT!");
}


