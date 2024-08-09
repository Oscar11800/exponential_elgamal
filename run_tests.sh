#!/bin/bash

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Navigate to the project directory
cd /home/ubuntu/trusted_apps/exponential_elgamal/ || { echo "Project directory not found!"; exit 1; }

# Check for necessary files
if [ ! -f "Nargo.toml" ]; then
    echo "Nargo.toml not found!"
    exit 1
fi

# Ensure 'cargo' is installed
if ! command_exists cargo; then
    echo "'cargo' is not installed. Attempting to install Rust..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y || { echo "Failed to install 'cargo'. Please install it manually."; exit 1; }
    source $HOME/.cargo/env
fi

# Ensure 'nargo' is installed
if ! command_exists nargo; then
    echo "'nargo' is not installed. Attempting to install..."
    curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash || { echo "Failed to install 'noirup'. Please install it manually."; exit 1; }
    noirup --version 0.32.0 || { echo "Failed to install 'nargo'. Please install it manually."; exit 1; }
fi

# Install dependencies
echo "Installing dependencies..."
cd babygiant_native/
cargo build || { echo "Dependency installation failed!"; exit 1; }

# Build the project
echo "Building the project..."
cd ..
nargo build || { echo "Build failed!"; exit 1; }

# Run tests with output
echo "Running tests with output..."
nargo test --show-output || { echo "Tests failed!"; exit 1; }

echo "Script execution completed."
