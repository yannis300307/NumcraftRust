# nwlink

The `nwlink` command-line tool allows you to communicate with your NumWorks calculator from the command line.

Simply install it with `npm install -g nwlink`, the run `nwlink --help` to get a list of possible commands.

### About binutils

This modules contains two WebAssembly files, `ld.wasm` and `objcopy.wasm`. Those files (and only those files) are distributed under the GNU GPL license, version 2. They are built from the unmodified sources of binutils version 2.38. To regenerate them, install emscripten 3.1.9-asserts and binutils 2.38, then run the following commands:

```sh
source emsdk/emsdk_env.sh
rm -rf build
mkdir build
cd build
SFLAGS="-Os -flto -sENVIRONMENT=web,node -sALLOW_MEMORY_GROWTH -sMODULARIZE -sEXPORTED_RUNTIME_METHODS=['FS']"
emconfigure ../configure \
  --host=wasm32 \
  --target=arm-none-eabi \
  --disable-libstdcxx \
  --enable-compressed-debug-sections=none \
  --disable-nls \
  --disable-largefile \
  CFLAGS="$SFLAGS" CXXFLAGS="$SFLAGS"
emmake make
```

Note that these two WebAssembly files can be re-used separately, and the rest of the code communicates with them using command-line arguments. They are therefore included here as a [mere aggregate](https://www.gnu.org/licenses/gpl-faq.html#MereAggregation) and the GNU GPL license subsequently does *not* apply to the rest of the provided files.
