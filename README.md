### Running

$ cargo embed

### Debugging (Mac)

```bash
cargo embed
# open new terminal
cd projects/rust/toad-time
arm-none-eabi-gdb -q -x openocd.gdb target/thumbv6m-none-eabi/debug/rp2040-tmp
```

#### Old Way To Debug

```bash
$ openocd -f interface/cmsis-dap.cfg -c "adapter speed 5000" -f target/rp2040.cfg -s tcl
# then swap `probe-run` runner for `arm-none-eabi-gdb` runner in `.cargo/config.toml`
```
