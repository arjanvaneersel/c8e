# 🕹️ C8e

A minimalist, cross-platform [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust.

This project is designed for learning, hacking, and retro gaming fun — while showcasing clean Rust code, memory-safe systems programming, and modular emulator architecture.

---

## Copyright and licensing
Copyright (c) 2025, Synthonyx Technologies Ltd

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

---

## ✨ Features

- ✅ Fully interprets the original CHIP-8 instruction set (~35 opcodes)
- ✅ Memory-mapped 64×32 monochrome display (configurable backend)
- ✅ 16-key hexadecimal keypad input
- ✅ 60Hz timers (delay & sound)
- ✅ Compatible with most public CHIP-8 ROMs and test suites
- ✅ Cross-platform (Linux, macOS, Windows, WASM-ready)

---

## 📸 Screenshots

| Game        | Screenshot       |
|-------------|------------------|
| Pong        | (Coming Soon)    |
| Tetris      | (Coming Soon)    |
| Blinky      | (Coming Soon)    |

---

## 📦 Installation

### Prerequisites
- Rust (latest stable) — [Install via rustup](https://rustup.rs)

### Clone & Build
```bash
git clone https://github.com/arjanvaneersel/c8e.git
cd c8e
cargo run --release -- path/to/roms/PONG.ch8
````

---

## 📚 Documentation & References

### 📜 CHIP-8 Specifications

* [Cowgod\'s CHIP-8 Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) – 🧠 *Most authoritative and widely used spec*
* [Wikipedia - CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) – Good intro
* [CHIP-8 Test ROMs](https://github.com/dmatlack/chip8/tree/master/roms) – For validation and fun
* [CHIP-8 Test Suite by Tobiasvl](https://github.com/tobiasvl/chip8-test-suite)

---

## 🎮 ROMs

You can find public-domain test ROMs and games here:

* [https://github.com/dmatlack/chip8/tree/master/roms](https://github.com/dmatlack/chip8/tree/master/roms)

Drop any `.ch8` ROM into the project and run with:

```bash
cargo run --release -- roms/INVADERS.ch8
```

---

## 🧠 Architecture Overview

```text
┌────────────────────────────┐
│        Emulator Core       │
│ ┌────────┐ ┌────────────┐  │
│ │ CPU    │ │ Timers     │  │
│ │        │ │            │  │
│ └────────┘ └────────────┘  │
│ ┌────────┐ ┌────────────┐  │
│ │ Memory │ │ Display    │  │
│ └────────┘ └────────────┘  │
└────────────────────────────┘
            ▲
     Input & Events
```

---

## ✅ TODO

* [ ] Sound output
* [ ] Configurable resolution (Super CHIP-8?)
* [ ] WebAssembly (WASM) frontend
* [ ] SDL2 backend for native UI
* [ ] Unit tests for instruction set

---

## 🙌 Credits

* Cowgod for the definitive CHIP-8 spec
* David Matlack, Tobiasvl, and others for test ROMs
* Rust community for making systems programming fun again