#!/usr/bin/env bash

#******************************************************************************
#
# Margins set to different values.
#
#******************************************************************************

htop -b --margin='1cm 15mm 200pt 300px' single margin.html actual.1 2>&1
htop -b -m '1cm 15mm 200pt 300px' single margin.html actual.2 2>&1
