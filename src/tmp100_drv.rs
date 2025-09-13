use embassy_stm32::{i2c::I2c, mode::Async};


const TMP_RANGE_DEG: f32 = 128.;

pub enum Resolution {
    Res9Bit,
    Res10Bit,
    Res11Bit,
    Res12Bit
}
impl Resolution {
    pub fn get_v_range(&self) -> i32 {
        match self {
            Self::Res9Bit => 256,
            Self::Res10Bit => 512,
            Self::Res11Bit => 1024,
            Self::Res12Bit => 2048,
        }
    }
    fn set_reg_bits(&self, config_reg: &mut u8) {
        *config_reg &= !(0b11 << 5);
        *config_reg |= (match self {
            Self::Res9Bit => 0b00,
            Self::Res10Bit => 0b01,
            Self::Res11Bit => 0b10,
            Self::Res12Bit => 0b11,
        } << 5);
    }
}
pub enum Addr0State {
    Floating,
    High,
    Low
}
impl Addr0State {
    pub fn get_addr(&self) -> u8 {
        match self {
            Self::Floating => 0b1001001,
            Self::High => 0b1001010,
            Self::Low => 0b1001000,
        }
    }
}

pub struct Tmp100<'d> {
    interface: I2c<'d, Async>,
    resolution: Resolution,
    addr_0_state: Addr0State
}

impl<'d> Tmp100<'d> {
    pub fn new(interface: I2c<'d, Async>, resolution: Resolution, addr_0_state: Addr0State) -> Self {
        Self { interface, resolution, addr_0_state }
    }
}
