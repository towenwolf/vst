#!/bin/bash
# Build script for GenX Delay JUCE Plugin

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"

# Check for CMake
CMAKE_CMD=""
for path in cmake /opt/homebrew/bin/cmake /usr/local/bin/cmake /Applications/CMake.app/Contents/bin/cmake; do
    if command -v "$path" &>/dev/null; then
        CMAKE_CMD="$path"
        break
    fi
done

if [ -z "$CMAKE_CMD" ]; then
    echo "Error: CMake not found!"
    echo ""
    echo "Please install CMake using one of:"
    echo "  brew install cmake"
    echo "  or download from https://cmake.org/download/"
    exit 1
fi

echo "Using CMake: $CMAKE_CMD"

# Parse arguments
BUILD_TYPE="${1:-Release}"

echo "Building GenX Delay JUCE Plugin ($BUILD_TYPE)..."

# Create build directory
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Detect CPU count
if command -v sysctl &>/dev/null; then
    CPU_COUNT=$(sysctl -n hw.ncpu)
elif command -v nproc &>/dev/null; then
    CPU_COUNT=$(nproc)
else
    CPU_COUNT=4
fi

# Configure with CMake
"$CMAKE_CMD" "$SCRIPT_DIR" \
    -DCMAKE_BUILD_TYPE="$BUILD_TYPE" \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

# Build
"$CMAKE_CMD" --build . --config "$BUILD_TYPE" -j"$CPU_COUNT"

echo ""
echo "Build complete!"
echo ""

# Show output files
echo "Plugin locations:"
if [ -d "$BUILD_DIR/GenXDelayJ_artefacts" ]; then
    find "$BUILD_DIR/GenXDelayJ_artefacts" -name "*.vst3" -o -name "*.component" -o -name "*.app" 2>/dev/null | head -10
fi

echo ""
echo "The plugins have been copied to your system plugin folders."
echo "Rescan plugins in your DAW to use GenX Delay."
