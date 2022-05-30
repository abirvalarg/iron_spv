# iron_spv

A supervisor for STM32 microcontrollers.

# Usage
## 1. Add `iron_spv` as a dependency
Don't forget to select peripheral drivers and modules by enabling features

## 2. Build target
You can specify your target in `/.cargo/config`. You will need to make your own linker script and
specify it in lineker options

Example of `/.cargo/config`:
```toml
[build]
target = "thumbv7em-none-eabihf"
rustflags = ["-Clinker=arm-none-eabi-ld", "-C", "link-arg=-Tmap.ld"]
```

### Linker script
#### Symbols
You must define a few symbols
|name|description|
|-|-|
|`_STACK`|Initial top of stack. Must point on the word right after the stack|
|`_DATA_START`|Start of `.data` segment in RAM|
|`_DATA_END`|End of `.data`|
|`_DATA_START_FLASH`|Location of initial data in ROM|
|`_BSS_START`|Start of `.bss` segment in RAM. This segment is initialised with `0`s|
|`_BSS_END`|End of `.bss`|
|`_HEAP_START`*|Start of heap segment|
|`_HEAP_END`*|End of heap segment|

\* those symbols are required only if `heap` feature is enabled

#### Segments
There aren't many
|name|description|
|-|-|
|`.text`|Contains executable code|
|`.text.init_vector`|Contains initialisation vector|
|`.rodata`|Constants, put it next to `.text`|
|`.data`|Segment for static/global variables, initialised from ROM|
|`.bss`|Segment for static/global variables, initialised with `0`|

## 3. `/src/main.rs`
Just repeat this template:
```rust
#![no_std]
#![no_main]

use iron_spv::prelude::*;

#[no_mangle]
extern "C" fn main() {
	// you code here
}
```
