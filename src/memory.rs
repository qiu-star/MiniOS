use crate::memory_alloc::{MEMORY_END, MEMORY_START, PAGE_SIZE, KERNEL_HEAP_SIZE, MemoryAllocater, PageTable};
use crate::HEAP_ALLOCATOR;
use lazy_static::lazy_static;
use spin::Mutex;

// lazy_static!{
//     static ref MEMORYALLOCATOR: Mutex<MemoryAllocater> = Mutex::new(MemoryAllocater{
//         pages: [0;(MEMORY_END-MEMORY_START)/PAGE_SIZE],
//     });
// }

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
    let mut MEMORYALLOC = MemoryAllocater{
        pages: [0; (MEMORY_END-MEMORY_START)/PAGE_SIZE],
    };
    let tmp: PageTable = MEMORYALLOC.alloc(PAGE_SIZE*2+4);
    print_page_table(tmp);
}

fn print_page_table(page_table: PageTable)
{
    let v = page_table.page_entries;
    let len = v.len();
    println!("Logistic Addr\tPysical Addr");
    for i in 0..len
    {
        let tmp = v[i];
        println!("{}\t\t{:#x}", tmp.0, tmp.1.start_address().as_usize());
    }
}