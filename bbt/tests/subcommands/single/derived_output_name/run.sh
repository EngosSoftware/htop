#!/usr/bin/env bash

#******************************************************************************
#
# Convert single HTML file into PDF file with output name
# derived from input name.
#
#******************************************************************************

htop -b single H_000010.html >> /dev/null 2>&1
mv H_000010.pdf actual.1