use core::result::Result::{self, Err, Ok};
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::sys::EspError;
use log::debug;

pub const AS7331_I2CADDR_DEFAULT: u8 = 0x74;

// Configuration State Registers
const AS7331_OSR: u8 = 0x00;
const AS7331_AGEN: u8 = 0x02;
const AS7331_CREG1: u8 = 0x06;
#[allow(unused)]
const AS7331_CREG2: u8 = 0x07;
const AS7331_CREG3: u8 = 0x08;
const AS7331_BREAK: u8 = 0x09;
#[allow(unused)]
const AS7331_EDGES: u8 = 0x0a;
#[allow(unused)]
const AS7331_OPTREG: u8 = 0x0b;

// Measurement State registers
const AS7331_STATUS: u8 = 0x00;
const AS7331_TEMP: u8 = 0x01;
const AS7331_MRES1: u8 = 0x02;
const AS7331_MRES2: u8 = 0x03;
const AS7331_MRES3: u8 = 0x04;
#[allow(unused)]
const AS7331_OUTCONV_L: u8 = 0x05;
#[allow(unused)]
const AS7331_OUTCONV_H: u8 = 0x06;

pub const AS7331_CREG1_GAIN_2048: u8 = 0x0;
pub const AS7331_CREG1_GAIN_1024: u8 = 0x1;
pub const AS7331_CREG1_GAIN_512: u8 = 0x2;
pub const AS7331_CREG1_GAIN_256: u8 = 0x3;
pub const AS7331_CREG1_GAIN_128: u8 = 0x4;
pub const AS7331_CREG1_GAIN_64: u8 = 0x5;
pub const AS7331_CREG1_GAIN_32: u8 = 0x6;
pub const AS7331_CREG1_GAIN_16: u8 = 0x7;
pub const AS7331_CREG1_GAIN_8: u8 = 0x8;
pub const AS7331_CREG1_GAIN_4: u8 = 0x9;
pub const AS7331_CREG1_GAIN_2: u8 = 0xa;
pub const AS7331_CREG1_GAIN_1: u8 = 0xb;

pub const AS7331_CREG1_TIME_1: u8 = 0;
pub const AS7331_CREG1_TIME_2: u8 = 1;
pub const AS7331_CREG1_TIME_4: u8 = 2;
pub const AS7331_CREG1_TIME_8: u8 = 3;
pub const AS7331_CREG1_TIME_16: u8 = 4;
pub const AS7331_CREG1_TIME_32: u8 = 5;
pub const AS7331_CREG1_TIME_64: u8 = 6;
pub const AS7331_CREG1_TIME_128: u8 = 7;
pub const AS7331_CREG1_TIME_256: u8 = 8;
pub const AS7331_CREG1_TIME_512: u8 = 9;
pub const AS7331_CREG1_TIME_1024: u8 = 10;
pub const AS7331_CREG1_TIME_2048: u8 = 11;
pub const AS7331_CREG1_TIME_4096: u8 = 12;
pub const AS7331_CREG1_TIME_8192: u8 = 13;
pub const AS7331_CREG1_TIME_16384: u8 = 14;

pub const AS7331_CREG3_MMODE_CONT: u8 = 0;
pub const AS7331_CREG3_MMODE_CMD: u8 = 1;
pub const AS7331_CREG3_MMODE_SYNS: u8 = 2;
pub const AS7331_CREG3_MMODE_SYND: u8 = 3;

pub const AS7331_CREG3_SB_OFF: u8 = 0;
pub const AS7331_CREG3_SB_ON: u8 = 1;

pub const AS7331_CREG3_RDYOD_PUSHPULL: u8 = 0;
pub const AS7331_CREG3_RDYOD_OPENDRAIN: u8 = 1;

pub const AS7331_CREG3_CCLK_1024: u8 = 0;
pub const AS7331_CREG3_CCLK_2048: u8 = 1;
pub const AS7331_CREG3_CCLK_4096: u8 = 2;
pub const AS7331_CREG3_CCLK_8192: u8 = 3;

pub const AS7331_OSR_SS_NO_MEASUREMENT: u8 = 0;
pub const AS7331_OSR_SS_MEASUREMENT: u8 = 1;

pub const AS7331_OSR_PD_OFF: u8 = 0;
pub const AS7331_OSR_PD_ON: u8 = 1;

pub const AS7331_OSR_SW_RES_OFF: u8 = 0;
pub const AS7331_OSR_SW_RES_ON: u8 = 1;

pub const AS7331_OSR_DOS_NOP: u8 = 0;
pub const AS7331_OSR_DOS_CONFIGURATION: u8 = 2;
pub const AS7331_OSR_DOS_MEASUREMENT: u8 = 3;

pub struct As7331<'a> {
    pub i2c: I2cDriver<'a>,
    pub addr: u8,
}

#[allow(dead_code)]
impl<'a> As7331<'a> {
    pub fn new(i2c: I2cDriver<'a>, addr: u8) -> Self {
        As7331 { i2c, addr }
    }

