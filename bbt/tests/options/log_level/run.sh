#!/usr/bin/env bash

#******************************************************************************
#
# Run converter with raised logging level
#
# Options:
#   --log-level : use custom logging level
#
#******************************************************************************

htop --log-level=info single H_000010.html actual.1 > out.txt 2>&1
head -n 10 out.txt > lines.txt
grep -c 'INFO' lines.txt
rm -f out.txt lines.txt
