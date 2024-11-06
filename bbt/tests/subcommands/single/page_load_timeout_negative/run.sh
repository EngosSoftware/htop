#!/usr/bin/env bash

#******************************************************************************
#
# The timeout value is invalid.
# It should be a non-negativen number of milliseconds,
# but a negative value is provided.
#
#******************************************************************************

htop --timeout=-20 single index.html 2>&1
