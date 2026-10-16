
use crate::core::*;
use crate::TLCS900L1::TLCS900L1;

struct TMP91C016 {
    core: TLCS900L1,
    mmu: MMU
}

impl TMP91C016 {
    pub fn new() -> Self {
        TMP91C016 {
            core: TLCS900L1::new(),
            mmu: MMU::new()
        }
    }
}

impl Memory for TMP91C016 {
    fn read_byte(&mut self, addr: u32) -> u8 {
        return self.mmu.read_byte(addr)
    }

    fn write_byte(&mut self, addr: u32, value: u8) {
        self.mmu.write_byte(addr, value);
    }
}

impl Package for TMP91C016 {
    fn reset(&mut self) {
        self.core.reset(self);
    }
}