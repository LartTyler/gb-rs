# gb-rs
Just another Gameboy emulator, written in Rust.

# Features
Currently completed (or rather, "ready") features include:

- [ ] Cartridge and MBC emulation
	- [x] MBC0 (no memory bank controller)
	- [x] MBC1
	- [x] MBC3
	- [ ] MBC5
- [x] Memory management (MMU)

# Sources
Big thanks to the following for providing excellent documentation for the inner workings of the Gameboy hardware.

- [Pan Docs](https://gbdev.io/pandocs/)
- [The Cycle Accurate Game Boy Docs](https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf)
- [PastRaiser's Gameboy CPU Instruction Set Table](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [RGBDS Opcode Reference](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7)
