#!/usr/bin/env bash

#******************************************************************************
#
# Convert multiple HTML files into multiple PDF files.
# Omit HTML files without *.html extension.
#
#******************************************************************************

htop multiple . 2>&1
mv H_000010.pdf actual.1
mv H_000011.pdf actual.2
find . -name '*.pdf' | wc -l