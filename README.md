# [WIP] Numcraft Rust

Numcraft Rust is a (WIP) cube sandbox game running natively on Numworks calculators.

## Support

NumcraftRust should run on both N120, N115 and N110 models. You will get better performances with the N120 model (N110 and N115 have the same CPU).

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
