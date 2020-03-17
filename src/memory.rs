use crate::memory_alloc::{MEMORY_END, MEMORY_START, PAGE_SIZE, KERNEL_HEAP_SIZE, MemoryAllocater, MemoryWriter};
use crate::HEAP_ALLOCATOR;
use spin::Mutex;
use lazy_static::lazy_static;

static mut MEMORYALLOC: Mutex<MemoryAllocater> = Mutex::new(MemoryAllocater{
    pages: [0; (MEMORY_END-MEMORY_START)/PAGE_SIZE],
});

lazy_static! {
    static ref MEMORYWRITER: Mutex<MemoryWriter> = Mutex::new(MemoryWriter::new());
}

pub fn init()
{
    use riscv::register::sstatus;
    unsafe {
        //允许在S模式下访问属于U模式的内存
        sstatus::set_sum();
    }
    init_heap();
}

fn init_heap()
{
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub fn alloc_test()
{
    // let mut lock = MEMORYWRITER.lock();
    // lock.writeU16('a' as u16, MEMORY_START+4);
    // // let tmp = lock.readU16(MEMORY_START+4);
    // // println!("{}", tmp);
    // drop(lock);

    let addr = MEMORY_START as *mut u8;
    unsafe {
        *addr.offset(4) = 1;
        *addr.offset(5) = 2;
    }

    unsafe {
        {
            let mut lock = MEMORYALLOC.lock();
            let mut tmp = lock.alloc(PAGE_SIZE * 2 + 4);
            tmp.print_page_table();
            lock.release_index(1);
            tmp.release(1);
            tmp.print_page_table();
            drop(lock);
        }
        {
            let mut lock = MEMORYALLOC.lock();
            let mut tmp = lock.alloc(PAGE_SIZE * 4);
            tmp.print_page_table();
            lock.release(tmp);
            drop(lock);
        }
    }
}

