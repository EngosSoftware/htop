#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of a single HTML page, save the image in WebP format.
# The windows size is the default in headless Chrome.
# The long option name --webp is used.
#
#******************************************************************************

htop --webp url https://engos.de actual.1 >> /dev/null 2>&1
