#!/usr/bin/env bash

#******************************************************************************
#
# Take a screenshot of non existing HTML page.
#
#******************************************************************************

htop -J url https://www.non-existing-web-page-123456789.com 2>&1
