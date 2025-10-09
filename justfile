current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")

build: setup_target
    cargo build --release --bin Numcraft --target=thumbv7em-none-eabihf

send: setup_target
    cargo run --release --bin Numcraft --target=thumbv7em-none-eabihf

check: setup_target
    cargo build --release --bin Numcraft --target=thumbv7em-none-eabihf
    cargo build --release --target={{current_target}} --lib


setup_target:
    mkdir -p target/assets target/structs target/crafts

[macos]
run_nwb:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/libnumcraft_sim.dylib

[linux]
run_nwb:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/libnumcraft_sim.so

sim jobs="1" features="": setup_target
    -git clone https://github.com/numworks/epsilon.git epsilon_simulator -b version-20 # Broken with version 21. Nice!
    if [ -n "{{features}}"];then \
        cargo build --release --target={{current_target}} --lib;\
    else \
        cargo build --release --target={{current_target}} --lib --features {{features}};\
    fi

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
