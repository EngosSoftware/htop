#!/usr/bin/env bash

#******************************************************************************
#
# Convert single HTML file into single PDF file with custom paper size.
# Provided a proper value for --paper-size option.
#
#******************************************************************************

htop -b --paper-size=212mm,299mm single H_000010.html actual.1
