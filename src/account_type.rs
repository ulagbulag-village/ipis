bitflags::bitflags! {
    pub struct AccountType: u8 {
        const IPIIS = 0b10000000;

        const IPDIS = 0b01000000;
        const IPNIS = 0b00100000;
        const IPQIS = 0b00010000;
        const IPSIS = 0b00001000;
        const IPWIS = 0b00000100;
    }
}
