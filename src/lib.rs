use crate::registers::Registers::*;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use nalgebra::Vector3;

pub mod registers;

#[derive(Debug)]
pub enum Gy271Error<E> {
    I2c(E),
    InvalidChipId(u8),
    DataNotReady,
}

pub struct Gy271<I> {
    i2c: I,
}

impl<I, E> Gy271<I>
where
    I: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I) -> Self {
        Gy271 { i2c }
    }

    pub fn init(&mut self) -> Result<(), Gy271Error<E>> {
        self.write_u8(SET_PERIOD_REG.addr(), 0x01)?;
        self.write_u8(CONTROL_REG_1.addr(), 0x1D)?;
        Ok(())
    }

    pub fn is_data_ready(&mut self) -> Result<(), Gy271Error<E>> {
        match self.read_u8(STATUS_REG.addr())? & 0x01 {
            1 => Ok(()),
            _ => Err(Gy271Error::DataNotReady),
        }
    }

    pub fn get_data(&mut self) -> Result<Vector3<i16>, Gy271Error<E>> {
        if let Ok(_) = self.is_data_ready() {
            let x_msb = self.read_u8(DATA_X_MSB.addr())?;
            let x_lsb = self.read_u8(DATA_X_LSB.addr())?;
            let x_data : i16 = i16::from(x_msb) << 8 | i16::from(x_lsb);

            let y_msb = self.read_u8(DATA_Y_MSB.addr())?;
            let y_lsb = self.read_u8(DATA_Y_LSB.addr())?;
            let y_data : i16 = i16::from(y_msb) << 8 | i16::from(y_lsb);

            let z_msb = self.read_u8(DATA_Z_MSB.addr())?;
            let z_lsb = self.read_u8(DATA_Z_LSB.addr())?;
            let z_data : i16 = i16::from(z_msb) << 8 | i16::from(z_lsb);
            Ok(Vector3::new(x_data, y_data, z_data))
        } else {
            Err(Gy271Error::DataNotReady)
        }
    }

    pub fn read_u8(&mut self, reg: u8) -> Result<u8, Gy271Error<E>> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c
            .write_read(SLAVE_ADDR.addr(), &[reg], &mut byte)
            .map_err(Gy271Error::I2c)?;
        Ok(byte[0])
    }

    pub fn write_u8(&mut self, reg: u8, byte: u8) -> Result<(), Gy271Error<E>> {
        self.i2c.write(SLAVE_ADDR.addr(), &[reg, byte]).map_err(Gy271Error::I2c)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nalgebra() {
        let mut v = Vector3::<f32>::new(1., 1., 1.);
        let o = v.clone();
        v *= 3.;
        assert_eq!(Vector3::<f32>::new(3., 3., 3.), v);
        v /= 3.;
        assert_eq!(o, v);
        v -= o;
        assert_eq!(Vector3::<f32>::new(0., 0., 0.), v);
    }
}
