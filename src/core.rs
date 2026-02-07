pub trait Core {
    fn new() -> Self;
    /// Resets the cpu to its default state
    fn reset(&mut self, mem: &mut MMU);
}

pub trait Package {
    /// Resets the microcontroller to its default state
    fn reset(&mut self);

    fn read_byte(&mut self, addr: u32) -> u8;
    fn write_byte(&mut self, addr: u32, value: u8);
}

pub trait Memory {
    fn read_byte(&mut self, addr: u32) -> u8;
    fn write_byte(&mut self, addr: u32, value: u8);
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