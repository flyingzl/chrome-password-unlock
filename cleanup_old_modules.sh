#!/bin/bash
# Script to remove old module directories after refactoring

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$SCRIPT_DIR/src"

echo "Removing old module directories..."

# Remove old module directories
for module in crypto database keychain models output profile; do
    if [ -d "$SRC_DIR/$module" ]; then
        echo "Removing $SRC_DIR/$module/mod.rs"
        rm "$SRC_DIR/$module/mod.rs"

        echo "Removing directory $SRC_DIR/$module"
        rmdir "$SRC_DIR/$module"

        echo "✓ Removed $module/"
    else
        echo "⊘ $module/ directory not found (already removed)"
    fi
done

echo ""
echo "Cleanup complete! Verifying new structure..."
ls -la "$SRC_DIR"

echo ""
echo "Running cargo check to verify..."
cd "$SCRIPT_DIR"
cargo check

echo ""
echo "✓ Refactoring completed successfully!"
