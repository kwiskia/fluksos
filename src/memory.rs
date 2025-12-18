use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr, VirtAddr, structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB
    }
};

unsafe fn active_page_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (page_table_frame, _) = Cr3::read();

    let phys_address = page_table_frame.start_address();
    let virt_address = physical_memory_offset + phys_address.as_u64();
    let page_table_ptr: *mut PageTable = virt_address.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub unsafe fn translate_address(
    addr: VirtAddr,
    phys_mem_offset: VirtAddr
) -> Option<PhysAddr> {
    translate_address_inner(addr, phys_mem_offset)
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("Mapping failed").flush();
}

fn translate_address_inner(
    addr: VirtAddr,
    phys_mem_offset: VirtAddr
) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (lvl4_page_table_frame, _) = Cr3::read();
    let mut frame = lvl4_page_table_frame;

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];

    for &index in &table_indexes {
        let virt_addr = phys_mem_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt_addr.as_ptr();
        let table = unsafe {&*table_ptr};

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge pages not supported"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let lvl4_table = active_page_table(phys_mem_offset);
        OffsetPageTable::new(lvl4_table, phys_mem_offset)
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map.iter()
            .filter(|region| region.region_type == MemoryRegionType::Usable)
            .map(|region| region.range.start_addr()..region.range.end_addr())
            .flat_map(|region| region.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
