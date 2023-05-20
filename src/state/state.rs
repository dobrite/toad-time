use super::*;

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

    pub fn handle_state_change(&mut self, state_change: &StateChange) {
        match state_change {
            StateChange::Bpm(bpm) => self.bpm = *bpm,
            StateChange::Sync(sync) => self.sync = *sync,
            StateChange::Rate(gate, rate) => self.gates[gate].rate = *rate,
            StateChange::Pwm(gate, pwm) => self.gates[gate].pwm = *pwm,
            StateChange::Prob(gate, prob) => self.gates[gate].prob = *prob,
            StateChange::PlayStatus(play_status) => self.play_status = *play_status,
            StateChange::NextPage(element) => self.current = *element,
            StateChange::NextElement(element) => self.current = *element,
            StateChange::None => {}
        }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        let current = self.current;

        match command {
            Command::EncoderRight => current.next(self),
            Command::EncoderLeft => current.prev(self),
            Command::EncoderPress => self.next_element(),
            Command::PagePress => self.next_page(),
            Command::PlayPress => self.toggle_play(),
        }
    }

    fn next_page(&mut self) -> StateChange {
        let next_page = match self.current {
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

        StateChange::NextPage(next_page)
    }

    fn next_element(&mut self) -> StateChange {
        let next_element = match self.current {
            Element::Bpm(_) => Element::Sync(Home),
            Element::Sync(_) => Element::Bpm(Home),
            Element::Rate(gate) => Element::Pwm(gate),
            Element::Pwm(gate) => Element::Prob(gate),
            Element::Prob(gate) => Element::Rate(gate),
        };

        StateChange::NextElement(next_element)
    }

    fn toggle_play(&self) -> StateChange {
        let play_status = match self.play_status {
            PlayStatus::Playing => PlayStatus::Paused,
            PlayStatus::Paused => PlayStatus::Playing,
        };

        StateChange::PlayStatus(play_status)
    }
}
