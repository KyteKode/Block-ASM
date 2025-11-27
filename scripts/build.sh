#!/bin/bash

# Make sure to run this from inside the scripts folder.
# It expects the parent directory to be Block-ASM.

cd ..

# Check if "build" directory exists, creates it if it doesn't
if [ ! -d "$build"]; then
mkdir build
fi

# Compiles with g++
g++ -Iinclude -Ithird_party --std=c++17 -o build/main.out

chmod +x main.out