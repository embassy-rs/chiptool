# chiptool

`chiptool` is an experimental fork of `svd2rust` to experiment with:

- Different API for the generated code.
- Integrating "transforms" in the generation process
- New workflow for storing register definitions in standalone YAML files.

## Example

Tested with the RP2040 SVD. Other SVDs might not work quite right yet.

- svd: https://github.com/Dirbaio/svd2rust/blob/master/svd/rp2040.svd
- yaml: https://github.com/Dirbaio/svd2rust/blob/master/svd/rp2040.yaml
- repo: https://github.com/Dirbaio/rp2040-pac/settings
- docs: https://dirbaio.github.io/rp2040-pac/rp2040_pac/index.html

## Changes from svd2rust main

### No owned structs

Original svd2rust generates an owned struct for each peripheral. This has turned out to have some severe downsides:

1. there are many cases where the HAL wants to "split up" a peripheral into multiple owned parts. Examples:
   - Many pins in a GPIO port peripheral.
   - The RX and TX halfs of a UART peripheral.
   - Different clocks/PLLs in a clock control peripheral.
   - Channels/streams in a DMA controller
   - PWM channels

    Virtually all existing HALs run into this issue, and have to unsafely bypass the ownership rules. [nrf gpio](https://github.com/nrf-rs/nrf-hal/blob/6fc5061509d5f3efaa2db15d4af7e3bced4a2e83/nrf-hal-common/src/gpio.rs#L135), [nrf i2c](https://github.com/nrf-rs/nrf-hal/blob/1d6e228f11b7df3847d33d66b01ff772501beb3c/nrf-hal-common/src/twi.rs#L28), [nrf ppi](https://github.com/nrf-rs/nrf-hal/blob/8a28455ab93eb47be4e4edb62ebe96939e1a7ebd/nrf-hal-common/src/ppi/mod.rs#L122), [stm32f4 gpio](https://github.com/stm32-rs/stm32f4xx-hal/blob/9b6aad4b3365a48ae652c315730ab47522e57cfb/src/gpio.rs#L302), [stm32f4 dma](https://github.com/stm32-rs/stm32f4xx-hal/blob/9b6aad4b3365a48ae652c315730ab47522e57cfb/src/dma/mod.rs#L359), [stm32f4 pwm](https://github.com/stm32-rs/stm32f4xx-hal/blob/bb214b6017d84a9c8dd2e8c9fd1f915141e167cc/src/pwm.rs#L228), [atsamd gpio](https://github.com/atsamd-rs/atsamd/blob/4816bb13a12a604e51f929d17b286071a0082c82/hal/src/common/gpio/v2/pin.rs#L669) ...

    Since HALs in practice always bypass the PAC ownership rules and create their own safe abstractions, there's not much advantage in having ownership rules in the PAC in the first place. Not having them makes HAL code cleaner.

2. sometimes "ownership" is not so clear-cut:
    - Multicore. Some peripherals are "core-local", they have an instance per core. Constant address, which instance you access depends on which core you're running on. For example Cortex-M core peripherals, and SIO in RP2040.
    - Mutually-exclusive peripherals. In nRF you can only use one of (UART0, SPIM0, SPIS0, TWIM0, TWIS0) at the same time, one of (UART1, SPIM1, SPIS1, TWIM1, TWIS1) at the same time... They're the same peripheral in different "modes". Current nRF PACs get this wrong, allowing you to use e.g. SPIM0 and TWIM0 at the same time, which breaks.
3. Ownership in PACs means upgrading the PAC is ALWAYS a breaking change.

    To guarantee you can't get two singletons for the same peripheral, PACs deliberately sabotage building a binary containing two PAC major versions (with this [no\_mangle thing](https://github.com/nrf-rs/nrf-pacs/blob/8f9da05ca1b496bd743f223ed1122dfe9220956c/pacs/nrf52840-pac/src/lib.rs#L2279-L2280)).

    This means the HAL major-bumping the PAC dep version  is a breaking change, so the HAL would have to be major-bumped as well. And all PAC bumps are breaking, and they're VERY common...

### All register access is unsafe

Reasons:

- Since there are no owned structs, there can be data races when writing to a register from multiple contexts (eg main thread and interrupt). Ensuring no data races is left to the HALs (HALs are already doing this anyway, see above)
- DMA registers can be turned into arbitrary pointer dereferencing.
- Controls for low-level chip features such as RAM power control or clock control can break safety in interesting ways.

### Structs representing register values (sets of fields)

Current svd2rust provides "read proxy" and "write proxy" structs with methods to access register fields when reading/writing. However:

- There's no type-safe way to save the _value_ of a register in a variable to write later. (there's `.bits()`, but it's not typesafe)
- There's no way to read/modify register fields on a saved value (if using `.bits()`, the user has a raw u32, they need to extract the fields manually with bitwise manipulation)

Solution: for each register with fields, a "fieldset" struct is generated. This struct wraps the raw `u32` and allows getting/setting individual fields.

```rust
let val = pac::watchdog::fields::Tick(0);
val.set_cycles(XOSC_MHZ as u16);
val.set_enable(true);
info!("enabled: {:bool}", val.enable());
```

On a register, `.read()` and `.write_value()` can get and set such fieldset values:

```rust
let val = pac::WATCHDOG.tick().read();
val.set_enable(false);
// We could save val in a variable somewhere else
// then get it and write it back later
pac::WATCHDOG.tick().write_value(val);
```

Closure-based `.write()` and `.modify()` are provided too, like the current svd2rust.

```rust
pac::WATCHDOG.tick().write(|w| {
    w.set_cycles(XOSC_MHZ as u16);
    w.set_enable(true);
});
```

### Structs representing enumerated values

For each EnumeratedValues in a field, a struct is generated.

This struct is _not_ a Rust enum, it is a struct with associated constants.

### Possibility to share items (blocks, fieldsets, enums)

Many peripherals have multiple registers with the same fields (same names, same bit offsets). This tool allows the user to merge them via YAML config. Same for enums and register blocks.

Fieldsets and enums can be shared across different registers, different register blocks, even different peripherals.

Example: the RP2040 chip has two GPIO banks: `BANK0` and `QSPI`. These share many enums and field sets. Example of merging some:

```yaml
- MergeEnums:
    from: io_[^:]+::values::Gpio.+Ctrl(.+)over
    to: io::values::${1}over
```

This merges all `INOVER`, `OUTOVER`, `OEOVER` and `IRQOVER` enums (144 enums!) into just 4.

- huge reduction in generated code, mitigating long compile times which is one of the top complaints of current PACs.
- Better code sharing in HALs since they can use a single enum/fieldset to read/write to multiple registers.

### Automatic cluster creation

```yaml
- MakeBlock:
    block: pio0::Pio0
    from: sm(\d+)_(.+)
    to_outer: sm$1
    to_inner: $2
    to_block: pio0::StateMachine
```

This collapses all `smX_*` registers into a single cluster:

    // before:
    RegisterBlock:
      sm0_clkdiv
      sm0_execctrl
      sm0_shiftctrl
      sm0_addr
      sm0_instr
      sm0_pinctrl
      sm1_clkdiv
      sm1_execctrl
      sm1_shiftctrl
      sm1_addr
      sm1_instr
      sm1_pinctrl
      sm2_clkdiv
      sm2_execctrl
      sm2_shiftctrl
      sm2_addr
      sm2_instr
      sm2_pinctrl
      sm3_clkdiv
      sm3_execctrl
      sm3_shiftctrl
      sm3_addr
      sm3_instr
      sm3_pinctrl

    // after:
    RegisterBlock:
      sm0
      sm1
      sm2
      sm3

    StateMachine block:
      clkdiv
      execctrl
      shiftctrl
      addr
      instr
      pinctrl

### Automatic array creation

example:

```yaml
- MakeRegisterArray:
    block: pio0::Pio0
    from: sm\d+
    to: sm
```

    // before:
    RegisterBlock:
      sm0
      sm1
      sm2
      sm3

    // after:
    RegisterBlock:
      sm (array of length 4)

### RegisterBlocks and Registers wrap pointers

```rust
// a RegisterBlock
pub struct Resets(*mut u8);

impl Resets {
    // A register access function. This is just pointer arithmetic
    pub fn reset_done(self) -> Reg<fields::Peripherals, RW> {
        unsafe { Reg::new(self.0.add(8usize))) }
    }
}

// the Reg struct
pub struct Reg<T: Copy, A: Access> {
    ptr: *mut u8,
    ...
}
```

- No need to calculate and fill padding holes in RegisterBlock structs
- No problem if registers overlap (currently svd2rust has to check for this, and falls back to a function-based codegen similar to this one)
- Pointer provenance is not erased. Previous codegen causes pointers to become references (&), so it's undefined behavior to do arithmetic with a register pointer to write somewhere else. This is useful in a few niche situations:
  - calculating a pointer to a particular register bit in the bitbanding region
  - The RP2040 chip has register aliases that atomically set/clear/xor register bits at addr + 0x1000/0x2000/0x3000

This generates the same assembly code as original svd2rust when optimizations are enabled.

## Running

    mkdir -p out
    mkdir -p out/src
    cargo run -- -i svd/rp2040.svd -c svd/rp2040.yaml
    rustfmt out/src/lib.rs
    (cd out; cargo build && cargo doc)

## To-Do

Missing features:

- Clusters in input SVD file
- registers with bit width other than 32

Nice to have features:

- More transforms (deletes, renames, move entire module...)
- clean up doc comments better

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][coc], the maintainer of this crate, the [Tools team][team], promises
to intervene to uphold that code of conduct.

[coc]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-tools-team
