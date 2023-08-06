![icon-112x112](https://github.com/rodrigoCucick/rusted-chip8/assets/16089829/8e1def2b-d5fe-452e-a198-5f765a93845a)
# Rusted - Chip-8 Emulator/Interpreter
A Chip-8 emulator/interpreter written in Rust that uses SDL2.

# Objectives
This project started as a personal programming challenge during my job vacation and it has been evolving ever since.

__Sources of knowledge:__
|Link|Information|
|--|--|
|https://en.wikipedia.org/wiki/CHIP-8|Initial research was done here.|
|http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#8xy3|Pioneer Chip-8 technical reference.|
|https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set|Another Chip-8 technical reference with some additional information, specially about the bit shift, store and read instructions.|
|http://johnearnest.github.io/Octo/|I used this app to better understand the Chip-8 games.|
|https://doc.rust-lang.org/book/|That's right, I didn't know anything about Rust and yet I decided to program an emulator using it.|

# Current Features
As it is, the emulator is fully functional and capable of executing any Chip-8 game based on the original instruction set.

Some videos of it can be found here: https://www.youtube.com/c/RodrigoCucick

__Macro features:__
|Feature|Implemented?|
|--|--|
|Instructions (CPU)|Yes|
|Memory|Yes|
|Input|Yes|
|Display|Yes|
|Audio (buzzer)|Yes|

__Individual instructions:__
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

# Keyboard
The keyboard inputs are mapped to the following keys:

__Emulator layout__:
|||||
|--|--|--|--|
|1|2|3|4|
|Q|W|E|R|
|A|S|D|F|
|Z|X|C|V|

__Original ASCII layout (_for reference only_):__
|||||
|--|--|--|--|
|1|2|3|C|   
|4|5|6|D|   
|7|8|9|E|   
|A|0|B|F|

# Settings
The emulator also has an external configuration file called `config.txt`, where it is possible to adjust some settings like:
1. Video resolution scale.
2. Instruction cycles per frame.
3. Color (background and pixel).

# Building
This project uses __SDL2__ so, in order to build it, you'll need to follow some steps located on the following page: https://github.com/Rust-SDL2/rust-sdl2.

__SDL_image__ is also used, you can download it here: https://github.com/libsdl-org/SDL_image.

As an example, on my enviroment I had to put the SDL2 files in the folder `C:\Users\my_user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib` before using `cargo`.

The SDL2 files are the following:
1. SDL2.dll
2. SDL2.lib
3. SDL2main.lib
4. SDL2test.lib
5. SDL2_image.dll
6. SDL2_image.lib

To execute the emulator, the files `SDL2.dll` and `SDL2_image.dll` need to be in the same folder where the main executable `rusted-chip8.exe` is located.

To be able to debug the code _(I used __VS Code__ with the extension __CodeLLDB__)_, the files `SDL2.dll` and `SDL2_image.dll` need to be in the root folder of the project (alongside with `Cargo.toml`).

# Download
The latest releases can be downloaded here: https://github.com/rodrigoCucick/rusted-chip8/releases

It does not include any ROM file. Maybe in the future I'll program some simple games/examples.
