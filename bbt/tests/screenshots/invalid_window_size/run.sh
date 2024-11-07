#!/usr/bin/env bash

#******************************************************************************
#
# The --window-size argument is invalid,
# because two integers should be separated by a colon.
#
#******************************************************************************

htop --jpeg --window-size=1200-1200 url https://engos.de 2>&1
