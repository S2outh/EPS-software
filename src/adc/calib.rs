use core::ptr::read_volatile;


const TS_CAL_1_REG: usize = 0x1FFF_75A8;
const TS_CAL_2_REG: usize = 0x1FFF_75CA;
const V_REFINT_REG: usize = 0x1FFF_75AA;

pub struct FactoryCalibratedValues {
    pub ts_cal_1: u16,
    pub ts_cal_2: u16,
    pub v_refint: u16,
}
impl FactoryCalibratedValues {
    pub fn new() -> Self {
        unsafe {
            let ts_cal_1 = read_volatile(TS_CAL_1_REG as *const u16);
            let ts_cal_2 = read_volatile(TS_CAL_2_REG as *const u16);
            let v_refint = read_volatile(V_REFINT_REG as *const u16);
            Self { ts_cal_1, ts_cal_2, v_refint }
        }
    }
}
