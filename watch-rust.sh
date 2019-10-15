##!/bin/bash


# 'reflex' program is required for this.
# reflex -r '^./src/' -s -- sh -c 'clear && cargo build'
reflex -g 'src/*.rs' -s -- sh -c 'clear && cargo build'





