#!/bin/bash

# Function to detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        MINGW*)     echo "windows";;
        *)          echo "unknown";;
    esac
}

# Function to detect CPU architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64*)    echo "x86_64";;
        aarch64*)   echo "aarch64";;
        arm64*)     echo "aarch64";;
        *)          echo "unknown";;
    esac
}

# Set variables
OS=$(detect_os)
ARCH=$(detect_arch)
LIBTORCH_VERSION="2.1.0"
CUDA_VERSION="cu118"

# Determine download URL based on OS and architecture
case "$OS" in
    "linux")
        URL="https://download.pytorch.org/libtorch/${CUDA_VERSION}/libtorch-cxx11-abi-shared-with-deps-${LIBTORCH_VERSION}%2B${CUDA_VERSION}-${OS}-${ARCH}.zip"
        ;;
    "windows")
        URL="https://download.pytorch.org/libtorch/${CUDA_VERSION}/libtorch-win-shared-with-deps-${LIBTORCH_VERSION}%2B${CUDA_VERSION}.zip"
        ;;
    "macos")
        URL="https://download.pytorch.org/libtorch/cpu/libtorch-macos-${LIBTORCH_VERSION}.zip"
        ;;
    *)
        echo "Unsupported operating system"
        exit 1
        ;;
esac

echo "Downloading libtorch for $OS ($ARCH)..."
echo "URL: $URL"

# Create libs directory if it doesn't exist
mkdir -p libs

# Download libtorch
if [ "$OS" = "windows" ]; then
    powershell -Command "Invoke-WebRequest -Uri $URL -OutFile libs/libtorch.zip"
else
    curl -L "$URL" -o libs/libtorch.zip
fi

# Extract libtorch
echo "Extracting libtorch..."
if [ "$OS" = "windows" ]; then
    powershell -Command "Expand-Archive -Path libs/libtorch.zip -DestinationPath libs -Force"
else
    unzip -o libs/libtorch.zip -d libs/
fi

echo "Setting up environment variables..."
if [ "$OS" = "windows" ]; then
    echo "Please add the following to your environment variables:"
    echo "LIBTORCH_PATH=$(pwd)/libs/libtorch"
else
    echo "export LIBTORCH_PATH=$(pwd)/libs/libtorch" >> ~/.bashrc
    source ~/.bashrc
fi

echo "Installation complete!" 