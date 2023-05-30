use super::{Screen, State, StateChange, Updatable};

#[derive(Clone, Copy)]
pub enum Element {
    Rate,
    Pwm,
    Prob,
    Length,
    Density,
    OutputType,
    Bpm,
    Sync,
}

impl Element {
    pub fn next(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm => state.bpm.next().into(),
            Element::Sync => state.sync.next().into(),
            Element::Rate => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].rate().next() {
                    Option::Some(rate) => StateChange::Rate(output, rate),
                    Option::None => StateChange::None,
                }
            }
            Element::Pwm => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].pwm().next() {
                    Option::Some(pwm) => StateChange::Pwm(output, pwm),
                    Option::None => StateChange::None,
                }
            }
            Element::Prob => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].prob().next() {
                    Option::Some(prob) => StateChange::Prob(output, prob),
                    Option::None => StateChange::None,
                }
            }
            Element::Length => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].length().next() {
                    Option::Some(length) => StateChange::Length(output, length),
                    Option::None => StateChange::None,
                }
            }
            Element::Density => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].density().next() {
                    Option::Some(density) => StateChange::Density(output, density),
                    Option::None => StateChange::None,
                }
            }
            Element::OutputType => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].output_type().next() {
                    Option::Some(output_type) => StateChange::OutputType(output, output_type),
                    Option::None => StateChange::None,
                }
            }
        }
    }

    pub fn prev(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm => state.bpm.prev().into(),
            Element::Sync => state.sync.prev().into(),
            Element::Rate => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].rate().prev() {
                    Option::Some(rate) => StateChange::Rate(output, rate),
                    Option::None => StateChange::None,
                }
            }
            Element::Pwm => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].pwm().prev() {
                    Option::Some(pwm) => StateChange::Pwm(output, pwm),
                    Option::None => StateChange::None,
                }
            }
            Element::Prob => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].prob().prev() {
                    Option::Some(prob) => StateChange::Prob(output, prob),
                    Option::None => StateChange::None,
                }
            }
            Element::Length => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].length().prev() {
                    Option::Some(length) => StateChange::Length(output, length),
                    Option::None => StateChange::None,
                }
            }
            Element::Density => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].density().prev() {
                    Option::Some(density) => StateChange::Density(output, density),
                    Option::None => StateChange::None,
                }
            }
            Element::OutputType => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                match state.outputs[&output].output_type().prev() {
                    Option::Some(output_type) => StateChange::OutputType(output, output_type),
                    Option::None => StateChange::None,
                }
            }
        }
    }
}
