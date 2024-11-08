#!/usr/bin/env bash

#******************************************************************************
#
# The width in --window-size argument is invalid,
# because it should be an integer.
#
#******************************************************************************

htop -J --window-size=12cm,1200 url https://engos.de 2>&1
