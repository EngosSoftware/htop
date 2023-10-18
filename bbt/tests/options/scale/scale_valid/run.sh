#!/usr/bin/env bash

#******************************************************************************
#
# Scale the content of the output PDF.
#
# Options:
#   -b : print background
#   --scale : use custom scale
#
#******************************************************************************

htop -b --scale=200% single H_000010.html actual.1 >> /dev/null 2>&1
htop -b --scale=2.0 single H_000010.html actual.2 >> /dev/null 2>&1