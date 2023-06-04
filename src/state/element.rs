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
            elem => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                let config = &mut state.outputs[usize::from(output)];
                match elem {
                    Element::Rate => config.rate().next().map_or(StateChange::None, |rate| {
                        config.set_rate(rate);
                        StateChange::Rate(output, config.output_type(), rate)
                    }),
                    Element::Pwm => config.pwm().next().map_or(StateChange::None, |pwm| {
                        config.set_pwm(pwm);
                        StateChange::Pwm(output, pwm)
                    }),
                    Element::Prob => config.prob().next().map_or(StateChange::None, |prob| {
                        config.set_prob(prob);
                        StateChange::Prob(output, prob)
                    }),
                    Element::Length => config.length().next().map_or(StateChange::None, |length| {
                        config.set_length(length);
                        StateChange::Length(output, length, config.density())
                    }),
                    Element::Density => {
                        config
                            .density()
                            .next()
                            .map_or(StateChange::None, |density| {
                                config.set_density(density);
                                StateChange::Density(output, config.length(), density)
                            })
                    }
                    Element::OutputType => {
                        config
                            .output_type()
                            .next()
                            .map_or(StateChange::None, |output_type| {
                                config.set_output_type(output_type);
                                StateChange::OutputType(ScreenState::Output(
                                    output,
                                    config.clone(),
                                    Option::None,
                                ))
                            })
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn prev(&self, state: &mut State) -> StateChange {
        match self {
            Element::Bpm => state.bpm.prev().into(),
            Element::Sync => state.sync.prev().into(),
            elem => {
                let output = match state.current_screen {
                    ScreenState::Home(..) => unreachable!(),
                    ScreenState::Output(output, ..) => output,
                };
                let config = &mut state.outputs[usize::from(output)];
                match elem {
                    Element::Rate => config.rate().prev().map_or(StateChange::None, |rate| {
                        config.set_rate(rate);
                        StateChange::Rate(output, config.output_type(), rate)
                    }),
                    Element::Pwm => config.pwm().prev().map_or(StateChange::None, |pwm| {
                        config.set_pwm(pwm);
                        StateChange::Pwm(output, pwm)
                    }),
                    Element::Prob => config.prob().prev().map_or(StateChange::None, |prob| {
                        config.set_prob(prob);
                        StateChange::Prob(output, prob)
                    }),
                    Element::Length => config.length().prev().map_or(StateChange::None, |length| {
                        config.set_length(length);
                        StateChange::Length(output, length, config.density())
                    }),
                    Element::Density => {
                        config
                            .density()
                            .prev()
                            .map_or(StateChange::None, |density| {
                                config.set_density(density);
                                StateChange::Density(output, config.length(), density)
                            })
                    }
                    Element::OutputType => {
                        config
                            .output_type()
                            .prev()
                            .map_or(StateChange::None, |output_type| {
                                config.set_output_type(output_type);
                                StateChange::OutputType(ScreenState::Output(
                                    output,
                                    config.clone(),
                                    Option::None,
                                ))
                            })
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
