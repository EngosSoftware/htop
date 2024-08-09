#!/usr/bin/env bash

#******************************************************************************
#
# Scale is too small.
#
# Options:
#   -b      : print background
#   --scale : used too small value for scale
#
#******************************************************************************

htop -b --scale=9% single H_000010.html actual.1 2>&1
htop -b --scale=0.09 single H_000010.html actual.1 2>&1