    pub fn destroy(self) -> I2cDriver<'a> {
        self.i2c
    }

    pub fn get_chip_id(&mut self) -> Result<u8, EspError> {
        let mut data = [0u8; 1];
        self.i2c_write_read_cmd(AS7331_AGEN, &mut data)?;
        Ok(data[0])
    }

    pub fn init(
        &mut self,
        mmode: u8,
        cclk: u8,
        sb: u8,
        break_time: u8,
        gain: u8,
        time: u8,
    ) -> Result<(), EspError> {
        self.i2c_write_cmd(AS7331_CREG1, gain << 4 | time)?;
        self.i2c_write_cmd(AS7331_CREG3, mmode << 6 | sb << 4 | cclk)?;
        self.i2c_write_cmd(AS7331_BREAK, break_time)
    }

    pub fn one_shot(&mut self) -> Result<(), EspError> {
        let mut data = [0u8; 1];
        self.i2c_write_read_cmd(AS7331_OSR, &mut data)?;
        self.i2c_write_cmd(AS7331_OSR, data[0] | 0x80)
    }

    pub fn get_status(&mut self) -> Result<[u8; 8], EspError> {
        let mut data = [0u8; 2];
        self.i2c_read_bytes(AS7331_STATUS, &mut data)?;
        Ok([
            (data[1] & 0x01) >> 0,
            (data[1] & 0x02) >> 1,
            (data[1] & 0x04) >> 2,
            (data[1] & 0x08) >> 3,
            (data[1] & 0x10) >> 4,
            (data[1] & 0x20) >> 5,
            (data[1] & 0x40) >> 6,
            (data[1] & 0x80) >> 7,
        ])
    }

    pub fn read_temp_data(&mut self) -> Result<u16, EspError> {
        let mut data = [0u8; 2];
        self.i2c_read_bytes(AS7331_TEMP, &mut data)?;
        Ok(((data[1] as u16) << 8) | (data[0] as u16))
    }

    pub fn read_uv_a_data(&mut self) -> Result<u16, EspError> {
        let mut data = [0u8; 2];
        self.i2c_read_bytes(AS7331_MRES1, &mut data)?;
        Ok(((data[1] as u16) << 8) | (data[0] as u16))
    }

    pub fn read_uv_b_data(&mut self) -> Result<u16, EspError> {
        let mut data = [0u8; 2];
        self.i2c_read_bytes(AS7331_MRES2, &mut data)?;
        Ok(((data[1] as u16) << 8) | (data[0] as u16))
    }

    pub fn read_uv_c_data(&mut self) -> Result<u16, EspError> {
        let mut data = [0u8; 2];
        self.i2c_read_bytes(AS7331_MRES3, &mut data)?;
        Ok(((data[1] as u16) << 8) | (data[0] as u16))
    }

    pub fn read_all_data(&mut self) -> Result<[u16; 4], EspError> {
        let mut raw_data = [0u8; 8];
        self.i2c_read_bytes(AS7331_TEMP, &mut raw_data)?;
        Ok([
            ((raw_data[1] as u16) << 8) | (raw_data[0] as u16),
            ((raw_data[3] as u16) << 8) | (raw_data[2] as u16),
            ((raw_data[5] as u16) << 8) | (raw_data[4] as u16),
            ((raw_data[7] as u16) << 8) | (raw_data[6] as u16),
        ])
    }

    fn i2c_write_read_cmd(&mut self, addr: u8, data: &mut [u8]) -> Result<(), EspError> {
        match self.i2c.write_read(self.addr, &[addr], data, BLOCK) {
            Ok(_) => debug!(
                "I2C_WRITE_READ - ADDR: 0x{:02X} - READ: 0x{:02X}",
                addr, data[0]
            ),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn i2c_read_bytes(&mut self, addr: u8, data: &mut [u8]) -> Result<(), EspError> {
        match self.i2c.write_read(self.addr, &[addr], data, BLOCK) {
            Ok(_) => debug!("I2C_READ_BYTES - ADDR: 0x{:02X} - DATA {:?}", addr, data),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn i2c_write_cmd(&mut self, addr: u8, cmd: u8) -> Result<(), EspError> {
        match self.i2c.write(self.addr, &[addr, cmd], BLOCK) {
            Ok(_) => debug!("I2C_WRITE - ADDR: 0x{:02X} - DATa: 0x{:02X}", addr, cmd),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    pub fn power_up(&mut self) -> Result<(), EspError> {
        let data = [0u8; 22];
        self.i2c_write_cmd(AS7331_OSR, data[0] | 0x40)
    }

    pub fn power_down(&mut self) -> Result<(), EspError> {
        let data = [0u8; 22];

        self.i2c_write_cmd(AS7331_OSR, data[0] & !0x40)
    }

    pub fn reset(&mut self) -> Result<(), EspError> {
        let data = [0u8; 22];

        self.i2c_write_cmd(AS7331_OSR, data[0] | 0x08)
    }

    pub fn set_configuration_mode(&mut self) -> Result<(), EspError> {
        let data = [0u8; 22];

        self.i2c_write_cmd(AS7331_OSR, data[0] | 0x02)
    }

    pub fn get_mode(&mut self) -> Result<[u8; 4], EspError> {
        let mut raw_data = [0u8; 2];
        self.i2c_read_bytes(AS7331_OSR, &mut raw_data)?;
        Ok([
            (raw_data[0] & 0x07),
            (raw_data[0] & 0x08) >> 3,
            (raw_data[0] & 0x40) >> 6,
            (raw_data[0] & 0x80) >> 7,
        ])
    }

    pub fn set_measurement_mode(&mut self) -> Result<(), EspError> {
        let data = [0u8; 22];
        /*match self.i2c_write_read_cmd(AS7331_OSR, &mut data) {
            Err(e) => return Err(e),
            _ => {}
        }*/

        self.i2c_write_cmd(AS7331_OSR, data[0] | 0x83)
    }
}
