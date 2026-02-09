use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::{Mutex, MutexGuard};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

use crate::register::Register;

macro_rules! i2c {
    ($i2c:expr, $method:ident, $($args:expr),*) => {
        {
            #[cfg(not(feature = "async"))]
            { $i2c.$method($($args),*) }
            #[cfg(feature = "async")]
            { $i2c.$method($($args),*).await }
        }
    }
}

pub(crate) struct Com<'a, I2C: I2c, RM: RawMutex> {
    pub(crate) address: u8,
    pub(crate) i2c_holder: I2cHolder<'a, I2C, RM>,
}

pub(crate) enum I2cHolder<'a, I2C: I2c, RM: RawMutex> {
    ThroughMutex(&'a Mutex<RM, I2C>),
    Direct(&'a mut I2C),
}

pub(crate) enum ActiveI2c<'a, I2C: I2c, RM: RawMutex> {
    Guarded(MutexGuard<'a, RM, I2C>),
    Borrowed(&'a mut I2C),
}

impl<'a, I2C: I2c, RM: RawMutex> core::ops::Deref for ActiveI2c<'a, I2C, RM> {
    type Target = I2C;
    fn deref(&self) -> &Self::Target {
        match self {
            ActiveI2c::Guarded(g) => &**g,
            ActiveI2c::Borrowed(r) => r,
        }
    }
}

impl<'a, I2C: I2c, RM: RawMutex> core::ops::DerefMut for ActiveI2c<'a, I2C, RM> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ActiveI2c::Guarded(g) => &mut **g,
            ActiveI2c::Borrowed(r) => r,
        }
    }
}

impl<'a, I2C, RM: RawMutex, E> Com<'a, I2C, RM>
where
    I2C: I2c<Error = E>,
    RM: RawMutex,
{
    pub(crate) async fn lock_i2c(&mut self) -> ActiveI2c<'_, I2C, RM> {
        match &mut self.i2c_holder {
            I2cHolder::ThroughMutex(mutex) => ActiveI2c::Guarded(mutex.lock().await),
            I2cHolder::Direct(i2c) => ActiveI2c::Borrowed(i2c),
        }
    }

    pub(crate) async fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let address = self.address;
        let mut data: [u8; 1] = [0];
        // FIXME:
        //  * device address is not a const
        //  * register address is u16
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read, address, &[reg as u8], &mut data)?;
        Ok(data[0])
    }

    pub(crate) async fn read_byte(&mut self, reg: u8) -> Result<u8, E> {
        let address = self.address;
        let mut data: [u8; 1] = [0];
        // FIXME:
        //  * device address is not a const
        //  * register address is u16
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read, address, &[reg], &mut data)?;
        Ok(data[0])
    }

    pub(crate) async fn read_6bytes(&mut self, reg: Register) -> Result<[u8; 6], E> {
        let mut ret: [u8; 6] = Default::default();
        self.read_registers(reg, &mut ret).await?;
        Ok(ret)
    }

    pub(crate) async fn read_registers(
        &mut self,
        reg: Register,
        buffer: &mut [u8],
    ) -> Result<(), E> {
        let address = self.address;
        // const I2C_AUTO_INCREMENT: u8 = 1 << 7;
        const I2C_AUTO_INCREMENT: u8 = 0;
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read,
            address,
            &[(reg as u8) | I2C_AUTO_INCREMENT],
            buffer
        )?;
        Ok(())
    }

    pub(crate) async fn read_16bit(&mut self, reg: Register) -> Result<u16, E> {
        let mut buffer: [u8; 2] = [0, 0];
        self.read_registers(reg, &mut buffer).await?;
        Ok(((buffer[0] as u16) << 8) + (buffer[1] as u16))
    }

    pub(crate) async fn write_byte(&mut self, reg: u8, byte: u8) -> Result<(), E> {
        let address = self.address;
        let mut buffer = [0];
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read, address, &[reg, byte], &mut buffer)?;
        Ok(())
    }

    pub(crate) async fn write_register(&mut self, reg: Register, byte: u8) -> Result<(), E> {
        let address = self.address;
        let mut buffer = [0];
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read, address, &[reg as u8, byte], &mut buffer)?;
        Ok(())
    }

    pub(crate) async fn write_6bytes(&mut self, reg: Register, bytes: [u8; 6]) -> Result<(), E> {
        let address = self.address;
        let mut buf: [u8; 6] = [0, 0, 0, 0, 0, 0];
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read,
            address,
            &[
                reg as u8, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
                bytes[5],
            ],
            &mut buf
        )?;
        Ok(())
    }

    pub(crate) async fn write_16bit(&mut self, reg: Register, word: u16) -> Result<(), E> {
        let address = self.address;
        let mut buffer = [0];
        let msb = (word >> 8) as u8;
        let lsb = (word & 0xFF) as u8;
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read, address, &[reg as u8, msb, lsb], &mut buffer)?;
        Ok(())
    }

    pub(crate) async fn write_32bit(&mut self, reg: Register, word: u32) -> Result<(), E> {
        let address = self.address;
        let mut buffer = [0];
        let v1 = (word & 0xFF) as u8;
        let v2 = ((word >> 8) & 0xFF) as u8;
        let v3 = ((word >> 16) & 0xFF) as u8;
        let v4 = ((word >> 24) & 0xFF) as u8;
        let mut i2c = self.lock_i2c().await;
        i2c!(i2c, write_read,
            address,
            &[reg as u8, v4, v3, v2, v1],
            &mut buffer
        )?;
        Ok(())
    }
}
