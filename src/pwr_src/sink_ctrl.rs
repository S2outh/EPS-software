use embassy_stm32::{gpio::{Level, Output, Pin, Speed}, Peri};
use defmt::Format;

#[repr(u8)]
#[derive(Format)]
pub enum Sink {
    Mainboard,
    RocketLST,
    RocketHD,
}

pub struct SinkCtrl<'d> {
    mb_enable: Output<'d>,
    lst_enable: Output<'d>,
    rhd_enable: Output<'d>,
}

impl<'d> SinkCtrl<'d> {
    pub fn new(
        mb_enable: Peri<'d, impl Pin>,
        lst_enable: Peri<'d, impl Pin>,
        rhd_enable: Peri<'d, impl Pin>,
    ) -> Self {
        let mb_enable = Output::new(mb_enable, Level::High, Speed::High);
        let lst_enable = Output::new(lst_enable, Level::High, Speed::High);
        let rhd_enable = Output::new(rhd_enable, Level::High, Speed::High);
        Self { mb_enable, lst_enable, rhd_enable }
    }
    fn get(&mut self, sink: Sink) -> &mut Output<'d> {
        match sink {
            Sink::Mainboard => &mut self.mb_enable,
            Sink::RocketLST => &mut self.lst_enable,
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
