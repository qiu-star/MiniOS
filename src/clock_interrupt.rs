use riscv::register::sie;
use riscv::register::{time, timeh};
use crate::sbi::set_timer;

static TIMEBASE: u64 = 100000;
pub static mut TICK: usize = 0;//TICK=n表示度过了n个TIMEBASE

pub fn init_clock_interrupt()
{
    unsafe {
        TICK = 0;
        sie::set_stimer();  //打开时钟中断
    }
    //设置下一次中断发生的时间
    set_next_clockinterrupt();
}

pub fn set_next_clockinterrupt()
{
    //当前时间+TIMEBASE=下次时钟中断发生的时间
    set_timer(get_current_time()+TIMEBASE);
}

fn get_current_time() -> u64
{
    loop{
        let high32 = timeh::read();
        let low32 = time::read();
        let tmp = timeh::read();
        //防止time::read()后高32位会发生变化
        //如果高32位发生变化了，则重新计算时间
        //否则返回结果
        //只关注高32位的值，却不关心低32位的原因是：高32位如果发生变化对结果的影响较大
        if tmp == high32
        {
            return (high32 as u64)<<32 | (low32 as u64);
        }
    }
}
