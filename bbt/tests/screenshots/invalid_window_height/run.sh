#!/usr/bin/env bash

#******************************************************************************
#
# The height in --window-size argument is invalid,
# because it should be an integer.
#
#******************************************************************************

htop -W --window-size=1200,1200cm url https://engos.de 2>&1
