# Yooso General UNIX Installer

# This script is useful for first-time contributors and users who
# try to quickly set up a running Yooso instance on their local
# machine in development mode.
# This script is opiniated and will install the Rust and Node.js
# toolchains, including the node package manager (npm) if they are
# not already installed on the system.
# If you prefer to use a different method of installing, or prefer
# to use different toolchains, you should ignore this script.

#
# DEPENDENCY INSTALLATION
#

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

#
# FIRST TIME PROJECT SETUP
#

# 3. Install web client local dependencies
#
if [ ! -d "node_modules" ]; then
    npm install
fi

#
# STARTING THE APPLICATION
#
# To start the application you can run either this script, or open
# two different terminal windows and run the following commands in
# each console:
#
# > cargo run --bin yooso-example
# > npm run dev
#

# 4. Start the Rust server in a separate process.
#
cargo run --bin yooso-example & 

# 5. Start the web client application in a separate process.
#
npm run dev &

# 6. Wait for both processes to finish.
# 
wait
