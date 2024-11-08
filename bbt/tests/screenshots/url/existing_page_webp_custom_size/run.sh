#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of a single HTML page, save the image in WebP format.
# The window size of the headless Chrome is set to custom value.
# Short options -W and -w are used.
#
#******************************************************************************

htop -Ww 1200,1200 url https://engos.de actual.1 >> /dev/null 2>&1
