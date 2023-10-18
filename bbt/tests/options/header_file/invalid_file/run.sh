#!/usr/bin/env bash

#******************************************************************************
#
# Invalid file name containing header HTML template.
#
#******************************************************************************

htop --header-file=non_existing.html single H_000010.html actual.1 2>&1