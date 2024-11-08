#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of a single HTML page, save the image in PNG format.
# The window size of the headless Chrome is set to custom value.
# Short options -P and -w are used.
#
#******************************************************************************

htop -Pw 1200,1200 url https://engos.de actual.1 >> /dev/null 2>&1
