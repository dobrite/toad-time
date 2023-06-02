#![no_std]
#![no_main]
#![deny(unused_must_use)]
#![feature(type_alias_impl_trait)]

use core::convert::Infallible;

use defmt_rtt as _;
use embassy_executor::{Executor, _export::StaticCell};
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    multicore::{spawn_core1, Stack},
    peripherals::{PIN_11, PIN_12, PIN_13, PIN_14, PIN_15, PIN_2, PIN_3, PIN_4, PIN_5},
    spi::{Config, Spi},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Delay, Duration, Ticker, Timer};
use embedded_hal::digital::v2::InputPin;
use embedded_hal_async::spi::ExclusiveDevice;
use heapless::Vec;
use panic_probe as _;
use rotary_encoder_embedded::{standard::StandardMode, Direction, RotaryEncoder};
use seq::{OutputConfig, OutputType, Seq};
use ssd1306_async::{prelude::*, Ssd1306};

use crate::{
    display::Display,
    screens::Screens,
    state::{Command, PlayStatus, Screen, State, StateChange},
};

static mut CORE1_STACK: Stack<65_536> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static COMMAND_CHANNEL: Channel<CriticalSectionRawMutex, Command, 8> = Channel::new();
static TICK_STATE_CHANNEL: Channel<CriticalSectionRawMutex, StateChange, 8> = Channel::new();
static DISPLAY_STATE_CHANNEL: Channel<CriticalSectionRawMutex, StateChange, 8> = Channel::new();

type Encoder = RotaryEncoder<StandardMode, Input<'static, PIN_14>, Input<'static, PIN_15>>;

mod display;
mod screens;
mod state;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    let oled_reset = p.PIN_20;
    let oled_dc = p.PIN_16;
    let oled_cs = p.PIN_17;
    let clk = p.PIN_18;
    let mosi = p.PIN_19;

    let spi = Spi::new_txonly(p.SPI0, clk, mosi, p.DMA_CH0, Config::default());

    let cs = Output::new(oled_cs, Level::Low);
    let device = ExclusiveDevice::new(spi, cs);

    let dc = Output::new(oled_dc, Level::Low);
    let interface = ssd1306_async::SPIInterface::new(device, dc);

    let display_ctx = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    let mut rst = Output::new(oled_reset, Level::Low);
    {
        use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
        Delay.delay_ms(1u8);
    }
    rst.set_high();

    let display = Display::new(display_ctx);

    let mut outputs = Vec::new();
    let mut gate_a_config = OutputConfig::new();
    gate_a_config.set_output_type(OutputType::Euclid);
    outputs.push(gate_a_config).ok();
    outputs.push(OutputConfig::new()).ok();
    outputs.push(OutputConfig::new()).ok();
    outputs.push(OutputConfig::new()).ok();

    let initial_state = State::new(outputs);
    let initial_state1 = initial_state.clone();
    let initial_state2 = initial_state.clone();

    let output_a = Output::new(p.PIN_2, Level::Low);
    let output_b = Output::new(p.PIN_3, Level::Low);
    let output_c = Output::new(p.PIN_4, Level::Low);
    let output_d = Output::new(p.PIN_5, Level::Low);
    let play_button = Input::new(p.PIN_11, Pull::Up);
    let page_button = Input::new(p.PIN_12, Pull::Up);
    let encoder_button = Input::new(p.PIN_13, Pull::Up);
    let encoder = {
        let rotary_dt = Input::new(p.PIN_14, Pull::Up);
        let rotary_clk = Input::new(p.PIN_15, Pull::Up);

        RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode()
    };

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| {
            let _ = spawner.spawn(core1_encoder_task(encoder));
            let _ = spawner.spawn(core1_display_task(initial_state, display));
            let _ = spawner.spawn(core1_encoder_button_task(encoder_button));
            let _ = spawner.spawn(core1_page_button_task(page_button));
            let _ = spawner.spawn(core1_play_button_task(play_button));
        });
    });

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        let _ = spawner.spawn(core0_state_task(initial_state1));
        let _ = spawner.spawn(core0_tick_task(
            initial_state2,
            output_a,
            output_b,
            output_c,
            output_d,
        ));
    });
}

