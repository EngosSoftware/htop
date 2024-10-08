#!/usr/bin/env bash

#******************************************************************************
#
# Run converter without generating the crash reports.
#
# Options:
#   -b : print background
#   -v : display messages during file conversion
#   --paper-format : set custom paper format
#   --no-crash-reports : disable Headless Chrome's crash reporter
#
#******************************************************************************

htop -bv --no-crash-reports --paper-format=A4 single H_000010.html actual.1 > out.txt 2>&1
tail -n 3 out.txt > lines.txt
cat lines.txt
rm -f lines.txt out.txt
