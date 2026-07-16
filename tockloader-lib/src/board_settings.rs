#[derive(Clone)]
pub struct BoardSettings {
    pub arch: Option<String>,
    pub flash_address: u64,
    pub start_address: u64,
    pub page_size: u64,
    pub ram_start_address: u64,
}

// TODO(eva-cosma): Does a default implementation make sense for this? Is a
// 'None' architechture a sane idea?
impl Default for BoardSettings {
    fn default() -> Self {
        Self {
            arch: None,
            flash_address: 0x00000, // this would be actually like -0x10000 
            start_address: 0x30000,
            page_size: 512,
            ram_start_address: 0x20000000,
        }
    }
}
