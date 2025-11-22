#!/bin/bash

echo "This setup script will automatically install all the dependencies for this template. It will install, if not installed:"
echo " - Rustup"
echo " - Cargo"
echo " - Just"
echo " - Python3, pip3 and python3-venv"
echo " - The Numworks sdk including arm-none-eabi-gcc"
echo " - NVM and Node js"
echo " - Other packages from apt repositories (see code for more info)"
echo " - Lz4, pypng and stringcase"
echo " - The Epsilon simulator"
echo "By using this installer, you agree with all the licenses associated with these softwares."
echo
echo "THIS SCRIPT ONLY WORKS ON DEBIAN BASED LINUX DISTROS!"
echo
read -p "Do you want to continue ? (y/N) " -r

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

which apt
if [ $? -ne 0 ]; then
    echo "Cannot reach the apt cli. Are you on a Debian based Linux distro? Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

sudo apt update
sudo apt install build-essential git imagemagick libx11-dev libxext-dev libfreetype6-dev libpng-dev libjpeg-dev pkg-config python3 python3-pip python3-venv curl libatomic1 -y
if [ $? -ne 0 ]; then
    echo "The installation of the dependency packages has failed. Perhaps sudo is not installed? Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

which arm-none-eabi-gcc
if [ $? -ne 0 ]; then
    echo "Installing gcc toolchain"
    sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi -y
fi

if [ ! -d "simulator" ]; then
    echo "Clonning Epsilon."
    git clone https://github.com/numworks/epsilon.git simulator -b version-20
    if [ $? -ne 0 ]; then
        echo "Cannot clone the Epsilon repository. Installation aborted."
        [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
    fi
fi

python3 -m venv ./simulator/.venv
if [ $? -ne 0 ]; then
    echo "Cannot create the Python venv. Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

source ./simulator/.venv/bin/activate
if [ $? -ne 0 ]; then
    echo "Cannot activate the Python venv. Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

pip3 install lz4 pypng stringcase
if [ $? -ne 0 ]; then
    echo "The installation of the pip packages has failed. Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

which cargo
if [ $? -ne 0 ]; then
    echo "Cargo not installed. Checking for Rustup."
    which rustup
    if [ $? -ne 0 ]; then
        echo "Rustup not installed. Downloading Rustup"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        . "$HOME/.cargo/env"
    else
        echo "Rustup installed but Cargo is not. Updating Rustup."
        rustup update
        if [ $? -ne 0 ]; then
            echo "Rustup is installed but Cargo is not. Please install cargo manually. Installation aborted."
            [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
        fi
    fi
    which cargo
    if [ $? -ne 0 ]; then
        echo "Cargo installation failed. Please retry or install cargo manually. Installation aborted."
        [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
    fi
fi

which npx
if [ $? -ne 0 ]; then
    echo "Npx is not installed. Checking for NVM."
    which nvm
    if [ $? -ne 0 ]; then
        echo "NVM is not installed. Downloading NVM"
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
        export NVM_DIR="$HOME/.nvm"
        [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
        [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
        which nvm
        if [ $? -ne 0 ]; then
            echo "NVM installation failed. Please retry or install NVM manually. Installation aborted."
            [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
        fi
    fi
    nvm install node
    if [ $? -ne 0 ]; then
        echo "Node installation failed. Please retry or install Node manually. Installation aborted."
        [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
    fi
    which npx
    if [ $? -ne 0 ]; then
        echo "Npx installation failed. Please retry or install Npx manually. Installation aborted."
        [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
    fi
fi

rustup target add thumbv7em-none-eabihf
if [ $? -ne 0 ]; then
    echo "The thumbv7em-none-eabihf target installation failed. Please retry or install the thumbv7em-none-eabihf target manually. Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi
cargo install just
if [ $? -ne 0 ]; then
    echo "Just installation failed. Please retry or install Just manually. Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi
npm install nwlink
if [ $? -ne 0 ]; then
    echo "Nwlink installation failed. Please retry or install Nwlink manually. Installation aborted."
    [[ "$0" = "$BASH_SOURCE" ]] && exit 1 || return 1
fi

echo
echo "=================================="
echo "|  Installation finished! Nice!  |"
echo "=================================="
echo
echo "Run \`. ~/.bashrc\` or reopen your terminal to load the newly added commands."
echo
echo "Type \`just --list\` to see the available commands."
