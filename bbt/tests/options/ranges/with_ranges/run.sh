#!/usr/bin/env bash

#******************************************************************************
#
# Printing ranges set to different values.
#
#******************************************************************************

htop -r '1-3' single ranges.html actual.1 2>&1
htop --ranges '1-2,4,6-9' single ranges.html actual.2 2>&1
