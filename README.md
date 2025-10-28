<img src="https://github.com/user-attachments/assets/1eeccc90-342c-4f45-a444-7acc9cb9112a" width="128" alt="logo">

# [WIP] Numcraft Rust

Numcraft Rust is a (WIP) cube sandbox game running natively on Numworks calculators.

## Screenshot

<img width="470" height="378" alt="image" src="https://github.com/user-attachments/assets/e704a3fb-6b2d-4f88-a660-9e3f469bf65d" />


<img src="https://github.com/user-attachments/assets/f247677e-0f19-4170-92b9-51563961c862" width="512" alt="screenshot">



Numcraft running on actual N0110 Numworks on latest Epsilon (more than 40 FPS on N120) and on the simulator.

Note that this is not the final result but the current state of the project.

## Current project state
[See Roadmap below]

The program runs at 10-20 FPS on N0110 and 30-50 FPS on N0120. It runs perfectly on the simulator (it depends on your hardware though).

## Support

NumcraftRust should run on both N120, N115 and N110 models. You will get better performances with the N120 model (N110 and N115 have the same CPU clock speed and same RAM).

## Roadmap
**Here is the current roadmap for the project :**
- [X] Fix the raycaster
- [ ] Textures
- [X] World deletion
- [X] World creation settings
- [X] Save system with compression
- [X] Collisions
- [X] Better world generation
- [X] Main menu
- [X] Creative menu to select multiple blocks
- [X] Menu to select between multiple worlds
- [ ] Lighting engine
- [ ] Mobs
- [X] Survival mode

If I have the time:
- [ ] TNT
- [ ] Redstone
- [ ] Commands support
- [ ] Make a tool to convert Minecraft schematics to Numcraft structures
- [ ] Structures such as Villages
- [ ] Upsilon / Omega compatibility
- [ ] Mini games

Good ideas but I will never have the time to do that :
- [ ] Mod support
- [ ] Multiplayer (impossible at this point in time)

## Known Bugs:
- Weird beahaviors in the negative coordinates
- Rendering issues with the block selection marker
- Can randomly crash on start on N0120. You have to reset your calculator before downloading a new version (I can't do anything about that, it's related to the OS)
- Rendering issues and memory corruption if too many triangles are shown at the same time on the screen

## Setup the build environment

To build this app, you will need to install an embedded ARM rust compiler, the [Arm GCC compiler](https://developer.arm.com/downloads/-/gnu-rm) as well as [Node.js](https://nodejs.org/en/). 
The SDK for Epsilon apps is shipped as a npm module called [nwlink](https://www.npmjs.com/package/nwlink) that will automatically be installed at compile time.
**Make sure that `arm-none-eabi-gcc`is in your path.**

For more explanations on how to install the c sdk, follow [this guide](https://www.numworks.com/engineering/software/build/).

You might need to create a Python venv in the `epsilon_simulator` folder to install the pip packages on certain Linux distros. 

Then, you can set up the dependencies as follows :
```shell
brew install rustup node # Or equivalent on your OS
rustup-init
rustup target add thumbv7em-none-eabihf
cargo install just # Similar to makefile
```

## Build the app
```shell
just build
```

## Build and send the app to an actual calculator

Connect the calculator to your computer and run
```shell
just send
```

## Run the app on the simulator

```shell
just sim
```
The simulator inputs will be automatically remapped for a better experience.

Use `w`, `s`, `a` and `d` to move the player, `shift` and `space` to go up and down, arrows to turn the camera, `return` to place a block or select in a menu and `backspace` to break a block or to go back in a menu.

You can speed up the simulator build by setting the job number.
```shell
just sim 5
```

## Legal info
NumWorks is a registered trademark.
