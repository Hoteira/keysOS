pub mod pmm;
pub mod vmm;
pub mod paging;
pub mod address;
pub mod mapper;
pub mod mmio;

pub fn init() {
    pmm::init();
    vmm::init();
}
