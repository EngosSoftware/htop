#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of a single HTML page, save the image in JPEG format.
# The window size of the headless Chrome is set to custom value.
# Short options -J and -w are used.
#
#******************************************************************************

htop -Jw 1200,1200 url https://engos.de actual.1 >> /dev/null 2>&1
