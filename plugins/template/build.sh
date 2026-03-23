#!/bin/bash
# Build script for GenX Template JUCE Plugin
#
# Usage:
#   ./build.sh                  # Build Release and launch Standalone
#   ./build.sh --debug          # Build Debug and launch Standalone
#   ./build.sh --build-only     # Build without launching
#   ./build.sh --run-only       # Launch last build without rebuilding
#   ./build.sh --clean          # Clean build directory and rebuild

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"
ARTIFACT_DIR="$BUILD_DIR/GenXTemplate_artefacts"
APP_NAME="GenX Template.app"

# ── Parse arguments ──────────────────────────────────────────────────
BUILD_TYPE="Release"
DO_BUILD=true
DO_RUN=true
DO_CLEAN=false

for arg in "$@"; do
    case "$arg" in
        --debug)       BUILD_TYPE="Debug" ;;
        --build-only)  DO_RUN=false ;;
        --run-only)    DO_BUILD=false ;;
        --clean)       DO_CLEAN=true ;;
        *)             echo "Unknown argument: $arg"; exit 1 ;;
    esac
done

# ── Find CMake ───────────────────────────────────────────────────────
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
    echo "Install with:  brew install cmake"
    exit 1
fi

# ── Clean ────────────────────────────────────────────────────────────
if $DO_CLEAN; then
    echo "Cleaning build directory..."
    rm -rf "$BUILD_DIR"
fi

# ── Build ────────────────────────────────────────────────────────────
if $DO_BUILD; then
    echo "Building GenX Template ($BUILD_TYPE)..."
    echo "Using CMake: $CMAKE_CMD"

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

    # Configure
    "$CMAKE_CMD" "$SCRIPT_DIR" \
        -DCMAKE_BUILD_TYPE="$BUILD_TYPE" \
        -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

    # Build
    "$CMAKE_CMD" --build . --config "$BUILD_TYPE" -j"$CPU_COUNT"

    echo ""
    echo "Build complete!"
fi

# ── Locate Standalone app ────────────────────────────────────────────
STANDALONE=""
for candidate in \
    "$ARTIFACT_DIR/$BUILD_TYPE/Standalone/$APP_NAME" \
    "$ARTIFACT_DIR/Standalone/$APP_NAME" \
    "$ARTIFACT_DIR/$BUILD_TYPE/$APP_NAME" \
    "$ARTIFACT_DIR/$APP_NAME"; do
    if [ -d "$candidate" ]; then
        STANDALONE="$candidate"
        break
    fi
done

if [ -z "$STANDALONE" ]; then
    echo ""
    echo "Plugin artifacts:"
    find "$ARTIFACT_DIR" \( -name "*.vst3" -o -name "*.component" -o -name "*.app" \) 2>/dev/null | head -10
    if $DO_RUN; then
        echo ""
        echo "Warning: Standalone app not found. Build may have succeeded but app path differs."
        echo "Check: $ARTIFACT_DIR"
        exit 1
    fi
    exit 0
fi

# ── Launch ───────────────────────────────────────────────────────────
if $DO_RUN; then
    echo ""
    echo "Launching: $STANDALONE"
    open "$STANDALONE"
fi
