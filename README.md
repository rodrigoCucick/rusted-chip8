![icon-112x112](https://github.com/rodrigoCucick/rusted-chip8/assets/16089829/8e1def2b-d5fe-452e-a198-5f765a93845a)
# Rusted - Chip-8 Emulator/Interpreter
A Chip-8 emulator/interpreter written in Rust that uses SDL2.

## Objectives
This project started as a personal programming challenge during my job vacation and it's been evolving ever since.

Here are the only sources of knowledge I used up to this point:
|Link|Information|
|--|--|
|http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#8xy3|This has to be one of the best Chip-8 technical reference out there. Don't let this page die!|
|https://en.wikipedia.org/wiki/CHIP-8|Initial research was done here.|
|http://johnearnest.github.io/Octo/|I used this app to better understand the Chip-8 game programs.|
|https://doc.rust-lang.org/book/|That's right, I didn't know anything about Rust and yet I decided to program an emulator using it... This book is phenomenal and covers basically everything you need to know to start your journey with the language. If you're an experienced Rust developer, please don't mind some "inappropriate" code here and there.|

## Current Features
Macro features:
|Feature|Implemented?|
|--|--|
|Instructions (CPU)|Yes|
|Memory|Yes|
|Input|Yes|
|Display|Yes|
|Audio (buzzer)|No|

Individual instructions:
|Instruction|Implemented?|
|--|--|
|`00E0` - CLS|Yes|
|`00EE` - RET|Yes|
|`0nnn` - SYS addr|No (not used)|
|`1nnn` - JP addr|Yes|
|`2nnn` - CALL addr|Yes|
|`3xkk` - SE Vx, byte|Yes|
|`4xkk` - SNE Vx, byte|Yes|
|`5xy0` - SE Vx, Vy|Yes|
|`6xkk` - LD Vx, byte|Yes|
|`7xkk` - ADD Vx, byte|Yes|
|`8xy0` - LD Vx, Vy|Yes|
|`8xy1` - OR Vx, Vy|Yes|
|`8xy2` - AND Vx, Vy|Yes|
|`8xy3` - XOR Vx, Vy|Yes|
|`8xy4` - ADD Vx, Vy|Yes|
|`8xy5` - SUB Vx, Vy|Yes|
|`8xy6` - SHR Vx {, Vy}|Yes|
|`8xy7` - SUBN Vx, Vy|Yes|
|`8xyE` - SHL Vx {, Vy}|Yes|
|`9xy0` - SNE Vx, Vy|Yes|
|`Annn` - LD I, addr|Yes|
|`Bnnn` - JP V0, addr|Yes|
|`Cxkk` - RND Vx, byte|Yes|
|`Dxyn` - DRW Vx, Vy, nibble|Yes|
|`Ex9E` - SKP Vx|Yes|
|`ExA1` - SKNP Vx|Yes|
|`Fx07` - LD Vx, DT|Yes|
|`Fx0A` - LD Vx, K|Yes|
|`Fx15` - LD DT, Vx|Yes|
|`Fx18` - LD ST, Vx|Yes|
|`Fx1E` - ADD I, Vx|Yes|
|`Fx29` - LD F, Vx|Yes|
|`Fx33` - LD B, Vx|Yes|
|`Fx55` - LD [I], Vx|Yes|
|`Fx65` - LD Vx, [I]|Yes|

As it is, the emulator is only missing the buzzer, that means no sound effects for now.

## Building
This project uses SDL2 so, in order to build it, you'll need to follow some steps located on the page https://github.com/Rust-SDL2/rust-sdl2.

SDL_image is also used, you can download it here https://github.com/libsdl-org/SDL_image.

As an example, on my enviroment I had to put the SDL2 files in the folder `C:\Users\my_user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib` before using `cargo`.

The SDL2 files are the following:
1. SDL2.dll
2. SDL2.lib
3. SDL2main.lib
4. SDL2test.lib
5. SDL2_image.dll
6. SDL2_image.lib
