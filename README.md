# [WIP] Numcraft Rust

Numcraft Rust is a (WIP) cube sandbox game running natively on Numworks calculators.

## Screenshot

<img src="https://github.com/user-attachments/assets/0d386999-08ce-4145-be99-173479376119" width="512">

Numcraft running on actual N0110 Numworks on latest Epsilon. (more than 40 FPS on N120)

Note that this is not the final result but the current state of the project.

## Current project state
At that time, on the main branch, the app is capable of rendering a single chunk (but I can add more of then by adding one single line). The mesh is optimised using greedy meshing.
The program runs at a stable 40 FPS on both N110 and N120 models (the max framerate the screen can perform but we can go much higher by disabling Vsync)

## Support

NumcraftRust should run on both N120, N115 and N110 models. You will get better performances with the N120 model (N110 and N115 have the same CPU clock speed and same RAM).

## Build the app

To build this sample app, you will need to install an embedded ARM rust compiler as well as [Node.js](https://nodejs.org/en/). The SDK for Epsilon apps is shipped as an npm module called [nwlink](https://www.npmjs.com/package/nwlink) that will automatically be installed at compile time.

```shell
brew install rustup node # Or equivalent on your OS
rustup-init
rustup target add thumbv7em-none-eabihf
cargo build
```

## Run the app

The app is sent over to the calculator using the DFU protocol over USB.

```shell
# Now connect your NumWorks calculator to your computer using the USB cable
cargo run
```
