<img src="https://github.com/user-attachments/assets/1eeccc90-342c-4f45-a444-7acc9cb9112a" width="128" alt="logo">

# [WIP] Numcraft Rust

Numcraft Rust is a (WIP) cube sandbox game running natively on Numworks calculators.

## Screenshot

<img src="https://github.com/user-attachments/assets/f247677e-0f19-4170-92b9-51563961c862" width="512" alt="screenshot">



Numcraft running on actual N0110 Numworks on latest Epsilon. (more than 40 FPS on N120)

Note that this is not the final result but the current state of the project.

## Current project state
[See Roadmap below]

The program runs at 10-20 FPS on N0110 and 30-50 FPS on N0120.

## Support

NumcraftRust should run on both N120, N115 and N110 models. You will get better performances with the N120 model (N110 and N115 have the same CPU clock speed and same RAM).

## Roadmap
**Here is the current roadmap for the project :**
- [X] Fix the raycaster
- [ ] Textures
- [X] World deletion
- [X] World creation settings
- [X] Save system with compression
- [ ] Collisions
- [ ] Better world generation
- [X] Main menu
- [ ] Creative menu to select multiple blocks
- [X] Menu to select between multiple worlds
- [ ] Lightning engine
- [ ] Mobs
- [ ] Survival mode

If I have the time:
- [ ] TNT
- [ ] Redstone
- [ ] Commands support
- [ ] Make a tool to convert Minecraft schematics to Numcraft structures
- [ ] Structures such as Villages
- [ ] Upsilon / Omega compatibility

Good ideas but I will never have the time to do that :
- [ ] Mod support
- [ ] Multiplayer (impossible at that time)

## Known Bugs:
- ~Weird beahaviors in the negative coordinates~
- Rendering issues with the block selection marker
- Can randomly crash on start on N0120. You have to reset your calculator before downloading a new version (I can't do anything about that, it's related to the OS)
- Rendering issues and memory corruption if too many triangles are shown at the same time on the screen

## Build the app

To build this app, you will need to install an embedded ARM rust compiler, the [Arm GCC compiler](https://developer.arm.com/downloads/-/gnu-rm) as well as [Node.js](https://nodejs.org/en/). The SDK for Epsilon apps is shipped as an npm module called [nwlink](https://www.npmjs.com/package/nwlink) that will automatically be installed at compile time. Make sure that `arm-none-eabi-gcc`is in your path.

```shell
brew install rustup node # Or equivalent on your OS
rustup-init
rustup target add thumbv7em-none-eabihf
cargo build --release
```

## Run the app

The app is sent over to the calculator using the DFU protocol over USB.

```shell
# Now connect your NumWorks calculator to your computer using the USB cable
cargo run --release
```
