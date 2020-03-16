extern crate alloc;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct PageTable
{
    page_entries: Vec<i8>,//页表项（类型i8）,页表项物理地址由后期计算得来，故不必存储
}

pub struct MemoryAllocater
{
    pages: [usize;(MEMORY_END-MEMORY_START)/PAGE_SIZE],//页的分配情况,0未分配，1分配
}

lazy_static!{
    static ref MEMORYALLOCATOR: Mutex<MemoryAllocater> = Mutex::new(MemoryAllocater{
        pages: [0;(MEMORY_END-MEMORY_START)/PAGE_SIZE],
    });
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
                page_table.page_entries.push(i as i8);//若该内存未分配，将它分配
                self.pages[i] = 1;
                page_num -= 1;
            }
            if(page_num == 0)
            {
                break;
            }
        }
        page_table
    }

    pub fn release(page_table: PageTable)
    {
        let v = page_table.page_entries;
        let len = v.len();
        for i in 0..len
        {
            v[i];
        }

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
