
global_asm!(include_str!("boot/entry.asm")); // 引入 _start

#[no_mangle]
pub fn rust_main() -> ! {
    crate::interrupt::init_interrupt();
    println!("{}","------------------WELCOME TO MINIOS------------------");
    unsafe {
        asm!("ebreak"::::"volatile");// put_string("h");
    }
    panic!("------------------end------------------")
}