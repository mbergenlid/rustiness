#!/bin/bash

if [ -x .cc65/bin/ld65 ] && [ -x .cc65/bin/ca65 ]
then
    echo "cc65 already installed"
else
    echo "Installing cc65 dependency"
    rm -rf .cc65
    git clone https://github.com/cc65/cc65.git .cc65
    cd .cc65
    git checkout 6de78c53
    make
    echo "Installed cc65 in .cc65/"
fi
