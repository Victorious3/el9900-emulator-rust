pub trait Core {
    fn new() -> Self;
    /// Resets the cpu to its default state
    fn reset(&mut self, memory: &mut dyn Memory);
}

pub trait Package {
    /// Resets the microcontroller to its default state
    fn reset(&mut self);
}

pub trait Memory {
    fn read_byte(&mut self, addr: u32) -> u8;
    fn write_byte(&mut self, addr: u32, value: u8);

    fn read_word(&mut self, addr: u32) -> u16 {
        let low = self.read_byte(addr) as u16;
        let high = self.read_byte(addr + 1) as u16;
        return (high << 8) | low
    }

    fn write_word(&mut self, addr: u32, value: u16) {
        let low = (value & 0xFF) as u8;
        let high = (value >> 8) as u8;
        self.write_byte(addr, low);
        self.write_byte(addr + 1, high);
    }

    fn read_dword(&mut self, addr: u32) -> u32 {
        let low = self.read_word(addr) as u32;
        let high = self.read_word(addr + 2) as u32;
        return (high << 16) | low
    }

    fn write_dword(&mut self, addr: u32, value: u32) {
        let low = (value & 0xFFFF) as u16;
        let high = (value >> 16) as u16;
        self.write_word(addr, low);
        self.write_word(addr + 2, high);
    }
}

struct MemoryModule {
    start_addr: u32,
    size: u32,
    memory: Box<dyn Memory>
}

trait FixedSizeMemory<const size: usize>: Memory {}

pub struct Rom<const size: usize> {
    data: [u8; size]
}

impl <const size: usize> Rom<size> {
    pub fn new(data: [u8; size]) -> Self {
        Rom { data }
    }
}

impl<const size: usize> Memory for Rom<size> {
    fn read_byte(&mut self, addr: u32) -> u8 {
        self.data[addr as usize]
    }

    fn write_byte(&mut self, _addr: u32, _value: u8) {
        // ROM is read-only, do nothing
    }
}

impl<const size: usize> FixedSizeMemory<size> for Rom<size> {}

pub struct Ram<const size: usize> {
    data: [u8; size]
}

impl <const size: usize> Ram<size> {
    pub fn new() -> Self {
        Ram { data: [0; size] }
    }
}

impl<const size: usize> Memory for Ram<size> {
    fn read_byte(&mut self, addr: u32) -> u8 {
        self.data[addr as usize]
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        self.data[addr as usize] = value;
    }
}

impl<const size: usize> FixedSizeMemory<size> for Ram<size> {}

pub struct MMU {
    modules: Vec<MemoryModule>
}

impl MMU {
    pub fn new() -> Self {
        MMU { modules: Vec::new() }
    }

    pub fn add_module(&mut self, start_addr: u32, size: u32, memory: Box<dyn Memory>) {
        self.modules.push(MemoryModule { start_addr, size, memory });
    }

    pub fn add_fixed<const size: usize>(&mut self, start_addr: u32, memory: Box<dyn FixedSizeMemory<size>>) {
        self.add_module(start_addr, size as u32, memory);
    }

    pub fn read_byte(&mut self, addr: u32) -> u8 {
        for module in &mut self.modules {
            if addr >= module.start_addr && addr < module.start_addr + module.size {
                return module.memory.read_byte(addr - module.start_addr);
            }
        }
        0
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) {
        for module in &mut self.modules {
            if addr >= module.start_addr && addr < module.start_addr + module.size {
                return module.memory.write_byte(addr - module.start_addr, value);
            }
        }
    }
}