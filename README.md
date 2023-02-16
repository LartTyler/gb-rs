# gb-rs
Just another Gameboy emulator, written in Rust.

# Features
Currently completed (or rather, "ready") features include:

- [x] Disassembler / ASM parser
- [ ] Cartridge and MBC emulation
	- [x] MBC0 (no memory bank controller)
	- [x] MBC1
	- [x] MBC3
	- [ ] MBC5
- [x] Memory management (MMU)
- [ ] CPU Instructions
	- [ ] "Main" instructions
	- [ ] "Extended (CB) instructions
- [ ] Graphics

# Sources
Big thanks to the following for providing excellent documentation for the inner workings of the Gameboy hardware.

- [Pan Docs](https://gbdev.io/pandocs/)
- [The Cycle Accurate Game Boy Docs](https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf)
- [PastRaiser's Gameboy CPU Instruction Set Table](https://pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [RGBDS Opcode Reference](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7)
- [WTF is the DAA Instruction](https://ehaskins.com/2018-01-30%20Z80%20DAA/)
- Matt Currie's [Mealyroom Tearoom Tests](https://github.com/mattcurrie/mealybug-tearoom-tests)
- Kevin Horton's [Nitty Gritty Gameboy VRAM Timings](http://blog.kevtris.org/blogfiles/Nitty%20Gritty%20Gameboy%20VRAM%20Timing.txt)
