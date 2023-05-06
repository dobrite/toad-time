use fugit::RateExtU32;

use super::*;
use crate::app::MicroSeconds;

pub struct State {
    pub bpm: Bpm,
    pub sync: Sync,
    pub play_status: PlayStatus,
    pub current: Element,
    pub gates: Gates,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let mut gates = Gates::new();
        gates.insert(Gate::A, GateState::new()).ok();
        gates.insert(Gate::B, GateState::new()).ok();
        gates.insert(Gate::C, GateState::new()).ok();
        gates.insert(Gate::D, GateState::new()).ok();

        Self {
            bpm: Bpm(120),
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current: Element::Bpm(Home),
            gates,
        }
    }

    pub fn gate_configs(&self) -> heapless::Vec<GateState, 4> {
        self.gates.values().copied().collect()
    }

    pub fn resolution(&self) -> u32 {
        PWM_PERCENT_INCREMENTS * MAX_MULT
    }

    pub fn tick_duration(&self) -> MicroSeconds {
        let bpm: f32 = self.bpm.0 as f32;
        let bps = bpm / SECONDS_IN_MINUTES;
        const MULTIPLYER: f32 = (PWM_PERCENT_INCREMENTS * MAX_MULT) as f32;
        let hertz: u32 = (bps * MULTIPLYER) as u32;

        hertz.Hz::<1, 1>().into_duration().into()
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        let current = self.current;

        match command {
            Command::EncoderRight => current.next(self),
            Command::EncoderLeft => current.prev(self),
            Command::EncoderPress => StateChange::NextElement(self.next_element()),
            Command::PagePress => StateChange::NextPage(self.next_page()),
            Command::PlayPress => self.toggle_play(),
        }
    }

    fn next_page(&mut self) -> Element {
        self.current = match self.current {
            Element::Bpm(_) => Element::Rate(Gate::A),
            Element::Sync(_) => Element::Rate(Gate::A),
            Element::Rate(Gate::A) => Element::Rate(Gate::B),
            Element::Pwm(Gate::A) => Element::Rate(Gate::B),
            Element::Prob(Gate::A) => Element::Rate(Gate::B),
            Element::Rate(Gate::B) => Element::Rate(Gate::C),
            Element::Pwm(Gate::B) => Element::Rate(Gate::C),
            Element::Prob(Gate::B) => Element::Rate(Gate::C),
            Element::Rate(Gate::C) => Element::Rate(Gate::D),
            Element::Pwm(Gate::C) => Element::Rate(Gate::D),
            Element::Prob(Gate::C) => Element::Rate(Gate::D),
            Element::Rate(Gate::D) => Element::Bpm(Home),
            Element::Pwm(Gate::D) => Element::Bpm(Home),
            Element::Prob(Gate::D) => Element::Bpm(Home),
        };

        self.current
    }

    fn next_element(&mut self) -> Element {
        self.current = match self.current {
            Element::Bpm(_) => Element::Sync(Home),
            Element::Sync(_) => Element::Bpm(Home),
            Element::Rate(gate) => Element::Pwm(gate),
            Element::Pwm(gate) => Element::Prob(gate),
            Element::Prob(gate) => Element::Rate(gate),
        };

        self.current
    }

    fn toggle_play(&mut self) -> StateChange {
        self.play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(self.play_status)
    }
}
