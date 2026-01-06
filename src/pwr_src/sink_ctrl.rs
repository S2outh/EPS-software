use embassy_stm32::{
    Peri,
    gpio::{Level, Output, Pin, Speed},
};
use south_common::types::Sink;

pub struct SinkCtrl<'d> {
    lst_enable: Output<'d>,
    sens_enable: Output<'d>,
    gps_enable: Output<'d>,
    rhd_enable: Output<'d>,
}

impl<'d> SinkCtrl<'d> {
    pub fn new(
        lst_enable: Peri<'d, impl Pin>,
        sens_enable: Peri<'d, impl Pin>,
        gps_enable: Peri<'d, impl Pin>,
        rhd_enable: Peri<'d, impl Pin>,
    ) -> Self {
        let lst_enable = Output::new(lst_enable, Level::High, Speed::High);
        let sens_enable = Output::new(sens_enable, Level::High, Speed::High);
        let gps_enable = Output::new(gps_enable, Level::High, Speed::High);
        let rhd_enable = Output::new(rhd_enable, Level::High, Speed::High);
        Self {
            lst_enable,
            sens_enable,
            gps_enable,
            rhd_enable,
        }
    }
    fn get(&mut self, sink: Sink) -> &mut Output<'d> {
        match sink {
            Sink::RocketLST => &mut self.lst_enable,
            Sink::SensorUpper => &mut self.sens_enable,
            Sink::GPS => &mut self.gps_enable,
            Sink::RocketHD => &mut self.rhd_enable,
        }
    }
    pub fn enable(&mut self, sink: Sink) {
        self.get(sink).set_high()
    }
    pub fn disable(&mut self, sink: Sink) {
        self.get(sink).set_low()
    }
    pub fn is_enabled(&mut self, sink: Sink) -> bool {
        self.get(sink).is_set_high()
    }
}
