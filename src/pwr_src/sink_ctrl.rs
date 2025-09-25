use embassy_stm32::{gpio::{Level, Output, Pin, Speed}, Peri};
use defmt::Format;

#[repr(u8)]
#[derive(Format, Clone, Copy)]
pub enum Sink {
    RocketLST,
    Mainboard,
    GPS,
    RocketHD,
}

pub struct SinkCtrl<'d> {
    lst_enable: Output<'d>,
    mb_enable: Output<'d>,
    gps_enable: Output<'d>,
    rhd_enable: Output<'d>,
}

impl<'d> SinkCtrl<'d> {
    pub fn new(
        lst_enable: Peri<'d, impl Pin>,
        mb_enable: Peri<'d, impl Pin>,
        gps_enable: Peri<'d, impl Pin>,
        rhd_enable: Peri<'d, impl Pin>,
    ) -> Self {
        let lst_enable = Output::new(lst_enable, Level::High, Speed::High);
        let mb_enable = Output::new(mb_enable, Level::High, Speed::High);
        let gps_enable = Output::new(gps_enable, Level::High, Speed::High);
        let rhd_enable = Output::new(rhd_enable, Level::High, Speed::High);
        Self { lst_enable, mb_enable, gps_enable, rhd_enable }
    }
    fn get(&mut self, sink: Sink) -> &mut Output<'d> {
        match sink {
            Sink::RocketLST => &mut self.lst_enable,
            Sink::Mainboard => &mut self.mb_enable,
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
    // pub fn enable_critical(&mut self) {
    //     self.enable(Sink::RocketLST);
    //     self.enable(Sink::Mainboard)
    // }
    // pub fn is_critical_enabled(&mut self) -> bool {
    //     self.is_enabled(Sink::RocketLST) &&
    //     self.is_enabled(Sink::Mainboard)
    // }
}
