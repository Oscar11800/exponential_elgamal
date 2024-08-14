#!/bin/bash

# Ensure 'nargo' is installed with the correct version
REQUIRED_VERSION="0.32.0"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

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

# Ensure 'nargo' is installed with the correct version
if ! command_exists nargo || [ "$(nargo --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')" != "$REQUIRED_VERSION" ]; then
    echo "'nargo' is not installed or not the required version ($REQUIRED_VERSION). Attempting to install..."

    # Install noirup if not already installed
    if ! command_exists noirup; then
        echo "'noirup' is not installed. Installing 'noirup'..."
        curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash || { echo "Failed to install 'noirup'. Please install it manually."; exit 1; }
        echo "'noirup' installed successfully."
        
        # Update PATH
        export PATH="$HOME/.noirup/bin:$PATH"
        source /home/user/.bashrc  # Reload bashrc to ensure PATH is updated
    fi

    # Double-check that noirup is in the PATH
    if ! command_exists noirup; then
        echo "'noirup' is still not found after installation. Please check your PATH."
        exit 1
    fi
    
    # Install the required version of nargo
    echo "Installing 'nargo' version $REQUIRED_VERSION..."
    noirup --version "$REQUIRED_VERSION" || { echo "Failed to install 'nargo' version $REQUIRED_VERSION. Please install it manually."; exit 1; }
fi

# Install dependencies
echo "Installing dependencies..."
cd babygiant_native/ || { echo "Directory 'babygiant_native/' not found!"; exit 1; }
cargo build || { echo "Dependency installation failed!"; exit 1; }

# Build the project
echo "Building the project..."
cd ..
nargo build || { echo "Build failed!"; exit 1; }

# Run encryption tests with output
echo "Running only encryption..."
nargo test --show-output || { echo "Tests failed!"; exit 1; }

echo "Running both encryption and decryption with output..."
cd babygiant_native/ || { echo "Directory 'babygiant_native/' not found!"; exit 1; }
cargo run || { echo "Rust encryption/decryption failed!"; exit 1; }

echo "Script execution completed."
