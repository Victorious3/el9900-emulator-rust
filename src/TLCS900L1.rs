use std::mem;
use crate::core::*;

pub struct TLCS900L1 {
    /*  
		General purpose bank registers
		
		r           +3                +2                +1                +0
		----+-----------------+-----------------+-----------------+-----------------+
		00H |      QW0    (QWA|0)    QA0    <XWA|0>    RW0    (RWA|0)    RA0        |
		04H |      QB0    (QBC|0)    QC0    <XBC|0>    RB0    (RBC|0)    RC0        |  BANK 0
		08H |      QD0    (QDE|0)    QE0    <XDE|0>    RD0    (RDE|0)    RE0        |
		0CH |      QH0    (QHL|0)    QL0    <XHL|0>    RH0    (RHL|0)    RL0        |
		----+-----------------+-----------------+-----------------+-----------------+
		10H |      QW1    (QWA|1)    QA1    <XWA|1>    RW1    (RWA|1)    RA1        |
		14H |      QB1    (QBC|1)    QC1    <XBC|1>    RB1    (RBC|1)    RC1        |  BANK 1
		18H |      QD1    (QDE|1)    QE1    <XDE|1>    RD1    (RDE|1)    RE1        |
		1CH |      QH1    (QHL|1)    QL1    <XHL|1>    RH1    (RHL|1)    RL1        |
		----+-----------------+-----------------+-----------------+-----------------+
		20H |      QW2    (QWA|2)    QA2    <XWA|2>    RW2    (RWA|2)    RA2        |
		24H |      QB2    (QBC|2)    QC2    <XBC|2>    RB2    (RBC|2)    RC2        |  BANK 2
		28H |      QD2    (QDE|2)    QE2    <XDE|2>    RD2    (RDE|2)    RE2        |
		2CH |      QH2    (QHL|2)    QL2    <XHL|2>    RH2    (RHL|2)    RL2        |
		----+-----------------+-----------------+-----------------+-----------------+
		30H |      QW3    (QWA|3)    QA3    <XWA|3>    RW3    (RWA|3)    RA3        |
		34H |      QB3    (QBC|3)    QC3    <XBC|3>    RB3    (RBC|3)    RC3        |  BANK 3
		38H |      QD3    (QDE|3)    QE3    <XDE|3>    RD3    (RDE|3)    RE3        |
		3CH |      QH3    (QHL|3)    QL3    <XHL|3>    RH3    (RHL|3)    RL3        |
		----+-----------------+-----------------+-----------------+-----------------+

		Current bank selected by RFP
		Will point at the above mentioned bank registers

		----+-----------------+-----------------+-----------------+-----------------+
		D0H |      QW     (QWA| )    QA     <XWA| >      W      (W|A )   A          |
		D4H |      QB     (QBC| )    QC     <XBC| >      B      (B|C )   C          |
		D8H |      QD     (QDE| )    QE     <XDE| >      D      (D|E )   E          |
		DCH |      QH     (QHL| )    QL     <XHL| >      H      (H|L )   L          |
		----+-----------------+-----------------+-----------------+-----------------+

		Previous bank selected by (RFP + 7) % 8
		Will point at the above mentioned bank registers

		----+-----------------+-----------------+-----------------+-----------------+
		E0H |      QW'    (QWA|')    QA'    <XWA|'>      W'     (W|A')   A'         |
		E4H |      QB'    (QBC|')    QC'    <XBC|'>      B'     (B|C')   C'         |
		E8H |      QD'    (QDE|')    QE'    <XDE|'>      D'     (D|E')   E'         |
		ECH |      QH'    (QHL|')    QL'    <XHL|'>      H'     (H|L')   L'         |
		----+-----------------+-----------------+-----------------+-----------------+
	*/
    bank: [u8; 0x40],

	/*   
		General purpose registers & Stack Pointer
				
		r           +3                +2                +1                +0
		----+-----------------+-----------------+-----------------+-----------------+
		F0H |     QIXH    (QIX| )    QIXL   <XIX| >    IXH      (I|X )   IXL        |
		F4H |     QIYH    (QBC| )    QIYL   <XIY| >    IYH      (I|Y )   IYL        |
		F8H |     QIZH    (QDE| )    QIZL   <XIZ| >    IZH      (I|Z )   IZL        |
		FCH |     QSPH    (QHL| )    QSPL   <XSP| >    SPH      (S|P )   SPL        |
		----+-----------------+-----------------+-----------------+-----------------+
	*/
    general: [u8; 0x10],

    // Dedicated registers
    SR: u16,    // Status register
    FS: u8,     // Shadow flag register
    PC: u32,    // Program counter

    // Control registers
    
    // DMA 
	DMA_source: [u32; 4],
	DMA_dest: [u32; 4],
	DMA_counter: [u16; 4],
	DMA_mode: [u8; 4],

    // Interrupt nesting counter
    INTNEST: u16,
}

impl Core for TLCS900L1 {
	fn new() -> Self {
		unsafe { mem::zeroed() }
	}
	
	fn reset(&mut self, mem: &mut dyn Memory) {
		// SYSM IFF2 IFF1 IFF0 MAX RFP2 RFP1 RFP0
		// S Z 0 H 0 V N C 
		self.SR = 0b11111000_00000000;
		// Read reset vector
		self.PC = mem.read_dword(0x0FFFF00);
	}
}