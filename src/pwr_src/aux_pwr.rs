use embassy_sync::watch::DynReceiver;

pub struct AuxPwr<'a> {
    adc_recv: DynReceiver<'a, i16>,
}

impl<'a> AuxPwr<'a> {
    pub async fn new(adc_recv: DynReceiver<'a, i16>) -> Self {
        Self { adc_recv }
    }
    pub async fn get_voltage(&mut self) -> i16 {
        self.adc_recv.get().await
    }
}
