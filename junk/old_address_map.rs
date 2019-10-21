/*
struct AddressSpace {
}

impl AddressSpace {
    pub fn new() -> AddressSpace {
        AddressSpace {
        }
    }

    fn device_at(&self, address: u16) -> BusDeviceKind {
        use BusDeviceKind::*;
        match address {
            0x0000 ... 0x3fff => Rom0,
            0x4000 ... 0x7fff => RomSwitch,
            0x8000 ... 0x9fff => Vram,
            0xa000 ... 0xbfff => Ram0,
            0xc000 ... 0xdfff => RamSwitch,
            //~ 0x0000 ... 0xffff => Io,
            0xe000 ... 0xffff => Other,
        }
    }

    fn read(&self, address: u16) -> u8 {
        use BusDeviceKind::*;
        match self.device_at(address) {
            Rom0 | RomSwitch => unimplemented!(),
            Vram => unimplemented!(),
            Ram0 | RamSwitch => unimplemented!(),
            //~ Io,
            Other => unimplemented!(),
        }
        unimplemented!()
    }

    fn write(&mut self, address: u16, data: u8) {
        unimplemented!()
    }
}

*/

/*
    (0x0000, 0x3fff) => Rom0,
    (0x4000, 0x7fff) => RomSwitch,
    (0x8000, 0x9fff) => Vram,
    (0xa000, 0xbfff) => Ram0,
    (0xc000, 0xdfff) => RamSwitch,
    //~ (0x0000, 0xffff) => Io,
    (0xe000, 0xffff) => Other,


enum BusDeviceKind {
    Rom0,
    RomSwitch,
    Vram,
    Ram0,
    RamSwitch,
    //~ Io,
    Other,
}

impl BusDeviceKind {
    fn address_range(&self) -> (u16, u16) {
        use BusDeviceKind::*;
        match self {
            Rom0 => (0x0000, 0x3fff),
            RomSwitch => (0x4000, 0x7fff),
            Vram => (0x8000, 0x9fff),
            Ram0 => (0xa000, 0xbfff),
            RamSwitch => (0xc000, 0xdfff),
            //~ Io => (0x0000, 0xffff),
            Other => (0xe000, 0xffff),
        }
    }
}
*/
