# bf5rh-fw

> [!WARNING]
> Experimental, and under development.

Experimental from-scratch Rust firmware for the Baofeng UV-5RH/5RM.

Scaffolding copied and adapted from [bf-k61-firmware-aura](https://github.com/sophiel-meow/bf-k61-firmware-aura) — same
structure, different MCU family (KD32F328CB/Cortex-M0 there vs.
AT32F421/Cortex-M4 here), so the PAC crate, `memory.x`, and register
access are all new.

## Build

```sh
nix develop --command cargo build --release
nix develop --command cargo objcopy --release -- -O binary bf5rh-fw.bin
```

## Flash (stock bootloader entry only)

1. Wrap+encrypt the built binary(https://github.com/amoxu/Baofeng-UV-5RM-5RH-RE):
   ```sh
   ./uv5rm-wrap-tool/uv5rm-wrap-tool \
     -m wrap -i bf5rh-fw.bin -o bf5rh-fw.BF -f
   ```
   (region 2 is omitted here — an empty/absent region 2 is the
   untested path `tools/flash.py`'s docstring warns about.)
2. Hold the radio's side key while powering on so it enters flash mode
3. `nix develop --command python tools/flash.py bf5rh-fw.BF /dev/ttyUSB0`

