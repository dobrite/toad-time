rtic on pi pico
https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_rtic.rs

A simple Pi Pico Rust setup based on the [rp-2040 template][1].

Blinks the built in LED, and prints "Hello Rust!" to a connected SD3306
OLED using SPI0. A second pi pico is setup as a probe using [picoprobe][2],
and `cargo-embed` as the runner. Also uses UART to print debug statements
to the console using the setup described in [the getting started guide][3].

### Running

$ cargo embed

### Debugging (Mac)

```bash
cargo embed
# open new terminal
cd projects/rust/rp-2040-tmp
arm-none-eabi-gdb -q -x openocd.gdb target/thumbv6m-none-eabi/debug/rp2040-tmp
```

#### Old Way To Debug

```bash
$ openocd -f interface/cmsis-dap.cfg -c "adapter speed 5000" -f target/rp2040.cfg -s tcl
# then swap `probe-run` runner for `arm-none-eabi-gdb` runner in `.cargo/config.toml`
```

[1]: https://github.com/rp-rs/rp2040-project-template
[2]: https://github.com/rp-rs/rp2040-project-template/blob/main/debug_probes.md#raspberry-pi-pico
[3]: https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf#page=64
