# Overview

The Chip8 is a programming language and virtual machine created in the 1970s for early microcomputers. It was originally designed to allow easy game development. This project provides YET ANOTHER IMPLEMENTATION of a disassembler and soon to be emulator for Chip8 programs, targeting Intel 8080-based systems.

The disassembler takes a Chip8 ROM file (.ch8) as input and converts the binary instructions into either a hexadecimal file or a file with the assembly language/opcodes. The hexadecimal file can be useful for further analysis or debugging, while the assembly language file provides a human-readable representation of the ROM's instructions. 

This is a fantastic resource for anything 8080 related:
<link>https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf</link>

The emulator allows you to load and execute Chip8 games by emulating the behavior of an Intel 8080 processor. It provides an interactive environment where you can play games using the keyboard as input and see the game output on the screen.\

# Usage

    ./target/debug/chip8_dissasembler -i ./testroms/Space\ Invaders\ \[David\ Winter\].ch8 -f "bin" // creates rom.hex
    ./target/debug/chip8_disassembler -i ./target/debug/rom.hex -f "hex" // creates rom.asm with CPU instructions

# Work in progress....

As of right now, it is not all that useful (who needs assembly language for a 60 year old CPU). When it can run space invaders, more info will be available.
