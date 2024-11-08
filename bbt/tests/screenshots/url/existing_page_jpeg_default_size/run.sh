#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of a single HTML page, save the image in JPEG format.
# The windows size is the default in headless Chrome.
# The long option name --jpeg is used.
#
#******************************************************************************

htop --jpeg url https://engos.de actual.1 >> /dev/null 2>&1
