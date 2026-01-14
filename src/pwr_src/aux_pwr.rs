use embassy_sync::{channel::DynamicSender, watch::DynReceiver};

use embassy_time::Timer;
use south_common::telemetry::eps as tm;

use crate::EpsTMContainer;

// Aux pwr task
#[embassy_executor::task]
pub async fn aux_pwr_thread(mut aux_pwr: AuxPwr<'static>) {
    const AUX_LOOP_LEN_MS: u64 = 500;
    loop {
        aux_pwr.run().await;
        Timer::after_millis(AUX_LOOP_LEN_MS).await;
    }
}

pub struct AuxPwr<'a> {
    adc_recv: DynReceiver<'a, i16>,
    tm_sender: DynamicSender<'a, EpsTMContainer>,
}

impl<'a> AuxPwr<'a> {
    pub async fn new(
        adc_recv: DynReceiver<'a, i16>,
        tm_sender: DynamicSender<'a, EpsTMContainer>,
    ) -> Self {
        Self {
            adc_recv,
            tm_sender,
        }
    }
    async fn get_voltage(&mut self) -> i16 {
        self.adc_recv.get().await
    }
    pub async fn run(&mut self) {
        let container =
            EpsTMContainer::new(&tm::AuxPowerVoltage, &self.get_voltage().await).unwrap();
        self.tm_sender.send(container).await;
    }
}
