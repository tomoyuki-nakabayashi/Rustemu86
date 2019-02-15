use crate::error::MemoryAccessError;
use crate::memory_access::{MemoryAccess, Result};

const TX: usize = 0x0;
const RX: usize = 0x4;
const TX_CTRL: usize = 0x8;
const RX_CTRL: usize = 0xc;
const DIV: usize = 0x18;

/// SiFive UART
pub struct SifiveUart {
    txctrl: u32,
    rxctrl: u32,
    baurate: u32,
}

impl SifiveUart {
    pub fn new() -> SifiveUart {
        SifiveUart {
            txctrl: 0,
            rxctrl: 0,
            baurate: 0,
        }
    }
}

impl MemoryAccess for SifiveUart {
    /// sifive uart must be accessed by u32.
    fn read_u8(&self, addr: usize) -> Result<u8> {
        Err( MemoryAccessError::InvalidAlignment { alignment: addr } )
    }

    fn read_u32(&self, addr: usize) -> Result<u32> {
        match addr {
            TX | RX => Ok(0),
            _ => Err( MemoryAccessError::DeviceNotMapped { addr } ),
        }
    }

    /// sifive uart must be accessed by u32.
    fn write_u8(&mut self, addr: usize, _data: u8) -> Result<()> {
        Err( MemoryAccessError::InvalidAlignment { alignment: addr } )
    }

    /// Assumption: `data` is safely casted into `u8` because the data is
    /// casted from `u8`.
    fn write_u32(&mut self, addr: usize, data: u32) -> Result<()> {
        match addr {
            TX => { print!("{}", data as u8 as char); Ok(()) }
            TX_CTRL => { self.txctrl = data; Ok(()) }
            RX_CTRL => { self.rxctrl = data; Ok(()) }
            DIV => { self.baurate = data; Ok(()) }
            _ => Err( MemoryAccessError::DeviceNotMapped { addr } ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write() {
        let mut sifive_uart = SifiveUart::new();

        let result = sifive_uart.write_u32(0, 'a' as u32);
        assert!(result.is_ok());

        // Can write only to TX Buffer.
        let result = sifive_uart.write_u32(4, 'a' as u32);
        assert!(result.is_err());
    }

    #[test]
    fn read() {
        let sifive_uart = SifiveUart::new();

        let result = sifive_uart.read_u32(4);
        assert!(result.is_ok());
    }
}
