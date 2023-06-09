#![no_std]
#![no_main]
#![deny(unused_must_use)]
#![feature(type_alias_impl_trait)]

use core::convert::Infallible;

use defmt_rtt as _;
use embassy_executor::{Executor, _export::StaticCell};
use embassy_rp::{
    gpio::{AnyPin, Input, Level, Output as EmbassyOutput, Pin, Pull},
    multicore::{spawn_core1, Stack},
    peripherals::{PIN_11, PIN_12, PIN_13, PIN_14, PIN_15},
    spi::{Config, Spi},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Delay, Duration, Ticker, Timer};
use embedded_hal::digital::v2::InputPin;
use embedded_hal_async::spi::ExclusiveDevice;
use heapless::Vec;
use panic_probe as _;
use rotary_encoder_embedded::{standard::StandardMode, Direction, RotaryEncoder};
use seq::{Frac, OutputConfig, OutputType, Rate, Seq};
use ssd1306_async::{prelude::*, Ssd1306};

use crate::{
    display::Display,
    screens::Screens,
    state::{Command, Output, ScreenState, State, StateChange},
    state_memo::StateMemo,
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
mod state_memo;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    let oled_reset = p.PIN_20;
    let oled_dc = p.PIN_16;
    let oled_cs = p.PIN_17;
    let clk = p.PIN_18;
    let mosi = p.PIN_19;

    let spi = Spi::new_txonly(p.SPI0, clk, mosi, p.DMA_CH0, Config::default());

    let cs = EmbassyOutput::new(oled_cs, Level::Low);
    let device = ExclusiveDevice::new(spi, cs);

    let dc = EmbassyOutput::new(oled_dc, Level::Low);
    let interface = ssd1306_async::SPIInterface::new(device, dc);

    let display_ctx = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    let mut rst = EmbassyOutput::new(oled_reset, Level::Low);
    {
        use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
        Delay.delay_ms(1u8);
    }
    rst.set_high();

    let display = Display::new(display_ctx);

    let mut outputs = Vec::new();

    let mut gate_a_config = OutputConfig::new();
    gate_a_config.set_output_type(OutputType::Euclid);
    gate_a_config.set_rate(Rate::Mult(16, Frac::Zero));
    outputs.push(gate_a_config).ok();

    let gate_b_config = OutputConfig::new();
    outputs.push(gate_b_config).ok();

    let gate_c_config = OutputConfig::new();
    outputs.push(gate_c_config).ok();

    let gate_d_config = OutputConfig::new();
    outputs.push(gate_d_config).ok();

    let initial_state = State::new(outputs);
    let initial_state1 = initial_state.clone();

    let outputs = {
        let mut outputs: Vec<EmbassyOutput<'static, AnyPin>, 4> = Vec::new();
        let output_a = EmbassyOutput::new(p.PIN_2.degrade(), Level::Low);
        outputs.push(output_a).ok();
        let output_b = EmbassyOutput::new(p.PIN_3.degrade(), Level::Low);
        outputs.push(output_b).ok();
        let output_c = EmbassyOutput::new(p.PIN_4.degrade(), Level::Low);
        outputs.push(output_c).ok();
        let output_d = EmbassyOutput::new(p.PIN_5.degrade(), Level::Low);
        outputs.push(output_d).ok();
        outputs
    };

    let play_button = Input::new(p.PIN_11, Pull::Up);
    let page_button = Input::new(p.PIN_12, Pull::Up);
    let encoder_button = Input::new(p.PIN_13, Pull::Up);
    let encoder = {
        let rotary_dt = Input::new(p.PIN_14, Pull::Up);
        let rotary_clk = Input::new(p.PIN_15, Pull::Up);

        RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode()
    };

    let seq = Seq::new(initial_state.bpm.0, initial_state.outputs.clone());
    let memo = StateMemo::new(initial_state.current_screen.clone());

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
        let _ = spawner.spawn(core0_tick_task(memo, seq, outputs));
    });
}

#[embassy_executor::task]
async fn core0_state_task(mut state: State) {
    loop {
        let command = COMMAND_CHANNEL.recv().await;
        if let Some(state_change) = state.handle_command(command) {
            TICK_STATE_CHANNEL.send(state_change).await;
        }
    }
}

#[embassy_executor::task]
async fn core0_tick_task(
    mut memo: StateMemo,
    mut seq: Seq,
    mut outputs: Vec<EmbassyOutput<'static, AnyPin>, 4>,
) {
    let tick_duration = seq.tick_duration_micros();
    let mut ticker = Ticker::every(Duration::from_micros(tick_duration));
    let mut state_changes: Vec<StateChange, 4> = Vec::new();

    loop {
        seq.tick();
        outputs.iter_mut().enumerate().for_each(|(idx, output)| {
            if seq.get_on_change(idx) {
                output.toggle()
            };
            let current_output = Output::into_output(idx);
            if seq.get_index_change(idx) && memo.current_screen.is_euclid(current_output) {
                let state_change = StateChange::Index(current_output, seq.get_index(idx));
                state_changes.push(state_change).ok();
            }
        });

        while let Ok(state_change) = TICK_STATE_CHANNEL.try_recv() {
            memo.update(&state_change);
            state_change.update_seq(&mut seq);
            let state_change = state_change.update_index(&seq);

            if let StateChange::Bpm(_) = state_change {
                let tick_duration = seq.tick_duration_micros();
                ticker = Ticker::every(Duration::from_micros(tick_duration));
            };

            DISPLAY_STATE_CHANNEL
                .try_send(state_change)
                .map_err(|_| panic!("display state channel full; state change"))
                .ok();
        }

        while let Option::Some(state_change) = state_changes.pop() {
            DISPLAY_STATE_CHANNEL
                .try_send(state_change)
                .map_err(|_| panic!("display state channel full; index change"))
                .ok();
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
            Direction::Clockwise => COMMAND_CHANNEL.send(Command::EncoderRight).await,
            Direction::Anticlockwise => COMMAND_CHANNEL.send(Command::EncoderLeft).await,
            Direction::None => {}
        }

        Timer::after(Duration::from_micros(1111)).await
    }
}

#[embassy_executor::task]
async fn core1_display_task(state: State, mut display: Display) {
    let mut screens = Screens::new();

    display.init().await;
    let next_screen = StateChange::NextScreen(ScreenState::new_home(
        state.bpm,
        state.sync,
        state.play_status,
    ));
    screens.draw(next_screen, &mut display);
    display.flush().await;

    loop {
        let state_change = DISPLAY_STATE_CHANNEL.recv().await;
        screens.draw(state_change, &mut display);
        display.flush().await
    }
}
