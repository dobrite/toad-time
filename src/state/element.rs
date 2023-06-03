use super::{ScreenState, State, StateChange, Updatable};

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
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].rate().next() {
                    Option::Some(rate) => StateChange::Rate(output, rate),
                    Option::None => StateChange::None,
                }
            }
            Element::Pwm => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].pwm().next() {
                    Option::Some(pwm) => StateChange::Pwm(output, pwm),
                    Option::None => StateChange::None,
                }
            }
            Element::Prob => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].prob().next() {
                    Option::Some(prob) => StateChange::Prob(output, prob),
                    Option::None => StateChange::None,
                }
            }
            Element::Length => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                let config = &state.outputs[usize::from(output)];
                match config.length().next() {
                    Option::Some(length) => StateChange::Length(output, length, config.density()),
                    Option::None => StateChange::None,
                }
            }
            Element::Density => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                let config = &state.outputs[usize::from(output)];
                match config.density().next() {
                    Option::Some(density) => StateChange::Density(output, config.length(), density),
                    Option::None => StateChange::None,
                }
            }
            Element::OutputType => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].output_type().next() {
                    Option::Some(output_type) => {
                        let mut config = state.outputs[usize::from(output)].clone();
                        config.set_output_type(output_type);
                        StateChange::OutputType(ScreenState::Output(output, config, Option::None))
                    }
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
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].rate().prev() {
                    Option::Some(rate) => StateChange::Rate(output, rate),
                    Option::None => StateChange::None,
                }
            }
            Element::Pwm => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].pwm().prev() {
                    Option::Some(pwm) => StateChange::Pwm(output, pwm),
                    Option::None => StateChange::None,
                }
            }
            Element::Prob => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].prob().prev() {
                    Option::Some(prob) => StateChange::Prob(output, prob),
                    Option::None => StateChange::None,
                }
            }
            Element::Length => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                let config = &state.outputs[usize::from(output)];
                match config.length().prev() {
                    Option::Some(length) => StateChange::Length(output, length, config.density()),
                    Option::None => StateChange::None,
                }
            }
            Element::Density => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                let config = &state.outputs[usize::from(output)];
                match config.density().prev() {
                    Option::Some(density) => StateChange::Density(output, config.length(), density),
                    Option::None => StateChange::None,
                }
            }
            Element::OutputType => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                match state.outputs[usize::from(output)].output_type().prev() {
                    Option::Some(output_type) => {
                        let mut config = state.outputs[usize::from(output)].clone();
                        config.set_output_type(output_type);
                        StateChange::OutputType(ScreenState::Output(output, config, Option::None))
                    }
                    Option::None => StateChange::None,
                }
            }
        }
    }
}
