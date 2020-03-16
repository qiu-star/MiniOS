
global_asm!(include_str!("boot/entry.asm")); // 引入 _start
use riscv::addr::Frame;

#[no_mangle]
pub fn rust_main() -> ! {
    crate::trap::init_interrupt();
    crate::clock_interrupt::init_clock_interrupt();
    println!("{}","------------------WELCOME TO MINIOS------------------");
    // unsafe {
    //     asm!("ebreak"::::"volatile");// put_string("h");
    // }
    loop{}
    // panic!("------------------end------------------")
}