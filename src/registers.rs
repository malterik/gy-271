#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Registers {
    SLAVE_ADDR = 0x0D,
    DATA_X_LSB = 0x00,
    DATA_X_MSB = 0x01,
    DATA_Y_LSB = 0x02,
    DATA_Y_MSB = 0x03,
    DATA_Z_LSB = 0x04,
    DATA_Z_MSB = 0x05,
    STATUS_REG = 0x06,
    TEMP_DATA_LSB = 0x07,
    TEMP_DATA_MSB = 0x08,
    CONTROL_REG_1 = 0x09,
    CONTROL_REG_2 = 0x0A,
    SET_PERIOD_REG = 0x0B,
}

impl Registers {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}
