use super::{Screen, ScreenState, State, StateChange, Updatable};

#[derive(Clone)]
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
    pub fn next(&self, state: &mut State) -> Option<StateChange> {
        match self {
            Element::Bpm => state.bpm.next().map(|bpm| {
                state.bpm = bpm;
                StateChange::Bpm(bpm)
            }),
            Element::Sync => state.sync.next().map(|sync| {
                state.sync = sync;
                StateChange::Sync(sync)
            }),
            elem => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                let config = &mut state.outputs[usize::from(output)];
                match elem {
                    Element::Rate => config.rate().next().map(|rate| {
                        config.set_rate(rate);
                        StateChange::Rate(output, config.output_type(), rate)
                    }),
                    Element::Pwm => config.pwm().next().map(|pwm| {
                        config.set_pwm(pwm);
                        StateChange::Pwm(output, pwm)
                    }),
                    Element::Prob => config.prob().next().map(|prob| {
                        config.set_prob(prob);
                        StateChange::Prob(output, prob)
                    }),
                    Element::Length => config.length().next().map(|length| {
                        config.set_length(length);
                        StateChange::Length(output, length, config.density())
                    }),
                    Element::Density => config.density().next().map(|density| {
                        config.set_density(density);
                        StateChange::Density(output, config.length(), density)
                    }),
                    Element::OutputType => config.output_type().next().map(|output_type| {
                        config.set_output_type(output_type);
                        StateChange::OutputType(ScreenState::new_output(
                            output,
                            config.clone(),
                            Option::None,
                        ))
                    }),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn prev(&self, state: &mut State) -> Option<StateChange> {
        match self {
            Element::Bpm => state.bpm.prev().map(|bpm| {
                state.bpm = bpm;
                StateChange::Bpm(bpm)
            }),
            Element::Sync => state.sync.prev().map(|sync| {
                state.sync = sync;
                StateChange::Sync(sync)
            }),
            elem => {
                let output = match state.current_screen {
                    Screen::Home => unreachable!(),
                    Screen::Output(output, _) => output,
                };
                let config = &mut state.outputs[usize::from(output)];
                match elem {
                    Element::Rate => config.rate().prev().map(|rate| {
                        config.set_rate(rate);
                        StateChange::Rate(output, config.output_type(), rate)
                    }),
                    Element::Pwm => config.pwm().prev().map(|pwm| {
                        config.set_pwm(pwm);
                        StateChange::Pwm(output, pwm)
                    }),
                    Element::Prob => config.prob().prev().map(|prob| {
                        config.set_prob(prob);
                        StateChange::Prob(output, prob)
                    }),
                    Element::Length => config.length().prev().map(|length| {
                        config.set_length(length);
                        StateChange::Length(output, length, config.density())
                    }),
                    Element::Density => config.density().prev().map(|density| {
                        config.set_density(density);
                        StateChange::Density(output, config.length(), density)
                    }),
                    Element::OutputType => config.output_type().prev().map(|output_type| {
                        config.set_output_type(output_type);
                        StateChange::OutputType(ScreenState::new_output(
                            output,
                            config.clone(),
                            Option::None,
                        ))
                    }),
                    _ => unreachable!(),
                }
            }
        }
    }
}
