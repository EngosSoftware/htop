#!/usr/bin/env bash

#******************************************************************************
#
# HTML page has infinite loop. Stop loading after specified timeout.
#
#******************************************************************************

htop -t 500 single index.html actual.1 2>&1