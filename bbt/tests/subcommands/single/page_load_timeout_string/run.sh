#!/usr/bin/env bash

#******************************************************************************
#
# The timeout value is invalid.
# It should be a number of milliseconds, but string is given.
#
#******************************************************************************

htop -t 20s single index.html 2>&1
