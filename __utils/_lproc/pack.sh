#!/bin/bash

# Get the current directory name
DIR_NAME=$(basename "$PWD")

# Get current branch name for the archive name
BRANCH_NAME=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")

# Create timestamp for unique filename
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Archive filename
ARCHIVE_NAME="${DIR_NAME}_${BRANCH_NAME}_${TIMESTAMP}.tar.gz"

# Essential files for lproc package
ESSENTIAL_FILES="
lproc.py
hlproc.py
pyproject.toml
MANIFEST.in
README.md
CLAUDE.md
lproc.yaml
.gitignore
pack.sh
"

# Check if lptail symlink exists
if [ -L "lptail" ]; then
    ESSENTIAL_FILES="$ESSENTIAL_FILES lptail"
else
    echo "Warning: lptail symlink not found. You'll need to create it manually:"
    echo "  ln -s /usr/bin/tail lptail"
fi

# Filter to only include files that exist
FILES=""
for file in $ESSENTIAL_FILES; do
    if [ -e "$file" ]; then
        FILES="$FILES $file"
    fi
done

if [ -z "$FILES" ]; then
    echo "Error: No files found to pack"
    exit 1
fi

# Create tarball preserving symlinks
tar -hczf "$ARCHIVE_NAME" $FILES

if [ $? -eq 0 ]; then
    echo "Archive created: $ARCHIVE_NAME"
    echo "Files included:"
    for file in $FILES; do
        if [ -L "$file" ]; then
            echo "  - $file (symlink -> $(readlink "$file"))"
        else
            echo "  - $file"
        fi
    done
    echo ""
    echo "Size: $(du -h "$ARCHIVE_NAME" | cut -f1)"
    echo ""
    echo "To install on another computer:"
    echo "  1. Extract: tar -xzf $ARCHIVE_NAME"
    echo "  2. Verify lptail symlink exists (or create it: ln -s /usr/bin/tail lptail)"
    echo "  3. Install: pip install -e ."
    echo "  4. Commands 'lproc' and 'hlproc' will be available"
else
    echo "Error: Failed to create archive"
    exit 1
fi