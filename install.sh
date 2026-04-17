# Yooso General UNIX Installer

# 1. Installs Rust toolchain if not already installed
#
if ! command -v rustc &> /dev/null
then
    # https://rust-lang.org/tools/install/
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    rustc --version
    cargo --version
fi

# 2. Installs Node.js and npm if not already installed
#
if ! command -v node &> /dev/null
then
    # https://nodejs.org/en/download
    echo "Node.js is not installed. Installing Node.js..."
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash
    \. "$HOME/.nvm/nvm.sh"
    nvm install 24
else
    echo "node $(node --version)"
    echo "npm $(npm --version)"
fi

# 3. Install web client local dependencies
#
if [ ! -d "node_modules" ]; then
    npm install
fi

# 4. Start the Rust server in a separate process.
# TODO:   cargo run --bin server & 

# 5. Start the web client application in a separate process.
# TODO:   npm run serve &

# 6. Wait for both processes to finish.
# TODO:   wait

