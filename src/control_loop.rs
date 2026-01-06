use crate::bitflags;
use embassy_futures::select::{Either, select};
use embassy_sync::channel::{DynamicReceiver, DynamicSender};
use embassy_time::Timer;
use south_common::telemetry::eps as tm;

use crate::EpsTMContainer;
use crate::pwr_src::d_flip_flop::{DFlipFlop, FlipFlopInput};
use crate::pwr_src::sink_ctrl::SinkCtrl;
use south_common::types::{Telecommand, EPSCommand, Sink};

const CONTROL_LOOP_TM_INTERVAL: u64 = 500;

bitflags! {
    pub struct Enabled: u8 {
        const BAT1      = 1 << 0;
        const BAT2      = 1 << 1;
        const AUXPWR    = 1 << 2;
        const ROCKETLST = 1 << 3;
        const SENSORUPP = 1 << 4;
        const ROCKETHD  = 1 << 5;
    }
}

// control loop task
#[embassy_executor::task]
pub async fn ctrl_thread(mut control_loop: ControlLoop<'static>) {
    loop { control_loop.run().await; }
}

pub struct ControlLoop<'d> {
    source_flip_flop: DFlipFlop<'d>,
    sink_ctrl: SinkCtrl<'d>,
    cmd_receiver: DynamicReceiver<'d, Telecommand>,
    tm_sender: DynamicSender<'d, EpsTMContainer>
}

impl<'d> ControlLoop<'d> {
    pub fn spawn(
        source_flip_flop: DFlipFlop<'d>,
        sink_ctrl: SinkCtrl<'d>,
        cmd_receiver: DynamicReceiver<'d, Telecommand>,
        tm_sender: DynamicSender<'d, EpsTMContainer>
    ) -> Self {
        Self {
            source_flip_flop,
            sink_ctrl,
            cmd_receiver,
            tm_sender
        }
    }

    async fn handle_cmd(&mut self, telecommand: Telecommand) {
        let Telecommand::EPS(telecommand) = telecommand else { return };
        match telecommand {
            EPSCommand::SetSource(state, time) => {
                let old_state = self.source_flip_flop.get_state();
                self.source_flip_flop.set(state).await;
                if let Some(time) = time {
                    // this is blocking and prevents tc during timeout.
                    // might be good to fix in the future
                    Timer::after_secs(time as u64).await;
                    self.source_flip_flop.set(old_state).await;
                }
            }
            EPSCommand::EnableSink(sink, time) => {
                if self.sink_ctrl.is_enabled(sink) {
                    return;
                }
                self.sink_ctrl.enable(sink);
                if let Some(time) = time {
                    // this is blocking and prevents tc during timeout.
                    // might be good to fix in the future
                    Timer::after_secs(time as u64).await;
                    self.sink_ctrl.disable(sink);
                }
            }
            EPSCommand::DisableSink(sink, time) => {
                if !self.sink_ctrl.is_enabled(sink) {
                    return;
                }
                self.sink_ctrl.disable(sink);
                if let Some(time) = time {
                    // this is blocking and prevents tc during timeout.
                    // might be good to fix in the future
                    Timer::after_secs(time as u64).await;
                    self.sink_ctrl.enable(sink);
                }
            }
        }
    }
    async fn send_state(&mut self) {
        let mut bitmap = Enabled::empty();
        bitmap.set(Enabled::BAT1, self.source_flip_flop.is_enabled(FlipFlopInput::Bat1));
        bitmap.set(Enabled::BAT2, self.source_flip_flop.is_enabled(FlipFlopInput::Bat2));
        bitmap.set(Enabled::AUXPWR, self.source_flip_flop.is_enabled(FlipFlopInput::AuxPwr));
        bitmap.set(Enabled::ROCKETLST, self.sink_ctrl.is_enabled(Sink::RocketLST));
        bitmap.set(Enabled::SENSORUPP, self.sink_ctrl.is_enabled(Sink::SensorUpper));
        bitmap.set(Enabled::ROCKETHD, self.sink_ctrl.is_enabled(Sink::RocketHD));

        let container = EpsTMContainer::new(&tm::EnableBitmap, &bitmap.bits()).unwrap();
        self.tm_sender.send(container).await;
    }

    pub async fn run(&mut self) {
        let event = select(
            self.cmd_receiver.receive(),
            Timer::after_millis(CONTROL_LOOP_TM_INTERVAL),
        ).await;
        match event {
            Either::First(cmd) => self.handle_cmd(cmd).await,
            Either::Second(()) => self.send_state().await,
        }
    }
}
