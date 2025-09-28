current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")

build:
    cargo build --release --bin Numcraft --target=thumbv7em-none-eabihf

send:
    cargo run --release --bin Numcraft --target=thumbv7em-none-eabihf

check:
    cargo build --release --bin Numcraft --target=thumbv7em-none-eabihf
    cargo build --release --target={{current_target}} --lib

[macos]
run_nwb:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/libnumcraft_sim.dylib

[linux]
run_nwb:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/libnumcraft_sim.so

sim jobs="1":
    -git clone https://github.com/numworks/epsilon.git epsilon_simulator -b version-20 # Broken with version 21. Nice!
    cargo build --release --target={{current_target}} --lib
    if [ ! -f "target/simulator_patched" ]; then \
        cd epsilon_simulator && make PLATFORM=simulator -j {{jobs}}; \
        cd ..; \
        echo "yes it is" >> target/simulator_patched; \
    fi
    just run_nwb

[confirm("This will clean the built app AND the simulator. Do you want to continue ?")]
clean-all:
    cd ./epsilon_simulator && make clean
    cargo clean

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear-all:
    rm -rf ./epsilon_simulator
    cargo clean
