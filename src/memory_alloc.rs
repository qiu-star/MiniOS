extern crate alloc;
use alloc::vec::Vec;
use riscv::addr::{Frame, PhysAddr};
use volatile::Volatile;

pub const MEMORY_START: usize = 0x8000_0000;
pub const MEMORY_END:usize = 0x8080_0000;
pub const PAGE_SIZE: usize = 4096;//4kb
pub const KERNEL_HEAP_SIZE: usize = 0x00a0_0000;

pub struct Memory
{
    constant: [[Volatile<u16>; PAGE_SIZE]; (MEMORY_END - MEMORY_START)/PAGE_SIZE],
}

pub struct MemoryWriter
{
    memory: &'static mut Memory,
}

impl MemoryWriter
{
    pub fn new() -> Self
    {
        let ret = MemoryWriter{
            memory: unsafe{&mut *(MEMORY_START as *mut Memory)},
        };
        ret
    }

    //将一个字符写入内存
    pub fn writeU16(&mut self, char: u16, addr: usize)
    {
        if addr > MEMORY_END
        {
            return;
        }
        let addr = addr - MEMORY_START;
        let page_offset: usize = addr % PAGE_SIZE;
        let page_index: usize = addr / PAGE_SIZE;
        println!("page_index: {} page_offset: {}", page_index, page_offset);
        println!("{}", self.memory.constant[page_index][page_offset].read());
        self.memory.constant[page_index][page_offset].write(char);
    }

    pub fn readU16(&mut self, addr: usize) -> u16
    {
        if addr > MEMORY_END
        {
            return 0;
        }
        let addr = addr - MEMORY_START;
        let page_offset: usize = addr % PAGE_SIZE;
        let page_index: usize = addr / PAGE_SIZE;
        self.memory.constant[page_index][page_offset].read()
    }

}


pub struct PageTable
{
    pub page_entries: Vec<(i8, Frame)>,//页表项（第一项为逻辑地址，第二项为物理地址）,页表项物理地址由后期计算得来，故不必存储
}

impl PageTable
{
    pub fn release(&mut self, index: usize)
    {
        self.page_entries.remove(index);
    }

    pub fn print_page_table(&mut self)
    {
        let len = self.page_entries.len();
        println!("Logistic Addr\tPysical Addr");
        for i in 0..len
        {
            let tmp = self.page_entries[i];
            println!("{}\t\t{:#x}", tmp.0, tmp.1.start_address().as_usize());
        }
    }
}

pub struct MemoryAllocater
{
    pub pages: [usize;(MEMORY_END-MEMORY_START)/PAGE_SIZE],//页的分配情况,0未分配，1分配
}

impl MemoryAllocater
{
    //输入需要分配的内存大小，输出页表
    pub fn alloc(&mut self, require_size: usize) -> PageTable
    {
        let mut page_table: PageTable = PageTable{
            page_entries: Vec::new(),
        };

        let mut page_num: usize = self.calc_page_num(require_size);
        let len = (MEMORY_END-MEMORY_START)/PAGE_SIZE;
        for i in 0..len
        {
            if self.pages[i] == 0
            {
                let addr = i * PAGE_SIZE + MEMORY_START;//物理地址
                page_table.page_entries.push((i as i8, Frame::of_addr(PhysAddr::new(addr))));//若该内存未分配，将它分配
                self.pages[i] = 1;
                page_num -= 1;
            }
            if page_num == 0
            {
                break;
            }
        }
        page_table
    }

    //释放PageTable中所有的页所占的内存
    pub fn release(&mut self, page_table: PageTable)
    {
        let v = page_table.page_entries;
        let len = v.len();
        for i in 0..len
        {
            self.release_index(v[i].0);
        }
    }

    //释放某一页所占的内存
    pub fn release_index(&mut self, page_loc: i8)
    {
        self.pages[page_loc as usize] = 0;
    }

    fn calc_page_num(&mut self, size: usize) -> usize
    {
        let mut num = size / PAGE_SIZE;
        if size % PAGE_SIZE != 0
        {
            num += 1;
            return num;
        }
        return num;
    }
}
