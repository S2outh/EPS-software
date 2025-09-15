mod calib;

use defmt::info;
use embassy_stm32::{adc::{Adc, AdcChannel, AnyAdcChannel, RxDma, SampleTime}, peripherals::ADC1};
use calib::FactoryCalibratedValues;

const VREF_VAL: f32 = 3.;
const TS_1_VAL: f32 = 30.;
const TS_2_VAL: f32 = 130.;

const V_DIVIDER_MULT: f32 = (100. + 10.) / 10.;

const RAW_VALUE_RANGE: f32 = 4096.;

const BAT_1_CH_POS: usize = 0;
const VTEMP_CH_POS: usize = 1;
const VREF_CH_POS: usize = 2;

pub struct EPSAdc<'d, D: RxDma<ADC1>> {
    adc: Adc<'d, ADC1>,
    dma_channel: D,
    calib: FactoryCalibratedValues,
    temp_mult: f32,
    vref_numerator: f32,
    bat_1_channel: AnyAdcChannel<ADC1>,
    // bat_2_channel: AnyAdcChannel<ADC1>,
    // ext_channel: AnyAdcChannel<ADC1>,
    vtemp_channel: AnyAdcChannel<ADC1>,
    vref_channel: AnyAdcChannel<ADC1>,
}

impl<'d, D: RxDma<ADC1>> EPSAdc<'d, D> {
    pub fn new(mut adc: Adc<'d, ADC1>, dma_channel: D, bat_1_channel: AnyAdcChannel<ADC1>) -> Self {
        let calib = FactoryCalibratedValues::new();

        info!("ts_cal_1 {} ts_cal_2 {} v_refint {}", calib.ts_cal_1, calib.ts_cal_2, calib.v_refint);

        adc.set_resolution(embassy_stm32::adc::Resolution::BITS12);
        // 16x oversampling
        adc.set_oversampling_ratio(0x03);
        adc.set_oversampling_shift(0x04);
        adc.oversampling_enable(true);

        let vtemp_channel = adc.enable_temperature().degrade_adc();
        let vref_channel = adc.enable_vrefint().degrade_adc();
        let temp_mult = (TS_2_VAL - TS_1_VAL) / (calib.ts_cal_2 - calib.ts_cal_1) as f32;
        let vref_numerator = VREF_VAL * calib.v_refint as f32;

        Self { adc, dma_channel, calib, temp_mult, vref_numerator, bat_1_channel, vtemp_channel, vref_channel }
    }
    pub async fn measure(&mut self) {
        let mut measurements = [0u16; 3];
        // lots of testing to do here
        self.adc.read(
            &mut self.dma_channel,
            [
                (&mut self.bat_1_channel, SampleTime::CYCLES160_5),
                (&mut self.vtemp_channel, SampleTime::CYCLES160_5),
                (&mut self.vref_channel, SampleTime::CYCLES160_5),
            ]
            .into_iter(),
            &mut measurements
        ).await;

        let vref_rel = self.calib.v_refint as f32 / measurements[VREF_CH_POS] as f32;
        let vref = VREF_VAL * vref_rel;
        info!("vref: {}", vref);

        let temp = self.temp_mult * (vref_rel * measurements[VTEMP_CH_POS] as f32 - self.calib.ts_cal_1 as f32) + TS_1_VAL;
        info!("temp: {}", temp);
        info!("raw: {}", measurements[VTEMP_CH_POS]);
        
        let vbat = (vref / RAW_VALUE_RANGE) * measurements[BAT_1_CH_POS] as f32;
        info!("vbat: {}", vbat * V_DIVIDER_MULT);
    }
}
