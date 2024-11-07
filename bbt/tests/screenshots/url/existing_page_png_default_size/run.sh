#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of a single HTML page, save the image in PNG format.
# The windows size is the default in headless Chrome.
# The long option name --png is used.
#
#******************************************************************************

htop --png url https://engos.de actual.1 >> /dev/null 2>&1
