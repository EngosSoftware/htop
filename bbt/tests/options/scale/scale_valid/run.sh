#!/usr/bin/env bash

#******************************************************************************
#
# Scale the content of the output PDF.
#
# Options:
#   -b      : print background
#   --scale : use a custom scale
#
#******************************************************************************

(
  htop -b --scale=200% single H_000010.html actual.1;
  htop -b -s 2.0 single H_000010.html actual.2;
  htop -b -s 200% single H_000010.html actual.3
) >> /dev/null 2>&1
