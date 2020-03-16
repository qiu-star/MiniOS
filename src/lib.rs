#![no_std]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(global_asm)]

#[macro_use]
mod io;

mod init;
mod lang_items;
mod sbi;
mod context;

mod trap;
mod clock_interrupt;

mod memory_alloc;
mod memory;

use buddy_system_allocator::LockedHeap;
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("DO NOTHING alloc_error_handler set by kernel");
}