
pub trait BusDevice {
    fn read(&mut self, address: u16) -> u8 {
        (address >> 8) as u8
    }

    fn write(&mut self, _address: u16, _byte: u8) -> () {
        ()
    }

    fn address_range(&self) -> (u16, u16) {
        (0x0000, 0xffff)
    }

    fn address_in_range(&self, address: u16) -> bool {
        let (min, max) = self.address_range();
        address >= min && address <= max
    }
}

pub struct OpenBus {
}

impl BusDevice for OpenBus {
    fn read(&mut self, address: u16) -> u8 {
        eprintln!("Open Bus read at {:04x}.", address);
        (address >> 8) as u8
    }

    fn write(&mut self, address: u16, byte: u8) -> () {
        eprintln!("Open Bus write at {:04x} of {:02x}.", address, byte);
        ()
    }

    fn address_range(&self) -> (u16, u16) {
        (0x0000, 0xffff)
    }
}

pub struct RomController {
    logical_bank: u8,
    rom_wrap: usize,
    rom: Vec<u8>, // TODO: make this be a slice instead, maybe?
}

impl BusDevice for RomController {
    fn read(&mut self, address: u16) -> u8 {
        //~ let physical_address = address % 0x8000;
        let address_in_bank = address % 0x4000;
        let physical_bank = (address & (1 << 14)) >> 14;
        let logical_bank = self.logical_bank as u16 * physical_bank;
        let rom_address = logical_bank as usize * 0x4000 + address_in_bank as usize;
        self.rom[rom_address % self.rom_wrap]
    }


    fn write(&mut self, _address: u16, _byte: u8) -> () {
        eprintln!("writing to rom");
        ()
    }

    fn address_range(&self) -> (u16, u16) {
        (0x0000, 0x7fff)
    }
}

impl RomController {
    pub fn from_bytes(mut rom: Vec<u8>) -> Self {
        if rom.len() == 0 {
            panic!("Attempt to create zero-length ROM");
        }

        // If a ROM's size is not a power of two, we pad to the next power of two
        let size = rom.len();
        let mut rom_wrap: usize = 1;
        while rom_wrap < size {
            rom_wrap = rom_wrap << 1;
        }

        rom.resize(rom_wrap, 0xff);

        RomController {
            logical_bank: 0,
            rom_wrap,
            rom,
        }
    }
}

