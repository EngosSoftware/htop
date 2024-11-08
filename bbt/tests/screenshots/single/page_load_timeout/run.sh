#!/usr/bin/env bash

#******************************************************************************
#
# HTML page has infinite loop.
# Stops loading the page after specified timeout.
# No screenshot is taken.
#
#******************************************************************************

htop -J -t 500 single index.html 2>&1
