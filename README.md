# MicroBit Name Display in Rust (Bare Metal)

**Description:**

The "MicroBit Name Display in Rust" project is a demonstration of using the Rust programming language for bare-metal programming on a MicroBit device. It showcases how to display a name or message on the MicroBit's LED matrix, making it an excellent resource for those interested in embedded systems and Rust programming.

**Usage:**

To build and run the project, follow these commands:

1. Build the project with the specified features and target:

   ```shell
   cargo build --features v2 --target thumbv7em-none-eabihf
   ```

2. Embed the project onto the MicroBit device:

   ```shell
   cargo embed --features v2 --target thumbv7em-none-eabihf
   ```

3. Debug the project using GDB (optional):

   ```shell
   gdb target/thumbv7em-none-eabihf/debug/led-roulette
   ```

The project provides an educational platform for those interested in Rust-based bare-metal programming on MicroBit devices.

**Features:**

- **LED Matrix Display:** The project demonstrates how to use Rust to control the MicroBit's LED matrix to display text, allowing for customized messages or names to be shown.

- **Bare-Metal Programming:** It showcases the principles of bare-metal programming, where you have direct control over hardware resources without the need for an operating system.

**Resources Used**

- https://docs.rs/microbit/latest/microbit/
- https://droogmic.github.io/microrust/
- https://github.com/nrf-rs/microbit
- https://docs.rust-embedded.org/discovery/microbit/
