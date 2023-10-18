#!/usr/bin/env bash

#******************************************************************************
#
# Convert single HTML page into single PDF file with default name.
#
#******************************************************************************

htop url https://www.engos.software >> /dev/null 2>&1
mv output.pdf actual.1