#[embassy_executor::task]
async fn core0_state_task(mut state: State) {
    loop {
        let command = COMMAND_CHANNEL.recv().await;
        match state.handle_command(command) {
            StateChange::None => {}
            state_change => {
                state.handle_state_change(&state_change);
                let _ = TICK_STATE_CHANNEL.send(state_change).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn core0_tick_task(
    mut state: State,
    mut output_a: Output<'static, PIN_2>,
    mut output_b: Output<'static, PIN_3>,
    mut output_c: Output<'static, PIN_4>,
    mut output_d: Output<'static, PIN_5>,
) {
    let mut seq = Seq::new(120, state.outputs.clone());

    let tick_duration = seq.tick_duration_micros();
    let mut ticker = Ticker::every(Duration::from_micros(tick_duration));

    loop {
        let result = seq.tick();

        if result[0].on_change {
            output_a.toggle();
        }

        if result[1].on_change {
            output_b.toggle();
        }

        if result[2].on_change {
            output_c.toggle();
        }

        if result[3].on_change {
            output_d.toggle();
        }

        match state.current_screen {
            Screen::Home => {}
            Screen::Output(output, output_type) => match output_type {
                OutputType::Gate => {}
                OutputType::Euclid => {
                    let index_change = result[usize::from(output)].index_change;
                    if index_change {
                        let index = result[usize::from(output)].index;
                        let state_change = StateChange::Index(output, index);
                        let _ = DISPLAY_STATE_CHANNEL.send(state_change).await;
                    }
                }
            },
        };

        while let Ok(state_change) = TICK_STATE_CHANNEL.try_recv() {
            state.handle_state_change(&state_change);
            match state_change {
                StateChange::Rate(output, rate) => seq.set_rate(output.into(), rate),
                StateChange::Pwm(output, pwm) => seq.set_pwm(output.into(), pwm),
                StateChange::Prob(output, prob) => seq.set_prob(output.into(), prob),
                StateChange::Length(output, length) => seq.set_length(output.into(), length),
                StateChange::Density(output, density) => seq.set_density(output.into(), density),
                StateChange::OutputType(output, output_type) => {
                    seq.set_output_type(output.into(), output_type)
                }
                StateChange::PlayStatus(play_status) => match play_status {
                    PlayStatus::Playing => { /* TODO: pause */ }
                    PlayStatus::Paused => { /* TODO: reset then play */ }
                },
                StateChange::Bpm(bpm) => {
                    seq.set_bpm(bpm.0);
                    let tick_duration = seq.tick_duration_micros();
                    ticker = Ticker::every(Duration::from_micros(tick_duration));
                }
                StateChange::Index(..)
                | StateChange::NextElement(_)
                | StateChange::NextScreen(_)
                | StateChange::None
                | StateChange::Sync(_) => {}
            }
            let _ = DISPLAY_STATE_CHANNEL.send(state_change).await;
        }

        ticker.next().await
    }
}

#[embassy_executor::task]
async fn core1_encoder_button_task(encoder_button: Input<'static, PIN_13>) {
    debounced_button(encoder_button, Command::EncoderPress).await
}

#[embassy_executor::task]
async fn core1_page_button_task(page_button: Input<'static, PIN_12>) {
    debounced_button(page_button, Command::PagePress).await
}

#[embassy_executor::task]
async fn core1_play_button_task(play_button: Input<'static, PIN_11>) {
    debounced_button(play_button, Command::PlayPress).await
}

async fn debounced_button<B: InputPin>(button: B, command: Command)
where
    B: InputPin<Error = Infallible>,
{
    let mut armed = true;
    let button_update_duration = Duration::from_micros(50_000);

    loop {
        if armed && button.is_low().unwrap() {
            armed = false;
            let _ = COMMAND_CHANNEL.send(command).await;
        } else if !armed && button.is_high().unwrap() {
            armed = true;
        }

        Timer::after(button_update_duration).await
    }
}

#[embassy_executor::task]
async fn core1_encoder_task(mut encoder: Encoder) {
    loop {
        encoder.update();
        match encoder.direction() {
            Direction::Clockwise => {
                let _ = COMMAND_CHANNEL.send(Command::EncoderRight).await;
            }
            Direction::Anticlockwise => {
                let _ = COMMAND_CHANNEL.send(Command::EncoderLeft).await;
            }
            Direction::None => {}
        }

        Timer::after(Duration::from_micros(1111)).await
    }
}

#[embassy_executor::task]
async fn core1_display_task(mut state: State, mut display: Display) {
    display.init().await;
    let mut screens = Screens::new();
    screens.draw_home(&state, &mut display).await;

    loop {
        let state_change = DISPLAY_STATE_CHANNEL.recv().await;
        match state_change {
            StateChange::None => {}
            _ => {
                state.handle_state_change(&state_change);
                screens.draw(&state, &mut display).await;
            }
        }
    }
}
