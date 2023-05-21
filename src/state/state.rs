use super::*;

#[derive(Clone)]
pub struct State {
    pub bpm: Bpm,
    pub sync: Sync,
    pub play_status: PlayStatus,
    pub current_element: Element,
    pub outputs: Outputs,
}

impl Default for State {
    fn default() -> Self {
        let mut outputs = Outputs::new();
        outputs.insert(Output::A, OutputConfig::new()).ok();
        outputs.insert(Output::B, OutputConfig::new()).ok();
        outputs.insert(Output::C, OutputConfig::new()).ok();
        outputs.insert(Output::D, OutputConfig::new()).ok();

        Self::new(outputs)
    }
}

impl State {
    pub fn new(outputs: Outputs) -> Self {
        Self {
            bpm: Bpm(120),
            sync: Sync::Ext,
            play_status: PlayStatus::Playing,
            current_element: Element::Bpm(Home),
            outputs,
        }
    }

    pub fn handle_state_change(&mut self, state_change: &StateChange) {
        match state_change {
            StateChange::Bpm(bpm) => self.bpm = *bpm,
            StateChange::Sync(sync) => self.sync = *sync,
            StateChange::Rate(output, rate) => self.outputs[output].rate = *rate,
            StateChange::Pwm(output, pwm) => self.outputs[output].pwm = *pwm,
            StateChange::Prob(output, prob) => self.outputs[output].prob = *prob,
            StateChange::PlayStatus(play_status) => self.play_status = *play_status,
            StateChange::NextPage(element) => self.current_element = *element,
            StateChange::NextElement(element) => self.current_element = *element,
            StateChange::None => {}
        }
    }

    pub fn handle_command(&mut self, command: Command) -> StateChange {
        let current = self.current_element;

        match command {
            Command::EncoderRight => current.next(self),
            Command::EncoderLeft => current.prev(self),
            Command::EncoderPress => self.next_element(),
            Command::PagePress => self.next_page(),
            Command::PlayPress => self.toggle_play(),
        }
    }

    fn next_page(&mut self) -> StateChange {
        let next_page = match self.current_element {
            Element::Bpm(_) => Element::Rate(Output::A),
            Element::Sync(_) => Element::Rate(Output::A),
            Element::Rate(Output::A) => Element::Rate(Output::B),
            Element::Pwm(Output::A) => Element::Rate(Output::B),
            Element::Prob(Output::A) => Element::Rate(Output::B),
            Element::Rate(Output::B) => Element::Rate(Output::C),
            Element::Pwm(Output::B) => Element::Rate(Output::C),
            Element::Prob(Output::B) => Element::Rate(Output::C),
            Element::Rate(Output::C) => Element::Rate(Output::D),
            Element::Pwm(Output::C) => Element::Rate(Output::D),
            Element::Prob(Output::C) => Element::Rate(Output::D),
            Element::Rate(Output::D) => Element::Bpm(Home),
            Element::Pwm(Output::D) => Element::Bpm(Home),
            Element::Prob(Output::D) => Element::Bpm(Home),
        };

        StateChange::NextPage(next_page)
    }

    fn next_element(&mut self) -> StateChange {
        let next_element = match self.current_element {
            Element::Bpm(_) => Element::Sync(Home),
            Element::Sync(_) => Element::Bpm(Home),
            Element::Rate(output) => Element::Pwm(output),
            Element::Pwm(output) => Element::Prob(output),
            Element::Prob(output) => Element::Rate(output),
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
