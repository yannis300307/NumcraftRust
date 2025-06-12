<img src="https://github.com/user-attachments/assets/1eeccc90-342c-4f45-a444-7acc9cb9112a" width="128" alt="logo">

# [WIP] Numcraft Rust

Numcraft Rust is a (WIP) cube sandbox game running natively on Numworks calculators.

## Screenshot

<img src="https://github.com/user-attachments/assets/1a674a16-ef5c-4e37-a69f-f88afc7acc4b" width="512" alt="screenshot">

Numcraft running on actual N0110 Numworks on latest Epsilon. (more than 40 FPS on N120)

Note that this is not the final result but the current state of the project.

## Current project state
At that time, on the main branch, the app is capable of:
- rendering multiple chunks
- loading chunks generated with noise around the camera
- Having multiple colors for each blocks
- setting the light level for each individual quad

The program runs at 10-20 FPS on N0110 and 30-50 FPS on N0120.

## Support

NumcraftRust should run on both N120, N115 and N110 models. You will get better performances with the N120 model (N110 and N115 have the same CPU clock speed and same RAM).

## Build the app

To build this app, you will need to install an embedded ARM rust compiler as well as [Node.js](https://nodejs.org/en/). The SDK for Epsilon apps is shipped as an npm module called [nwlink](https://www.npmjs.com/package/nwlink) that will automatically be installed at compile time.

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
