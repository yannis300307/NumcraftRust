app_name := "Numcraft"
lib_file_name := "libnumcraft_sim"


current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")

build-epsilon: setup_target
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features

build-upsilon: setup_target
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "upsilon" --no-default-features

send-epsilon: setup_target
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features
    npm exec --yes -- nwlink@0.0.19 install-nwa ./target/thumbv7em-none-eabihf/release/{{app_name}}

check: setup_target
    cargo check --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features
    cargo check --release --target={{current_target}} --lib --features "epsilon" --no-default-features
    cargo check --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "upsilon" --no-default-features
    cargo check --release --target={{current_target}} --lib --features "upsilon" --no-default-features
    @echo All checks passed!

setup_target:
    mkdir -p target/assets target/structs target/crafts

[macos]
run_nwb:
    ./simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/{{lib_file_name}}.dylib

[linux]
run_nwb:
    ./simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/{{lib_file_name}}.so

sim jobs="1" features="": setup_target
    -git clone https://github.com/numworks/epsilon.git simulator -b version-20 # Broken with version 21. Nice!
    if [ -n "{{features}}"];then \
        cargo build --release --target={{current_target}} --lib --features "epsilon" --no-default-features;\
    else \
        cargo build --release --target={{current_target}} --lib --features "{{features}} epsilon" --no-default-features;\
    fi

    if [ ! -f "target/simulator_patched" ]; then \
        cd simulator && make PLATFORM=simulator -j {{jobs}}; \
        cd ..; \
        echo "yes it is" >> target/simulator_patched; \
    fi
    just run_nwb

[confirm("This will clean the built app AND the simulator. Do you want to continue ?")]
clean-all:
    cd ./simulator && make clean
    cargo clean

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear-all:
    rm -rf ./simulator
    cargo clean
